// public api routes

// Path: webshop_server\src\routes\public\public.rs

use actix_web::{get, web, Responder};

pub mod auth;
pub mod categories;
pub mod companies;
pub mod licenses;
pub mod products;
pub mod testimonials;
pub mod users;

#[get("")]
async fn index() -> impl Responder {
    "Hello world!"
}

pub fn public(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
    cfg.configure(categories::configure);
    cfg.configure(licenses::configure);
    cfg.configure(products::configure);
    cfg.configure(users::configure);
    cfg.configure(auth::configure);
    cfg.configure(companies::configure);
    cfg.configure(testimonials::configure);
    cfg.default_service(web::route().to(super::api_not_found));
}
