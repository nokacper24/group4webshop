use actix_multipart::Multipart;
use futures::StreamExt;
use image::{io::Reader as ImageReader, DynamicImage, ImageError, ImageFormat};
use std::io::Cursor;

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

pub fn parse_img(img_buffer: Vec<u8>) -> Result<DynamicImage, MyImageError> {
    let image = match ImageReader::new(Cursor::new(&img_buffer)).with_guessed_format() {
        Ok(image) => image,
        Err(e) => return Err(MyImageError::IoError(e)),
    };
    let image_format = image.format();

    let image = match image.decode() {
        Ok(image) => image,
        Err(e) => return Err(MyImageError::DecodeError(e)),
    };

    match image_format {
        Some(format) => {
            if !super::ALLOWED_FORMATS.contains(&format) {
                return Err(MyImageError::UnsuppoertedFormat(format));
            }
        }
        None => return Err(MyImageError::NoFormatFound),
    }
    Ok(image)
}
pub enum MyImageError {
    IoError(std::io::Error),
    DecodeError(ImageError),
    NoFormatFound,
    UnsuppoertedFormat(ImageFormat),
}
