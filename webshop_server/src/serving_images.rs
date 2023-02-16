use actix_files::NamedFile;
use actix_web::http::header::{ContentDisposition, DispositionType};
use actix_web::{get, App, Error, HttpRequest, Responder, HttpResponse};

#[get("/images/{filename:.*}")]
async fn get_image(
    req: HttpRequest,
) -> impl Responder {
    let path: std::path::PathBuf = req.match_info().query("filename").parse().unwrap();
    let path = std::path::Path::new("resources/images/").join(path);
    let file = NamedFile::open(path).unwrap();
    let response = file.into_response(&req);

    response
}