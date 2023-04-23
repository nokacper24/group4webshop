use actix_multipart::Multipart;
use futures::StreamExt;
use image::{io::Reader as ImageReader, DynamicImage, ImageError, ImageFormat};
use std::{collections::HashMap, io::Cursor};

pub const IMAGES_DIR: &str = {
    match option_env!("RESOURCES_DIR") {
        Some(path) => path,
        None => "resources/images",
    }
};

const MAX_IMAGE_SIZE: usize = 1024 * 1024 * 5; // 5 MB
pub const ALLOWED_FORMATS: [ImageFormat; 3] =
    [ImageFormat::Png, ImageFormat::Jpeg, ImageFormat::WebP];

pub enum ImageExtractorError {
    Utf8Error(std::str::Utf8Error),
    MultipartError(actix_multipart::MultipartError),
    MissingContentDisposition(String),
    MissingData,
    UnexpectedField(String),
    FileTooLarge,
}

/// Contains image data and file name, as extracted from a multipart request.
/// # Fields
/// * `img_buffer` - Vector containing the image data as bytes.
/// * `file_name` - The file name of the image.
pub struct ExtractedImageData {
    pub img_buffer: Vec<u8>,
    pub file_name: String,
}

/// Extracts image and text fields from a multipart request.
/// Field containg the image is expected to be named `image`. **Do not** use this name for any text fields.
/// # Arguments
/// * `payload` - The multipart payload to extract the image component from.
/// * `text_fields` - A vector containing the names of the text fields wanted.
///
/// # Returns
/// A tuple containing the image buffer and the file name, and a vector containing the text fields.
/// * `image_buffer` - Vector containing the image data as bytes.
/// * `file_name` - The file name of the image.
/// * `text_fields` - HashMap containing the text fields wanted.
///
/// # Errors
/// Returns an `ImageExtractorError` containing the error that occurred.
pub async fn extract_image_and_texts_from_multipart(
    mut payload: Multipart,
    expected_fields: Vec<&str>,
) -> Result<(Option<ExtractedImageData>, HashMap<String, String>), ImageExtractorError> {
    let mut file_name = String::new();
    let mut image_buffer = Vec::new();
    // map of field name to field value, both strings
    let mut fields_found: HashMap<String, String> = HashMap::new();

    while let Some(mut item) = payload.next().await {
        let field = match item {
            Ok(ref mut field) => field,
            Err(e) => return Err(ImageExtractorError::MultipartError(e)),
        };
        let name = match field.content_disposition().get_name() {
            Some(name) => name,
            None => {
                return Err(ImageExtractorError::MissingContentDisposition(
                    "name".to_string(),
                ))
            }
        };

        if name == "image" {
            extract_image_from_field(field, &mut image_buffer, &mut file_name).await?;
        } else if expected_fields.iter().any(|field| field == &name) {
            let field_name = String::from(name);
            let mut field_content = String::new();
            extract_text_field(field, &mut field_content).await?;
            fields_found.insert(field_name, field_content);
        } else {
            return Err(ImageExtractorError::UnexpectedField(name.to_string()));
        }
    }

    let extracted_img = if file_name.trim().is_empty() || image_buffer.is_empty() {
        // no file_name or no data written to image_buffer - no image was found
        None
    } else {
        Some(ExtractedImageData {
            img_buffer: image_buffer,
            file_name,
        })
    };

    // check if all expected text fields were found
    if fields_found.len() != expected_fields.len() {
        return Err(ImageExtractorError::MissingData);
    }
    // check if any of the text fields are empty strings
    for field in fields_found.values() {
        if field.trim().is_empty() {
            return Err(ImageExtractorError::MissingData);
        }
    }

    Ok((extracted_img, fields_found))
}

/// Extracts a text field from a multipart field.
/// # Arguments
/// * `field` - The multipart field to extract the text from.
/// * `text_field` - Mut string ref to write found text to.
#[deprecated]
async fn extract_text_field(
    field: &mut actix_multipart::Field,
    text_field: &mut String,
) -> Result<(), ImageExtractorError> {
    while let Some(chunk) = field.next().await {
        let data = match chunk {
            Ok(data) => data,
            Err(e) => return Err(ImageExtractorError::MultipartError(e)),
        };
        let string = match std::str::from_utf8(&data) {
            Ok(s) => s,
            Err(e) => return Err(ImageExtractorError::Utf8Error(e)),
        };
        text_field.push_str(string);
    }
    Ok(())
}

/// Extracts an image from a multipart field.
/// # Arguments
/// * `field` - The multipart field to extract the image from.
/// * `image_buffer` - The buffer to write the image data to.
/// * `file_name` - Mut string ref to write the file name to.
#[deprecated]
async fn extract_image_from_field(
    field: &mut actix_multipart::Field,
    image_buffer: &mut Vec<u8>,
    file_name: &mut String,
) -> Result<(), ImageExtractorError> {
    file_name.push_str(match field.content_disposition().get_filename() {
        Some(name) => name,
        None => {
            return Err(ImageExtractorError::MissingContentDisposition(
                "filename".to_string(),
            ))
        }
    });

    while let Some(chunk) = field.next().await {
        let data = match chunk {
            Ok(data) => data,
            Err(e) => return Err(ImageExtractorError::MultipartError(e)),
        };
        if image_buffer.len() + data.len() > MAX_IMAGE_SIZE {
            return Err(ImageExtractorError::FileTooLarge);
        } else {
            image_buffer.extend_from_slice(&data);
        }
    }
    Ok(())
}

async fn extract_image_from_field_function(
    field: &mut actix_multipart::Field,
) -> Result<Option<(Vec<u8>, String)>, ImageExtractorError> {
    let mut file_name = String::new();
    let mut image_buffer = Vec::new();
    file_name.push_str(match field.content_disposition().get_filename() {
        Some(name) => name,
        None => return Ok(None), // no file name found, no image
    });
    while let Some(chunk) = field.next().await {
        let data = match chunk {
            Ok(data) => data,
            Err(e) => return Err(ImageExtractorError::MultipartError(e)),
        };
        if image_buffer.len() + data.len() > MAX_IMAGE_SIZE {
            return Err(ImageExtractorError::FileTooLarge);
        } else {
            image_buffer.extend_from_slice(&data);
        }
    }
    Ok(Some((image_buffer, file_name)))
}

async fn extract_text_field_function(
    field: &mut actix_multipart::Field,
) -> Result<Option<String>, ImageExtractorError> {
    let mut field_content = String::new();
    while let Some(chunk) = field.next().await {
        let data = match chunk {
            Ok(data) => data,
            Err(e) => return Err(ImageExtractorError::MultipartError(e)),
        };
        let string = match std::str::from_utf8(&data) {
            Ok(s) => s,
            Err(e) => return Err(ImageExtractorError::Utf8Error(e)),
        };
        field_content.push_str(string);
    }
    Ok(Some(field_content))
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
            if !ALLOWED_FORMATS.contains(&format) {
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
pub fn save_image(image: DynamicImage, dir: &str, file_name: &str) -> Result<String, ImageError> {
    std::fs::create_dir_all(dir)?;

    let file_name = sanitize_filename::sanitize(file_name).replace(' ', "-");

    let mut path = format!("{}/{}", dir, file_name);
    let mut i = 1;
    while std::path::Path::new(&path).exists() {
        path = format!("{}/{}-{}", dir, i, file_name);
        i += 1;
    }

    image.save(&path)?;

    if !path.starts_with('/') {
        path = format!("/{}", path);
    }

    Ok(path)
}

/// Removes a file from the file system.
pub fn remove_image(path: &str) -> Result<(), std::io::Error> {
    let real_path = {
        if !IMAGES_DIR.starts_with('/') {
            &path[1..]
        } else {
            path
        }
    };

    std::fs::remove_file(real_path)
}
