// public api routes

// Path: webshop_server\src\routes\public\public.rs

use actix_web::{get, web, Responder};

pub mod categories;
pub mod licenses;
pub mod products;
pub mod users;
pub mod company;

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
    cfg.configure(company::configure);
}
