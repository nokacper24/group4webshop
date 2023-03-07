use crate::data_access::product;
use actix_web::{get, web, HttpResponse, Responder};

use log::error;
use sqlx::{Pool, Postgres};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_product_descriptions);
    cfg.service(get_product_description_component_by_id);
}

#[get("/{product_id}/descriptions")]
pub async fn get_product_descriptions(
    pool: web::Data<Pool<Postgres>>,
    product_id: web::Path<String>,
) -> impl Responder {
    let descriptions =
        product::description::get_product_description_components(&pool, product_id.as_str()).await;

    let descriptions = match descriptions {
        Ok(descriptions) => descriptions,
        Err(e) => {
            error!("Error while getting product descriptions: {}", e);
            return HttpResponse::InternalServerError().json("Internal Server Error");
        }
    };

    if descriptions.is_empty() {
        let is_valid = match product::product_exists(&pool, product_id.as_str()).await {
            Ok(is_valid) => is_valid,
            Err(e) => {
                error!("Error while checking if product exists: {}", e);
                return HttpResponse::InternalServerError().json("Internal Server Error");
            }
        };
        if !is_valid {
            return HttpResponse::NotFound().json("Product not found");
        }
    }

    HttpResponse::Ok().json(descriptions)
}

#[get("/{product_id}/descriptions/{component_id}")]
pub async fn get_product_description_component_by_id(
    pool: web::Data<Pool<Postgres>>,
    path: web::Path<(String, i32)>,
) -> impl Responder {
    let (product_id, component_id) = path.into_inner();
    let description = product::description::get_description_component_checked(
        &pool,
        product_id.as_str(),
        component_id,
    )
    .await;

    match description {
        Ok(description) => HttpResponse::Ok().json(description),
        Err(e) => match e {
            sqlx::Error::RowNotFound => {
                HttpResponse::NotFound().json("Product or description not found")
            }
            _ => HttpResponse::InternalServerError().json("Internal Server Error"),
        },
    }
}
