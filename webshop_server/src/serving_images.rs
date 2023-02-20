use actix_files::NamedFile;
use actix_multipart_extract::{File, MultipartForm};
use actix_multipart::Multipart;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use log::{error, info, warn};
use serde::Deserialize;
use std::fs;
use futures_util::StreamExt as _;
use image::io::Reader as ImageReader;
use image::ImageFormat;
use std::io::Cursor;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_image);
    cfg.service(post_image);
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

#[derive(Deserialize, MultipartForm, Debug)]
pub struct ImageUploadForm {
    #[multipart(max_size = 5MB)]
    file: File,
    product_id: String,
}

#[post("/")]
async fn post_image(payload: actix_multipart_extract::Multipart<ImageUploadForm>) -> impl Responder {
    // save file in resources/images/{product_id}/{filename}
    let folder_path = format!("resources/images/{}", payload.product_id);
    if let Err(e) = fs::create_dir_all(&folder_path) {
        error!("Error creating folder: {}", e);
        return HttpResponse::InternalServerError().finish();
    }
    let file_path = format!("{}/{}", folder_path, payload.file.name);
    match fs::write(&file_path, &payload.file.bytes) {
        Ok(_) => {
            let success = format!("File saved: {}", file_path);
            info!("{}", success);
            HttpResponse::Ok().body(success)
        }
        Err(e) => {
            error!("Error writing file: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
#[post("/upload/{product_id}")]
async fn upload_image(mut payload: Multipart, product_id: web::Path<String>,) -> impl Responder {
    
    //TODO check if product exists
    warn!("Product id: {:?}", product_id);

    let max_size = 5 * 1024 * 1024;
    let allowed_formats = vec![ImageFormat::Jpeg, ImageFormat::Png];

    let mut image_buffer = Vec::new();
    let mut file_name = String::new();
    // get payload in chunks and collect the image
    while let Some(mut item) = payload.next().await {
        let fild = match item{
            Ok(ref mut field) => field,
            Err(_) => return HttpResponse::InternalServerError().body("Error getting content disposition"),
        };
        let name = match fild.content_disposition().get_name() {
            Some(name) => name,
            None => return HttpResponse::BadRequest().body("No field name provided, expected 'image'"),
        };

        if let Some(filename) = fild.content_disposition().get_filename() {
            file_name = filename.to_string();
        } else {
            return HttpResponse::BadRequest().body("No file name provided");
        }

        if name == "image" {
            while let Some(chunk) = fild.next().await {
                let data = match chunk {
                    Ok(data) => data,
                    Err(_) => return HttpResponse::InternalServerError().body("Error getting chunk"),
                };
                if (image_buffer.len() + data.len()) > max_size {
                    return HttpResponse::PayloadTooLarge().body("File too large");
                }
                image_buffer.extend_from_slice(&data);
                warn!("Chunk: {:?}", data);
            }
        } else {
            return HttpResponse::BadRequest().body(format!("Wrong field name provided, expected 'image', got: '{}'", name));
        }
    }


    //parse the image with reader
    let image = match ImageReader::new(Cursor::new(&image_buffer)).with_guessed_format() {
        Ok(image) => image,
        Err(_) => return HttpResponse::BadRequest().body("Not a valid image"),
    };

    //check if the image format is allowed
    match image.format() {
        Some(format) => {
            if !allowed_formats.contains(&format) {
                return HttpResponse::BadRequest().body(format!("File format not allowed, allowed formats: {:?}", allowed_formats));
            }
            format
        }
        None => return HttpResponse::BadRequest().body("Not an image"),
    };
    


    // save file in resources/images/{product_id}/{filename}
    let folder_path = format!("resources/images/{}", product_id);
    if let Err(e) = fs::create_dir_all(&folder_path) {
        error!("Error creating folder: {}", e);
        return HttpResponse::InternalServerError().finish();
    }
    let file_path = format!("{}/{}", folder_path, file_name);
    match fs::write(&file_path, &image_buffer) {
        Ok(_) => {
            let success = format!("File saved: {}", file_path);
            info!("{}", success);
            HttpResponse::Ok().body(success)
        }
        Err(e) => {
            error!("Error writing file: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}