use actix_web::{get, patch, post, put, web, HttpResponse, Responder};
use sqlx::{Pool, Postgres};

use crate::data_access::product::{self, PartialProduct, Product};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(products);
    cfg.service(product_by_id);
    cfg.service(create_product);
    cfg.service(update_product);
    cfg.service(get_product_description);
    cfg.service(description_swap_priorities);
}

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

#[put("products")]
pub async fn update_product(
    pool: web::Data<Pool<Postgres>>,
    product: web::Json<Product>,
) -> impl Responder {
    match product::update_product(&pool, &product).await {
        Ok(product) => HttpResponse::Created().json(product),
        Err(e) => match e {
            sqlx::Error::RowNotFound => HttpResponse::NotFound().json("Product not found"),
            _ => HttpResponse::InternalServerError().json("Internal Server Error"),
        },
    }
}

#[get("products/{product_id}/description")]
pub async fn get_product_description(
    pool: web::Data<Pool<Postgres>>,
    product_id: web::Path<String>,
) -> impl Responder {
    let descriptions =
        product::description::get_product_description_components(&pool, product_id.as_str()).await;

    match descriptions {
        Ok(descriptions) => HttpResponse::Ok().json(descriptions),
        Err(e) => match e {
            sqlx::Error::RowNotFound => HttpResponse::NotFound().json("Product not found"),
            _ => HttpResponse::InternalServerError().json("Internal Server Error"),
        },
    }
}

#[patch("products/{product_id}/description/priorityswap")]
pub async fn description_swap_priorities(
    pool: web::Data<Pool<Postgres>>,
    product_id: web::Path<String>,
    description_ids: web::Json<Vec<i32>>,
) -> impl Responder {
    let descriptions = product::description::swap_priority(
        &pool,
        product_id.as_str(),
        (description_ids[0], description_ids[1]),
    )
    .await;

    match descriptions {
        Ok(descriptions) => HttpResponse::Ok().json(descriptions),
        Err(e) => match e {
            sqlx::Error::RowNotFound => {
                HttpResponse::NotFound().json(format!("Product or description not found: {}", e))
            }
            _ => HttpResponse::InternalServerError().json(format!("Internal Server Error: {}", e)),
        },
    }
}
