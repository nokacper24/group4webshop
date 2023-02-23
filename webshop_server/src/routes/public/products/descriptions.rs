use actix_web::{web, Responder, HttpResponse, get, patch};
use sqlx::{Postgres, Pool};

use crate::data_access::product;


pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_product_descriptions);
    cfg.service(description_swap_priorities);
}

#[get("/{product_id}/descriptions")]
pub async fn get_product_descriptions(
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

#[patch("/{product_id}/descriptions/priorityswap")]
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