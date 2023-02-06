use actix_web::{get, HttpResponse, web, Responder, post};
use sqlx::{Postgres, Pool};

use crate::data_access::product::{get_products, get_product_by_id};


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

#[post("products")]
async fn create_product(pool: web::Data<Pool<Postgres>>, product: web::Json<Product>) -> impl Responder {
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