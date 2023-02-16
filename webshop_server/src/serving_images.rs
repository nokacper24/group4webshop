use actix_files::NamedFile;
use actix_multipart_extract::{File, Multipart, MultipartForm};
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use log::{error, info};
use serde::Deserialize;
use std::fs;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_image);
    cfg.service(post_image);
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

#[post("")]
async fn post_image(payload: Multipart<ImageUploadForm>) -> impl Responder {
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
