// public api routes

// Path: webshop_server\src\routes\public\public.rs

use actix_web::{get, web, Responder, HttpResponse};
use sqlx::{Pool, Postgres};
use crate::data_access::product::{get_products, get_product_by_id};

#[get("")]
async fn index() -> impl Responder {
    "Hello world!"
}

pub fn public(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
    cfg.service(products);
    cfg.service(product_by_name);
}

/// Get all products
#[get("products")]
async fn products(pool: web::Data<Pool<Postgres>>) -> impl Responder {
    let products = get_products(&pool).await;

    //error check
    if products.is_err() {
        return HttpResponse::InternalServerError().json("Internal Server Error");
    }
    
    //parse to json
    if let Ok(products) = products {
        return HttpResponse::Ok().json(products);
    }

    HttpResponse::InternalServerError().json("Internal Server Error")
}

/// Get a specific product by name
#[get("products/{product_name}")]
async fn product_by_name(pool: web::Data<Pool<Postgres>>, product_name: web::Path<String>) -> impl Responder {
    let product = get_product_by_id(&pool, product_name.as_str()).await;

    //error check
    if product.is_err() {
        return HttpResponse::InternalServerError().json("Internal Server Error");
    }
    
    //parse to json
    if let Ok(product) = product {
        return HttpResponse::Ok().json(product);
    }

    HttpResponse::InternalServerError().json("Internal Server Error")
}