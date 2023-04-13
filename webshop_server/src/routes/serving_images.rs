use crate::IMAGES_DIR;
use actix_files::NamedFile;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_image);
}

#[get("/{filename:.*}")]
async fn get_image(req: HttpRequest) -> impl Responder {
    let path: std::path::PathBuf = match req.match_info().query("filename").parse() {
        Ok(path) => path,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    let images_path = format!("{}/", IMAGES_DIR);
    let full_path = std::path::Path::new(&images_path).join(path);
    match NamedFile::open(full_path.clone()) {
        Ok(file) => file.into_response(&req),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}
