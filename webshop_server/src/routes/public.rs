// public api routes

// Path: webshop_server\src\routes\public\public.rs

use actix_web::{get, web, Responder};

pub mod auth;
pub mod categories;
pub mod products;
pub mod testimonials;

#[get("")]
async fn index() -> impl Responder {
    "Hello world!"
}

pub fn public(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
    cfg.configure(categories::configure);
    cfg.configure(products::configure);
    cfg.configure(auth::configure);
    cfg.configure(testimonials::configure);
}
