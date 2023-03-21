use crate::{
    data_access::{
        error_handling,
        product::{self, description::DescriptionCompError},
        user,
    },
    utils::auth,
    routes::private::products_protected::descriptions_protected::description_utils::{
        ImageExtractorError, ImageParsingError,
    },
};
use actix_multipart::Multipart;
use actix_web::{
    delete, patch, post,
    web::{self, ReqData},
    HttpRequest, HttpResponse, Responder,
};
use image::ImageFormat;
use log::{error, warn};
use sqlx::{Pool, Postgres};

pub mod description_utils;

const MAX_IMAGE_SIZE: usize = 1024 * 1024 * 5; // 5 MB
pub const ALLOWED_FORMATS: [ImageFormat; 3] =
    [ImageFormat::Png, ImageFormat::Jpeg, ImageFormat::WebP];
pub const IMAGE_DIR: &str = "resources/images";

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(delete_description_component);
    cfg.service(description_swap_priorities);
    cfg.service(add_text_description);
    cfg.service(update_priority);
    cfg.service(upload_image);
}

#[delete("/{product_id}/descriptions/{component_id}")]
async fn delete_description_component(
    pool: web::Data<Pool<Postgres>>,
    path: web::Path<(String, i32)>,
    req: HttpRequest,
) -> impl Responder {
    match auth::validate_user(req, &pool).await {
        Ok(user) => {
            if user.role != user::Role::Admin {
                return HttpResponse::Forbidden().finish();
            }
        }
        Err(e) => {
            return match e {
                auth::AuthError::Unauthorized => HttpResponse::Unauthorized().finish(),
                auth::AuthError::SqlxError(e) => {
                    error!("{}", e);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
    };

    let (product_id, component_id) = path.into_inner();
    let query_result =
        product::description::delete_component(&pool, product_id.as_str(), component_id).await;
    let img_path: Option<String> = match query_result {
        Ok(opt_path) => opt_path,
        Err(e) => match e {
            sqlx::Error::RowNotFound => {
                return HttpResponse::NotFound().json("Product or description not found")
            }
            _ => {
                error!("Error while deleting description component: {}", e);
                return HttpResponse::InternalServerError().json("Internal Server Error");
            }
        },
    };
    if let Some(path) = img_path {
        if let Err(e) = description_utils::remove_image(&path) {
            error!("Error while deleting image: {}.", e);
            warn!("Image file may be left in the file system.");
            return HttpResponse::InternalServerError().json("Internal Server Error");
        }
    }
    HttpResponse::NoContent().finish()
}

#[patch("/{product_id}/descriptions/{component_id}/priority")]
async fn update_priority(
    pool: web::Data<Pool<Postgres>>,
    path: web::Path<(String, i32)>,
    new_priority: web::Json<i32>,
    req: HttpRequest,
) -> impl Responder {
    match auth::validate_user(req, &pool).await {
        Ok(user) => {
            if user.role != user::Role::Admin {
                return HttpResponse::Forbidden().finish();
            }
        }
        Err(e) => {
            return match e {
                auth::AuthError::Unauthorized => HttpResponse::Unauthorized().finish(),
                auth::AuthError::SqlxError(e) => {
                    error!("{}", e);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
    };

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
            }
            sqlx::Error::Database(e) => match error_handling::PostgresDBError::from_str(e) {
                error_handling::PostgresDBError::UniqueViolation => {
                    HttpResponse::Conflict().json("Conflict: Priority already in use")
                }
                _ => HttpResponse::InternalServerError().json("Internal Server Error"),
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
    req: HttpRequest,
) -> impl Responder {
    match auth::validate_user(req, &pool).await {
        Ok(user) => {
            if user.role != user::Role::Admin {
                return HttpResponse::Forbidden().finish();
            }
        }
        Err(e) => {
            return match e {
                auth::AuthError::Unauthorized => HttpResponse::Unauthorized().finish(),
                auth::AuthError::SqlxError(e) => {
                    error!("{}", e);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
    };

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
    req: HttpRequest,
) -> impl Responder {
    match auth::validate_user(req, &pool).await {
        Ok(user) => {
            if user.role != user::Role::Admin {
                return HttpResponse::Forbidden().finish();
            }
        }
        Err(e) => {
            return match e {
                auth::AuthError::Unauthorized => HttpResponse::Unauthorized().finish(),
                auth::AuthError::SqlxError(e) => {
                    error!("{}", e);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
    };

    match product::product_exists(&pool, product_id.as_str()).await {
        Ok(exists) => {
            if !exists {
                return HttpResponse::NotFound().json("Product not found");
            }
        }
        Err(_) => return HttpResponse::InternalServerError().json("Internal Server Error"),
    }

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
    req: HttpRequest,
) -> impl Responder {
    match auth::validate_user(req, &pool).await {
        Ok(user) => {
            if user.role != user::Role::Admin {
                return HttpResponse::Forbidden().finish();
            }
        }
        Err(e) => {
            return match e {
                auth::AuthError::Unauthorized => HttpResponse::Unauthorized().finish(),
                auth::AuthError::SqlxError(e) => {
                    error!("{}", e);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
    };

    match product::product_exists(&pool, product_id.as_str()).await {
        Ok(exists) => {
            if !exists {
                return HttpResponse::NotFound().json("Product not found");
            }
        }
        Err(_) => return HttpResponse::InternalServerError().json("Internal Server Error"),
    };

    let (image_buffer, file_name, mut text_fields) =
        match description_utils::extract_image_and_texts_from_multipart(payload, vec!["alt_text"])
            .await
        {
            Ok((image_buffer, file_name, text_fields)) => (image_buffer, file_name, text_fields),
            Err(e) => {
                return match e {
                    ImageExtractorError::MultipartError(e) => HttpResponse::InternalServerError()
                        .json(format!("Couldnt extract multipart: {}", e)),
                    ImageExtractorError::MissingField(field) => {
                        HttpResponse::BadRequest().json(format!("Missing field: {}", field))
                    }
                    ImageExtractorError::MissingData => HttpResponse::BadRequest()
                        .json("Missing data, expected 'image' and 'alt_text'"),
                    ImageExtractorError::UnexpectedField(e) => {
                        HttpResponse::BadRequest().json(format!(
                            "Unexpected field! Expected 'image' and 'alt_text', got '{}'",
                            e
                        ))
                    }
                    ImageExtractorError::Utf8Error(e) => {
                        HttpResponse::BadRequest().json(format!("Couldnt parse utf8: {}", e))
                    }
                    ImageExtractorError::FileTooLarge => {
                        HttpResponse::PayloadTooLarge().json("File too large")
                    }
                }
            }
        };

    let image = match description_utils::parse_img(image_buffer) {
        Ok(img) => img,
        Err(e) => {
            return match e {
                ImageParsingError::DecodeError(e) => {
                    HttpResponse::UnsupportedMediaType().json(format!("Decode error: {}", e))
                }
                ImageParsingError::NoFormatFound => HttpResponse::BadRequest().json(format!(
                    "No format found. Supported formats: {:?}",
                    ALLOWED_FORMATS
                )),
                ImageParsingError::UnsuppoertedFormat(e) => HttpResponse::UnsupportedMediaType()
                    .json(format!(
                        "Unsupported format, found {:?}. Supported formats: {:?}",
                        e, ALLOWED_FORMATS
                    )),
                ImageParsingError::IoError(e) => {
                    HttpResponse::InternalServerError().json(format!("Image reader error: {}", e))
                }
            }
        }
    };

    let image_dir = format!("{}/{}", IMAGE_DIR, product_id.as_str());

    let path = match description_utils::save_image(image, &image_dir, &file_name) {
        Ok(path) => path,
        Err(e) => {
            log::error!("Couldnt save image to {}, Error: {}", &image_dir, e);
            return HttpResponse::InternalServerError()
                .json("Internal Server Error while saving image");
        }
    };

    let alt_text = match text_fields.remove("alt_text") {
        Some(alt_text) => alt_text,
        None => {
            return HttpResponse::InternalServerError().json("Internal Server Error");
        }
    };

    let created_component = product::description::create_image_component(
        &pool,
        product_id.as_str(),
        product::description::ImageComponent::new(None, path, alt_text),
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
