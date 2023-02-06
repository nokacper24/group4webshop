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
    cfg.service(products::products);
    cfg.service(products::product_by_name);
    cfg.service(products::create_product);
    cfg.service(products::update_product);
    cfg.service(products::delete_product);
}
