use actix_multipart::Multipart;
use actix_web::{get, patch, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use utoipa::ToSchema;

use crate::data_access::product::{self, description::DescriptionCompError};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_product_descriptions);
    cfg.service(description_swap_priorities);
    cfg.service(add_text_description);
    cfg.service(update_priority);
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

#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct RequestNewPriority {
    component_id: i32,
    new_priority: i32,
}
#[patch("/{product_id}/descriptions/priority")]
async fn update_priority(
    pool: web::Data<Pool<Postgres>>,
    product_id: web::Path<String>,
    req_body: web::Json<RequestNewPriority>,
) -> impl Responder {
    let query_result = product::description::update_priority(
        &pool,
        product_id.as_str(),
        req_body.component_id,
        req_body.new_priority,
    )
    .await;

    match query_result {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => match e {
            sqlx::Error::RowNotFound => {
                HttpResponse::NotFound().json("Product or description not found")
            }
            _ => HttpResponse::Conflict().json(format!("Internal Server Error: {}", e)), //TODO
        },
    }
}

#[patch("/{product_id}/descriptions/priorityswap")]
async fn description_swap_priorities(
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

#[post("/{product_id}/descriptions/text")]
async fn add_text_description(
    pool: web::Data<Pool<Postgres>>,
    product_id: web::Path<String>,
    description: web::Json<product::description::TextComponent>,
) -> impl Responder {
    let created_component = product::description::create_text_component(
        &pool,
        product_id.as_str(),
        description.into_inner(),
    )
    .await;
    match created_component {
        Ok(created_component) => HttpResponse::Ok().json(created_component),
        Err(e) => match e {
            DescriptionCompError::InvalidComponent(e) => {
                HttpResponse::BadRequest().json(format!("Invalid component: {}", e))
            }
            DescriptionCompError::SqlxError(e) => {
                HttpResponse::InternalServerError().json(format!("Internal Server Error: {}", e))
            }
        },
    }
}

struct ImageComponentDetails {}

#[post("/{product_id}/descriptions/image")]
async fn upload_image(
    payload: Multipart,
    product_id: web::Path<String>,
    pool: web::Data<Pool<Postgres>>,
) -> impl Responder {
    match product::product_exists(&pool, product_id.as_str()).await {
        Ok(exists) => {
            if !exists {
                return HttpResponse::NotFound().json("Product not found");
            }
        }
        Err(_) => {
            return HttpResponse::InternalServerError().json("Internal Server Error");
        }
    };

    let (image_component_details, file_name, image_buffer) =
        match image_comp_from_multipart(payload) {
            Ok((form_details, image_buffer, file_name)) => (form_details, image_buffer, file_name),
            Err(_) => todo!("Handle error"),
        };

    HttpResponse::Ok().json("Image uploaded")
}

fn image_comp_from_multipart(
    payload: Multipart,
) -> Result<(ImageComponentDetails, Vec<u8>, String), ()> {
    todo!()
}
