// public api routes

// Path: webshop_server\src\routes\public\public.rs

use crate::data_access::category::{get_categories, get_category_by_id};
use crate::data_access::product::{get_product_by_id, get_products};
use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::{Pool, Postgres};

mod categories;
mod products;

#[get("")]
async fn index() -> impl Responder {
    "Hello world!"
}

pub fn public(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
    cfg.service(products::products);
}
