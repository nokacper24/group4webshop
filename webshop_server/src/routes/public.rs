// public api routes

// Path: webshop_server\src\routes\public\public.rs

use actix_web::{get, web, Responder};


use crate::data_access::product::Product;

pub mod categories;
pub mod products;
pub mod users;

#[get("")]
async fn index() -> impl Responder {
    "Hello world!"
}



pub fn public(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
    cfg.configure(products::configure);
    cfg.configure(categories::configure);
    cfg.configure(users::configure);
}
