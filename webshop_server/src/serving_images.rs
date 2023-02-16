use actix_files::NamedFile;
use actix_web::{get, HttpRequest, HttpResponse, Responder};

#[get("/resources/images/{filename:.*}")]
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
