use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use sqlx::{Pool, Postgres};

use crate::data_access::product::{get_product_by_id, get_products};

/// Get all products
#[get("products")]
pub async fn products(pool: web::Data<Pool<Postgres>>) -> impl Responder {
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
pub async fn product_by_name(
    pool: web::Data<Pool<Postgres>>,
    product_name: web::Path<String>,
) -> impl Responder {
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

#[post("products")]
pub async fn create_product(
    pool: web::Data<Pool<Postgres>>,
    product: web::Json<Product>,
) -> impl Responder {
    let product = create_new_product(&pool, &product).await;

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
    product: web::Json<FullProduct>,
) -> impl Responder {
    let product = update_product_by_id(&pool, product_id.as_str(), &product).await;

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

#[delete("products/{product_id}")]
pub async fn delete_product(
    pool: web::Data<Pool<Postgres>>,
    product_id: web::Path<String>,
) -> impl Responder {
    let product = delete_product_by_id(&pool, product_id.as_str()).await;

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
