use actix_multipart::Multipart;
use actix_web::{get, patch, post, web, HttpResponse, Responder};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use utoipa::ToSchema;

use crate::data_access::{product::{self, description::DescriptionCompError}, error_handling};

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


#[patch("/{product_id}/descriptions/{component_id}/priority")]
async fn update_priority(
    pool: web::Data<Pool<Postgres>>,
    path: web::Path<(String, i32)>,
    new_priority: web::Json<i32>,
) -> impl Responder {
    let (product_id, component_id) = path.into_inner();
    let query_result = product::description::update_priority(
        &pool,
        product_id.as_str(),
        component_id,
        new_priority.into_inner(),
    )
    .await;

    match query_result {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => match e {
            sqlx::Error::RowNotFound => {
                HttpResponse::NotFound().json("Product or description not found")
            },
            sqlx::Error::Database(e) => {
                match error_handling::PostgresDBError::from_str(e) {
                    error_handling::PostgresDBError::UniqueViolation => HttpResponse::Conflict().json("Conflict: Priority already in use"),
                    _ => HttpResponse::InternalServerError().json("Internal Server Error"),
                }
            },
            _ => HttpResponse::InternalServerError().json("Internal Server Error"),
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

    let (alt_text, file_name, image_buffer) = match extract_image_component_multipart(payload).await
    {
        Ok((alt_text, image_buffer, file_name)) => (alt_text, image_buffer, file_name),
        Err(_) => todo!("Handle error"),
    };

    HttpResponse::Ok().json("Image uploaded")
}

async fn extract_image_component_multipart(
    mut payload: Multipart,
) -> Result<(String, Vec<u8>, String), ImageExtractorError> {
    todo!("Implement");
    
    let mut alt_text = String::new();
    let mut file_name = String::new();
    let mut image_buffer = Vec::new();

    while let Some(mut item) = payload.next().await {
        let field = match item {
            Ok(ref mut field) => field,
            Err(e) => return Err(ImageExtractorError::MultipartError(e)),
        };
        let name = match field.content_disposition().get_name() {
            Some(name) => name,
            None => return Err(ImageExtractorError::MissingField("name".to_string())),
        };
        if name == "alt_text" {
            while let Some(chunk) = field.next().await {
                let data = match chunk {
                    Ok(data) => data,
                    Err(e) => return Err(ImageExtractorError::MultipartError(e)),
                };
                let string = match std::str::from_utf8(&data) {
                    Ok(s) => s,
                    Err(e) => return Err(ImageExtractorError::Utf8Error(e)),
                };
                alt_text.push_str(string);
            }
        }
    }

    Ok((alt_text, image_buffer, file_name))
}

enum ImageExtractorError {
    Utf8Error(std::str::Utf8Error),
    IoError(std::io::Error),
    MultipartError(actix_multipart::MultipartError),
    MissingField(String),
}
