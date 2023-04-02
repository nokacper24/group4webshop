use crate::{
    data_access::{
        error_handling,
        product::{
            self,
            description::{DescriptionCompError, DescriptionUpdateError},
        },
        user,
    },
    routes::private::products_protected::descriptions_protected::description_utils::{
        ImageExtractorError, ImageParsingError,
    },
    utils::auth,
};
use actix_multipart::Multipart;
use actix_web::{delete, patch, post, put, web, HttpRequest, HttpResponse, Responder};
use image::{ImageError, ImageFormat};
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
    cfg.service(set_full_width);
    cfg.service(upload_image);
    cfg.service(update_priorities);
    cfg.service(update_text_description);
    cfg.service(update_image_description);
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

#[patch("/{product_id}/descriptions/{component_id}/full-width")]
async fn set_full_width(
    pool: web::Data<Pool<Postgres>>,
    path: web::Path<(String, i32)>,
    full_width: web::Json<bool>,
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
    let query_result = product::description::update_full_width(
        &pool,
        product_id.as_str(),
        component_id,
        full_width.into_inner(),
    )
    .await;
    match query_result {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => match e {
            sqlx::Error::RowNotFound => {
                HttpResponse::NotFound().json("Product or description not found")
            }
            _ => HttpResponse::InternalServerError().json("Internal Server Error"),
        },
    }
}

#[patch("/{product_id}/descriptions/all/priorities")]
async fn update_priorities(
    pool: web::Data<Pool<Postgres>>,
    product_id: web::Path<String>,
    ids_and_priotities: web::Json<Vec<(i32, i32)>>,
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

    let ids = ids_and_priotities
        .iter()
        .map(|(id, _)| *id)
        .collect::<Vec<i32>>();
    let valid =
        match product::description::verify_component_ids(&pool, product_id.as_str(), &ids).await {
            Ok(valid) => valid,
            Err(e) => match e {
                sqlx::Error::RowNotFound => {
                    return HttpResponse::NotFound().json("No descriptions found for such product")
                }
                _ => {
                    error!("{}", e);
                    return HttpResponse::InternalServerError().json("Internal Server Error");
                }
            },
        };

    if !valid {
        return HttpResponse::Conflict().json("Not all components belong to this product");
    }

    let query_result = product::description::update_priorities_bulk(
        &pool,
        product_id.as_str(),
        &ids_and_priotities,
    )
    .await;

    match query_result {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => match e {
            sqlx::Error::Database(e) => match error_handling::PostgresDBError::from_str(e) {
                error_handling::PostgresDBError::UniqueViolation => {
                    HttpResponse::Conflict().json("Conflict: Priority already in use")
                }
                e => {
                    error!("PostgresDBError while updating priorities: {:?}", e);
                    HttpResponse::InternalServerError().json("Internal Server Error")
                }
            },
            e => {
                error!("{}", e);
                HttpResponse::InternalServerError().json("Internal Server Error")
            }
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

    let (image, mut text_fields) =
        match description_utils::extract_image_and_texts_from_multipart(payload, vec!["alt_text"])
            .await
        {
            Ok((image, text_fields)) => (image, text_fields),
            Err(e) => {
                return match e {
                    ImageExtractorError::MultipartError(e) => HttpResponse::InternalServerError()
                        .json(format!("Couldnt extract multipart: {}", e)),
                    ImageExtractorError::MissingContentDisposition(field) => {
                        HttpResponse::BadRequest()
                            .json(format!("Missing content dispositio: {}", field))
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

    let extracted_image = match image {
        Some(image) => image,
        None => return HttpResponse::BadRequest().json("Missing image"),
    };

    let image = match description_utils::parse_img(extracted_image.img_buffer) {
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

    let path = match description_utils::save_image(image, &image_dir, &extracted_image.file_name) {
        Ok(path) => path,
        Err(e) => match e {
            ImageError::Unsupported(e) => {
                return HttpResponse::UnsupportedMediaType().json(format!("Format error: {}", e))
            }
            _ => {
                log::error!("Couldnt save image to {}, Error: {}", &image_dir, e);
                return HttpResponse::InternalServerError().finish();
            }
        },
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

#[put("/{product_id}/descriptions/text/{component_id}")]
async fn update_text_description(
    path_parms: web::Path<(String, i32)>,
    description: web::Json<product::description::TextComponent>,
    pool: web::Data<Pool<Postgres>>,
    req: HttpRequest,
) -> impl Responder {
    let product_id = path_parms.0.as_str();
    let component_id = path_parms.1;

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

    let updated_component = product::description::update_text_component(
        &pool,
        product_id,
        description.into_inner(),
        component_id,
    )
    .await;

    match updated_component {
        Ok(description_component) => HttpResponse::Ok().json(description_component),
        Err(e) => match e {
            DescriptionUpdateError::WrongComponentType => HttpResponse::Conflict().json(format!(
                "Wrong component type! Component with id {} is not a text component.",
                component_id
            )),
            DescriptionUpdateError::NotFound => {
                HttpResponse::NotFound().json("No component found with given id and product.")
            }
            DescriptionUpdateError::SqlxError(e) => {
                error!("{}", e);
                HttpResponse::InternalServerError().json("Internal Server Error")
            }
        },
    }
}

#[put("/{product_id}/descriptions/image/{component_id}")]
async fn update_image_description(
    payload: Multipart,
    path_parms: web::Path<(String, i32)>,
    pool: web::Data<Pool<Postgres>>,
    req: HttpRequest,
) -> impl Responder {
    let product_id = path_parms.0.as_str();
    let component_id = path_parms.1;

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

    let unupdated_desc = match product::description::get_description_component_checked(
        &pool,
        product_id,
        component_id,
    )
    .await
    {
        Ok(desc) => desc,
        Err(e) => {
            return match e {
                sqlx::Error::RowNotFound => {
                    HttpResponse::NotFound().json("No component found with given id and product.")
                }
                _ => {
                    error!("{}", e);
                    HttpResponse::InternalServerError().json("Internal Server Error")
                }
            }
        }
    };
    let unupdated_img_comp = match unupdated_desc.image() {
        Some(img_comp) => img_comp,
        None => {
            return HttpResponse::Conflict().json(format!(
                "Wrong component type! Component with id {} is not an image component.",
                path_parms.1
            ))
        }
    };

    let (image, mut text_fields) =
        match description_utils::extract_image_and_texts_from_multipart(payload, vec!["alt_text"])
            .await
        {
            Ok((image, text_fields)) => (image, text_fields),
            Err(e) => {
                return match e {
                    ImageExtractorError::MultipartError(e) => HttpResponse::InternalServerError()
                        .json(format!("Couldnt extract multipart: {}", e)),
                    ImageExtractorError::MissingContentDisposition(field) => {
                        HttpResponse::BadRequest()
                            .json(format!("Missing content dispositio: {}", field))
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

    let new_image_path = match image {
        Some(image) => {
            let new_img = match description_utils::parse_img(image.img_buffer) {
                Ok(img) => img,
                Err(e) => {
                    return match e {
                        ImageParsingError::DecodeError(e) => HttpResponse::UnsupportedMediaType()
                            .json(format!("Decode error: {}", e)),
                        ImageParsingError::NoFormatFound => HttpResponse::BadRequest().json(
                            format!("No format found. Supported formats: {:?}", ALLOWED_FORMATS),
                        ),
                        ImageParsingError::UnsuppoertedFormat(e) => {
                            HttpResponse::UnsupportedMediaType().json(format!(
                                "Unsupported format, found {:?}. Supported formats: {:?}",
                                e, ALLOWED_FORMATS
                            ))
                        }
                        ImageParsingError::IoError(e) => HttpResponse::InternalServerError()
                            .json(format!("Image reader error: {}", e)),
                    }
                }
            };

            let image_dir = format!("{}/{}", IMAGE_DIR, product_id);

            let new_img_path =
                match description_utils::save_image(new_img, &image_dir, &image.file_name) {
                    Ok(path) => path,
                    Err(e) => match e {
                        ImageError::Unsupported(e) => {
                            return HttpResponse::UnsupportedMediaType()
                                .json(format!("Format error: {}", e))
                        }
                        _ => {
                            log::error!("Couldnt save image to {}, Error: {}", &image_dir, e);
                            return HttpResponse::InternalServerError().finish();
                        }
                    },
                };
            if let Err(e) = description_utils::remove_image(unupdated_img_comp.image_path()) {
                match e.kind() {
                    std::io::ErrorKind::NotFound => {} // Image already deleted
                    _ => {
                        error!("{}", e);
                        return HttpResponse::InternalServerError().json("Internal Server Error");
                    }
                }
            }
            new_img_path
        }
        None => unupdated_img_comp.image_path().to_string(),
    };

    let alt_text = match text_fields.remove("alt_text") {
        Some(alt_text) => alt_text,
        None => {
            return HttpResponse::InternalServerError().json("Internal Server Error");
        }
    };

    let new_img_comp = product::description::ImageComponent::new(None, new_image_path, alt_text);

    let updated_component =
        product::description::update_image_component(&pool, product_id, new_img_comp, component_id)
            .await;

    match updated_component {
        Ok(description_component) => HttpResponse::Ok().json(description_component),
        Err(e) => match e {
            DescriptionUpdateError::WrongComponentType => HttpResponse::Conflict().json(format!(
                "Wrong component type! Component with id {} is not an image component.",
                component_id
            )),
            DescriptionUpdateError::NotFound => {
                HttpResponse::NotFound().json("No component found with given id and product.")
            }
            DescriptionUpdateError::SqlxError(e) => {
                error!("{}", e);
                HttpResponse::InternalServerError().json("Internal Server Error")
            }
        },
    }
}
