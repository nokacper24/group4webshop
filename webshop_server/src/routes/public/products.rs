use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use sqlx::{Pool, Postgres};

use crate::data_access::product::{self, Product, PartialProduct};

/// Get all products
#[get("products")]
pub async fn products(pool: web::Data<Pool<Postgres>>) -> impl Responder {
    let products = product::get_products(&pool).await;

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
#[get("products/{product_id}")]
pub async fn product_by_id(
    pool: web::Data<Pool<Postgres>>,
    product_id: web::Path<String>,
) -> impl Responder {
    let product = product::get_product_by_id(&pool, product_id.as_str()).await;

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

#[post("products")]
pub async fn create_product(
    pool: web::Data<Pool<Postgres>>,
    product: web::Json<PartialProduct>,
) -> impl Responder {
    let product = product::create_product(&pool, &product).await;

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

#[put("products/{product_id}")]
pub async fn update_product(
    pool: web::Data<Pool<Postgres>>,
    product_id: web::Path<String>,
    product: web::Json<Product>,
) -> impl Responder {
    let product = product::update_product(&pool, product_id.as_str(), &product).await;

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