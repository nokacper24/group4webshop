// public api routes

// Path: webshop_server\src\routes\public\public.rs

use actix_web::{get, web, Responder};

mod products;

#[get("")]
async fn index() -> impl Responder {
    "Hello world!"
}

pub fn public(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
    cfg.configure(products::configure);
}
