use crate::{
    data_access::{
        product,
        testimonial::{self, PartialTestimonial, Testimonial},
        user,
    },
    utils::{
        auth,
        img_multipart::{
            self, ImageExtractorError, ImageParsingError, ALLOWED_FORMATS, IMAGES_DIR,
        },
    },
};

use actix_multipart::Multipart;
use actix_web::{delete, post, put, web, HttpRequest, HttpResponse, Responder};
use image::ImageError;
use log::error;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use utoipa::{OpenApi, ToSchema};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(create_testimonial);
    cfg.service(update_testimonial);
    cfg.service(delete_testimonial);
}

#[derive(OpenApi)]
#[openapi(
    paths(
        create_testimonial,
        update_testimonial,
        delete_testimonial,
    ),
    components(
        schemas(Testimonial, PartialTestimonial, NewTestimonialForm, UpdateTestimonialForm)
    ),
    tags(
        (name = "Testimonials", description = "API endpoints for testimonials management")
    ),
)]
pub struct TestimonialsProtectedOpenApi;

/// Form data for creating a new testimonial, all fields are required.
#[derive(Deserialize, Serialize, ToSchema)]
struct NewTestimonialForm {
    author: String,
    text: String,
    image: Vec<u8>,
}

/// Form data for updating a testimonial, image field is optional.
#[derive(Deserialize, Serialize, ToSchema)]
struct UpdateTestimonialForm {
    author: String,
    text: String,
    image: Option<Vec<u8>>,
}

/// Create a new testimonial for a product.
#[utoipa::path(
    context_path = "/api/priv",
    post,
    tag = "Testimonials",
    responses(
        (status = 201, description = "Testimonial created", body = Testimonial),
        (status = 401, description = "Unauthorized - no valid authentification"),
        (status = 403, description = "Forbidden - no permission to create a testimonial"),
        (status = 400, description = "Bad request - invalid form data"),
        (status = 413, description = "Payload too large - image too large"),
        (status = 415, description = "Unsupoorted media type - unsupported image format"),
        (status = 500, description = "Internal Server Error"),
    ),
    params(
        ("product_id", description = "The id of the product"),
    ),
    request_body(
        content_type = "multipart/form-data",
        description = "Testimonial creation form",
        content = NewTestimonialForm,
    ),
)]
#[post("/testimonials/{product_id}")]
pub async fn create_testimonial(
    pool: web::Data<Pool<Postgres>>,
    product_id: web::Path<String>,
    req: HttpRequest,
    payload: Multipart,
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

    match product::product_exists(&pool, &product_id).await {
        Ok(exists) => {
            if !exists {
                return HttpResponse::NotFound().json("Product not found");
            }
        }
        Err(e) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    }

    let (extracted_image, text_fields) =
        match img_multipart::extract_image_and_texts_from_multipart(payload, vec!["author", "text"])
            .await
        {
            Ok((extracted_image, text_fields)) => (extracted_image, text_fields),
            Err(e) => {
                return match e {
                    ImageExtractorError::Utf8Error(e) => {
                        HttpResponse::BadRequest().json(format!("Couldnt parse utf8: {}", e))
                    }
                    ImageExtractorError::MultipartError(e) => HttpResponse::InternalServerError()
                        .json(format!("Couldnt extract multipart: {}", e)),
                    ImageExtractorError::MissingContentDisposition(field) => {
                        HttpResponse::BadRequest().json(format!("Missing field: {}", field))
                    }
                    ImageExtractorError::MissingData => HttpResponse::BadRequest()
                        .json("Missing data, expected 'author', 'text', 'image'"),
                    ImageExtractorError::UnexpectedField(field) => {
                        HttpResponse::BadRequest().json(format!(
                        "Unexpected field! Expected expected 'author', 'text', 'image', got '{}'",
                        field
                    ))
                    }
                    ImageExtractorError::FileTooLarge => {
                        HttpResponse::PayloadTooLarge().json("File too large")
                    }
                }
            }
        };
    let extracted_img = match extracted_image {
        Some(extracted_image) => extracted_image,
        None => return HttpResponse::BadRequest().json("Missing image"),
    };
    let image = match img_multipart::parse_img(extracted_img.img_buffer) {
        Ok(image) => image,
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

    let author = match text_fields.get("author") {
        Some(author) => author,
        None => return HttpResponse::InternalServerError().finish(),
    };
    let text = match text_fields.get("text") {
        Some(text) => text,
        None => return HttpResponse::InternalServerError().finish(),
    };

    let path = format!("{}/{}", IMAGES_DIR, product_id);
    let file_name = match img_multipart::save_image(image, &path, &extracted_img.file_name) {
        Ok(file_name) => file_name,
        Err(e) => match e {
            ImageError::Unsupported(e) => {
                return HttpResponse::UnsupportedMediaType().json(format!("Format error: {}", e))
            }
            _ => {
                log::error!("Couldnt save image: {}", e);
                return HttpResponse::InternalServerError().finish();
            }
        },
    };

    let testimonial = PartialTestimonial::new(
        author.to_string(),
        text.to_string(),
        file_name,
        product_id.to_string(),
    );

    match testimonial::create_testimonial(&pool, testimonial).await {
        Ok(testimonial) => HttpResponse::Created().json(testimonial),
        Err(e) => {
            log::error!("Couldnt create testimonial: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Update a testimonial.
#[utoipa::path(
    context_path = "/api/priv",
    put,
    tag = "Testimonials",
    responses(
        (status = 200, description = "Testimonial updated", body = Testimonial),
        (status = 401, description = "Unauthorized - no valid authentification"),
        (status = 403, description = "Forbidden - no permission to update a testimonial"),
        (status = 404, description = "Not found - testimonial not found"),
        (status = 400, description = "Bad request - invalid form data"),
        (status = 413, description = "Payload too large - image too large"),
        (status = 415, description = "Unsupoorted media type - unsupported image format"),
        (status = 500, description = "Internal Server Error"),
    ),
    params(
        ("product_id", description = "The id of the product"),
        ("testimonial_id", description = "The id of the testimonial"),
    ),
    request_body(
        content_type = "multipart/form-data",
        description = "Testimonial update form",
        content = UpdateTestimonialForm,
    ),
)]
#[put("/testimonials/{product_id}/{testimonial_id}")]
pub async fn update_testimonial(
    pool: web::Data<Pool<Postgres>>,
    params: web::Path<(String, i32)>,
    req: HttpRequest,
    payload: Multipart,
) -> impl Responder {
    let product_id = &params.0;
    let testimonial_id = params.1;
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

    let unupdated_testimonial =
        match testimonial::get_testimonial_by_prod_and_id(&pool, product_id, &testimonial_id).await
        {
            Ok(testimonial) => testimonial,
            Err(e) => match e {
                sqlx::Error::RowNotFound => {
                    return HttpResponse::NotFound().json("Testimonial not found")
                }
                _ => {
                    log::error!("Couldnt get testimonial: {}", e);
                    return HttpResponse::InternalServerError().finish();
                }
            },
        };

    let (extracted_image, text_fields) =
        match img_multipart::extract_image_and_texts_from_multipart(payload, vec!["author", "text"])
            .await
        {
            Ok((extracted_image, text_fields)) => (extracted_image, text_fields),
            Err(e) => {
                return match e {
                    ImageExtractorError::Utf8Error(e) => {
                        HttpResponse::BadRequest().json(format!("Couldnt parse utf8: {}", e))
                    }
                    ImageExtractorError::MultipartError(e) => HttpResponse::InternalServerError()
                        .json(format!("Couldnt extract multipart: {}", e)),
                    ImageExtractorError::MissingContentDisposition(field) => {
                        HttpResponse::BadRequest().json(format!("Missing field: {}", field))
                    }
                    ImageExtractorError::MissingData => HttpResponse::BadRequest()
                        .json("Missing data, expected 'author', 'text', 'image'"),
                    ImageExtractorError::UnexpectedField(field) => {
                        HttpResponse::BadRequest().json(format!(
                        "Unexpected field! Expected expected 'author', 'text', 'image', got '{}'",
                        field
                    ))
                    }
                    ImageExtractorError::FileTooLarge => {
                        HttpResponse::PayloadTooLarge().json("File too large")
                    }
                }
            }
        };

    let new_img_path = match extracted_image {
        Some(extracted_image) => {
            // new image was provided, remove old one and save new one
            let new_img = match img_multipart::parse_img(extracted_image.img_buffer) {
                Ok(image) => image,
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
            let path = format!("{}/{}", IMAGES_DIR, product_id);
            let new_path =
                match img_multipart::save_image(new_img, &path, &extracted_image.file_name) {
                    Ok(file_name) => file_name,
                    Err(e) => match e {
                        ImageError::Unsupported(e) => {
                            return HttpResponse::UnsupportedMediaType()
                                .json(format!("Format error: {}", e))
                        }
                        _ => {
                            log::error!("Couldnt save image: {}", e);
                            return HttpResponse::InternalServerError()
                                .json("Internal Server Error");
                        }
                    },
                };
            if let Err(e) = img_multipart::remove_image(unupdated_testimonial.author_pic()) {
                match e.kind() {
                    std::io::ErrorKind::NotFound => {} // Image already deleted
                    _ => {
                        error!("Couldnt remove image from file system: {}", e);
                        return HttpResponse::InternalServerError().json("Internal Server Error");
                    }
                }
            }
            new_path
        }
        None => unupdated_testimonial.author_pic().to_string(),
    };

    let new_author = match text_fields.get("author") {
        Some(author) => author.to_string(),
        None => return HttpResponse::InternalServerError().finish(),
    };
    let new_text = match text_fields.get("text") {
        Some(text) => text.to_string(),
        None => return HttpResponse::InternalServerError().finish(),
    };

    let new_testimonial = Testimonial::new(
        testimonial_id,
        new_author,
        new_text,
        new_img_path,
        product_id.to_string(),
    );

    match testimonial::update_testimonial(&pool, new_testimonial).await {
        Ok(updated_testimonial) => HttpResponse::Ok().json(updated_testimonial),
        Err(e) => {
            error!("Couldnt update testimonial: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Deletes a testimonial
#[utoipa::path(
    context_path = "/api/priv",
    delete,
    tag = "Testimonials",
    responses(
        (status = 200, description = "Testimonial deleted", body = Testimonial),
        (status = 401, description = "Unauthorized - no valid authentification"),
        (status = 403, description = "Forbidden - no permission to update a testimonial"),
        (status = 404, description = "Not found - testimonial not found"),
        (status = 400, description = "Bad request - invalid params"),
        (status = 500, description = "Internal Server Error"),
    ),
    params(
        ("product_id", description = "The id of the product"),
        ("testimonial_id", description = "The id of the testimonial"),
    ),
)]
#[delete("/testimonials/{product_id}/{testimonial_id}")]
pub async fn delete_testimonial(
    req: HttpRequest,
    pool: web::Data<Pool<Postgres>>,
    paramas: web::Path<(String, i32)>,
) -> impl Responder {
    let product_id = &paramas.0;
    let testimonial_id = paramas.1;
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

    let deleted_testimonial =
        match testimonial::delete_testimonial(&pool, product_id, &testimonial_id).await {
            Ok(testimonial) => testimonial,
            Err(e) => match e {
                sqlx::Error::RowNotFound => {
                    return HttpResponse::NotFound().json("Testimonial not found")
                }
                _ => {
                    log::error!("Couldnt delete testimonial from db: {}", e);
                    return HttpResponse::InternalServerError().finish();
                }
            },
        };
    if let Err(e) = img_multipart::remove_image(deleted_testimonial.author_pic()) {
        match e.kind() {
            std::io::ErrorKind::NotFound => {} // Image already deleted
            _ => {
                error!("Couldnt remove image from file system: {}", e);
                return HttpResponse::InternalServerError().json("Internal Server Error");
            }
        }
    }
    HttpResponse::Ok().json(deleted_testimonial)
}
