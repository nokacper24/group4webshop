use actix_multipart::Multipart;
use futures::StreamExt;
use image::{io::Reader as ImageReader, DynamicImage, ImageError, ImageFormat, ImageResult};
use std::io::Cursor;

/// Extracts the image component from a multipart request.
///
/// # Arguments
///
/// * `payload` - The multipart payload to extract the image component from.
///
/// # Returns
///
/// A tuple containing the alt text, the image buffer and the file name.
/// * `alt_text` - The alt text of the image.
/// * `image_buffer` - Vector containing the image data as bytes.
/// * `file_name` - The file name of the image.
///
/// # Errors
///
/// Returns an `ImageExtractorError` containing the error that occurred.
pub async fn extract_image_component_from_multipart(
    mut payload: Multipart,
) -> Result<(String, Vec<u8>, String), ImageExtractorError> {
    let mut alt_text = String::new();
    let mut file_name = String::new();
    let mut image_buffer = Vec::new();

    while let Some(mut item) = payload.next().await {
        let field = match item {
            Ok(ref mut field) => field,
            Err(e) => return Err(ImageExtractorError::MultipartError(e)),
        };
        let name = match field.content_disposition().get_name() {
            Some(name) => name,
            None => return Err(ImageExtractorError::MissingField("name".to_string())),
        };
        if name == "alt_text" {
            while let Some(chunk) = field.next().await {
                let data = match chunk {
                    Ok(data) => data,
                    Err(e) => return Err(ImageExtractorError::MultipartError(e)),
                };
                let string = match std::str::from_utf8(&data) {
                    Ok(s) => s,
                    Err(e) => return Err(ImageExtractorError::Utf8Error(e)),
                };
                alt_text.push_str(string);
            }
        } else if name == "image" {
            file_name = match field.content_disposition().get_filename() {
                Some(name) => name.to_string(),
                None => return Err(ImageExtractorError::MissingField("filename".to_string())),
            };

            while let Some(chunk) = field.next().await {
                let data = match chunk {
                    Ok(data) => data,
                    Err(e) => return Err(ImageExtractorError::MultipartError(e)),
                };
                if image_buffer.len() + data.len() > super::MAX_IMAGE_SIZE {
                    return Err(ImageExtractorError::FileTooLarge);
                } else {
                    image_buffer.extend_from_slice(&data);
                }
            }
        } else {
            return Err(ImageExtractorError::UnexpectedField(name.to_string()));
        }
    }
    if alt_text.trim().is_empty() || file_name.trim().is_empty() || image_buffer.is_empty() {
        return Err(ImageExtractorError::MissingData);
    }

    Ok((alt_text, image_buffer, file_name))
}

pub enum ImageExtractorError {
    Utf8Error(std::str::Utf8Error),
    MultipartError(actix_multipart::MultipartError),
    MissingField(String),
    MissingData,
    UnexpectedField(String),
    FileTooLarge,
}

/// Parses an image buffer into a `DynamicImage`.
///
/// # Arguments
///
/// * `img_buffer` - The image buffer to parse.
///
/// # Returns
///
/// A `DynamicImage` if the image could be parsed successfully.
///
/// # Errors
///
/// Returns a `ImageParsingError` if the image could not be parsed.
pub fn parse_img(img_buffer: Vec<u8>) -> Result<DynamicImage, ImageParsingError> {
    let image = match ImageReader::new(Cursor::new(&img_buffer)).with_guessed_format() {
        Ok(image) => image,
        Err(e) => return Err(ImageParsingError::IoError(e)),
    };
    let image_format = image.format();

    let image = match image.decode() {
        Ok(image) => image,
        Err(e) => return Err(ImageParsingError::DecodeError(e)),
    };

    match image_format {
        Some(format) => {
            if !super::ALLOWED_FORMATS.contains(&format) {
                return Err(ImageParsingError::UnsuppoertedFormat(format));
            }
        }
        None => return Err(ImageParsingError::NoFormatFound),
    }
    Ok(image)
}
pub enum ImageParsingError {
    IoError(std::io::Error),
    DecodeError(ImageError),
    NoFormatFound,
    UnsuppoertedFormat(ImageFormat),
}

/// Saves an image to the file system.
/// Does not overwrite existing files, but appends a number to the file name.
///
/// # Arguments
///
/// * `image` - The image to save.
/// * `dir` - The directory to save the image to.
/// * `file_name` - The file name of the image.
///
/// # Returns
///
/// The path to the saved image.
pub fn save_image(
    image: DynamicImage,
    dir: &str,
    file_name: &str,
) -> Result<String, std::io::Error> {
    std::fs::create_dir_all(dir)?;

    let file_name = sanitize_filename::sanitize(file_name).replace(" ", "-");

    let mut path = format!("{}/{}", dir, file_name);
    let mut i = 1;
    while std::path::Path::new(&path).exists() {
        path = format!("{}/{}-{}", dir, i, file_name);
        i += 1;
    }

    if image.save(&path).is_err() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Could not save image",
        ));
    }

    Ok(path)
}

/// Removes a file from the file system.
pub fn remove_image(path: &str) -> Result<(), std::io::Error> {
    std::fs::remove_file(path)
}
