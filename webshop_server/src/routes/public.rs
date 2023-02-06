// public api routes

// Path: webshop_server\src\routes\public\public.rs

use actix_web::{get, web, Responder, HttpResponse, post};
use sqlx::{Pool, Postgres};
use crate::data_access::product::{get_products, get_product_by_id};

mod products;

#[get("")]
async fn index() -> impl Responder {
    "Hello world!"
}

pub fn public(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
    cfg.service(products::products);
}
