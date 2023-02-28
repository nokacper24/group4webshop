use actix_web::{get, web, Responder};


pub mod auth;

#[get("")]
async fn index() -> impl Responder {
    "Hello world!"
}

pub fn private(cfg: &mut web::ServiceConfig) {
    cfg.configure(auth::configure);
}
