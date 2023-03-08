use actix_web::{get, web, Responder};

pub mod auth;
pub mod licenses_protected;
pub mod products_protected;

#[get("")]
async fn index() -> impl Responder {
    "Hello world!"
}

pub fn private(cfg: &mut web::ServiceConfig) {
    cfg.configure(auth::configure);
    cfg.configure(products_protected::configure);
    cfg.configure(licenses_protected::configure);
}
