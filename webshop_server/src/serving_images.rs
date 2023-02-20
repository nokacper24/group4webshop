use actix_files::NamedFile;
use actix_multipart::Multipart;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use futures_util::StreamExt;
use image::{io::Reader as ImageReader, ImageFormat};
use log::{error, info};
use serde_json::json;
use sqlx::{Pool, Postgres};
use std::{fs, io::Cursor};

use crate::data_access::product;

const MAX_SIZE: usize = 1024 * 1024 * 5; // 5 MB
const ALLOWED_FORMATS: [ImageFormat; 2] = [ImageFormat::Png, ImageFormat::Jpeg];

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_image);
    cfg.service(upload_image);
}

#[get("/{filename:.*}")]
async fn get_image(req: HttpRequest) -> impl Responder {
    let path: std::path::PathBuf = match req.match_info().query("filename").parse() {
        Ok(path) => path,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    let path = std::path::Path::new("resources/images/").join(path);
    match NamedFile::open(path) {
        Ok(file) => file.into_response(&req),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[post("/{product_id}")]
async fn upload_image(
    payload: Multipart,
    product_id: web::Path<String>,
    pool: web::Data<Pool<Postgres>>,
) -> impl Responder {
    match product::product_exists(&pool, product_id.as_str()).await {
        Ok(exists) => {
            if !exists {
                return HttpResponse::NotFound().json("Product not found");
            }
        }
        Err(_) => {
            return HttpResponse::InternalServerError().json("Internal Server Error");
        }
    };

    let (image_buffer, file_name) = match get_file_from_multipart(payload).await {
        Ok((image_buffer, file_name)) => (image_buffer, file_name),
        Err(error_response) => return error_response,
    };

    if let Err(error_response) = is_image_valid(&image_buffer) {
        return error_response;
    }

    // save file in resources/images/{product_id}/{filename}
    let folder_path = format!("resources/images/{}", product_id);
    if let Err(e) = fs::create_dir_all(&folder_path) {
        error!("Error creating folder: {}", e);
        return HttpResponse::InternalServerError().finish();
    }
    let file_path = format!("{}/{}", folder_path, file_name);
    match fs::write(&file_path, &image_buffer) {
        Ok(_) => {
            let success =
                json!({ "path": format!("/resources/images/{}/{}", product_id, file_name) });
            info!("File saved: {}", file_path);
            HttpResponse::Created().json(success)
        }
        Err(e) => {
            error!("Error writing file: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Takes multipart request and returns buffered file and file name
async fn get_file_from_multipart(mut payload: Multipart) -> Result<(Vec<u8>, String), HttpResponse> {
    let mut image_buffer = Vec::new();
    let mut file_name = String::new();
    // get payload in chunks and collect the image
    while let Some(mut item) = payload.next().await {
        let fild = match item {
            Ok(ref mut field) => field,
            Err(_) => {
                return Err(
                    HttpResponse::InternalServerError().json("Error getting content disposition")
                );
            }
        };
        let name = match fild.content_disposition().get_name() {
            Some(name) => name,
            None => {
                return Err(
                    HttpResponse::BadRequest().json("No field name provided, expected 'image'")
                );
            }
        };
        if let Some(filename) = fild.content_disposition().get_filename() {
            file_name = filename.to_string();
        } else {
            return Err(HttpResponse::BadRequest().json("No file name provided"));
        }
        if name == "image" {
            while let Some(chunk) = fild.next().await {
                let data = match chunk {
                    Ok(data) => data,
                    Err(_) => {
                        return Err(HttpResponse::InternalServerError().json("Error getting chunk"));
                    }
                };
                if (image_buffer.len() + data.len()) > MAX_SIZE {
                    return Err(HttpResponse::PayloadTooLarge().json("File too large"));
                }
                image_buffer.extend_from_slice(&data);
            }
        } else {
            return Err(HttpResponse::BadRequest().json(format!(
                "Wrong field name provided, expected 'image', got: '{}'",
                name
            )));
        }
    }
    Ok((image_buffer, file_name))
}


fn is_image_valid(
    image_buffer: &Vec<u8>,
) -> Result<(), HttpResponse>
{
    let image = match ImageReader::new(Cursor::new(&image_buffer)).with_guessed_format() {
        Ok(image) => image,
        Err(_) => return Err(HttpResponse::UnprocessableEntity().json("Not a valid image")),
    };
    match image.format() {
        Some(format) => {
            if !ALLOWED_FORMATS.contains(&format) {
                return Err(HttpResponse::UnsupportedMediaType().json(format!(
                    "File format not allowed, allowed formats: {:?}",
                    ALLOWED_FORMATS
                )));
            }
            format
        }
        None => return Err(HttpResponse::UnprocessableEntity().json("Not an image")),
    };
    Ok(())
}