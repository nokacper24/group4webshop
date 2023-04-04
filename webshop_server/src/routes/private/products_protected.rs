use std::fs;

use actix_multipart::Multipart;
use actix_web::{delete, get, patch, post, put, web, HttpRequest, HttpResponse, Responder};
use image::ImageError;
use log::error;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use utoipa::{OpenApi, ToSchema};

pub mod descriptions_protected;

use crate::{
    data_access::{error_handling::PostgresDBError, user},
    routes::private::products_protected::descriptions_protected::description_utils::{
        self, ImageExtractorError, ImageParsingError,
    },
    {
        data_access::product::{self, Product},
        utils::auth,
    },
};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_products);
    cfg.service(create_product);
    cfg.service(delete_product);
    cfg.service(update_product);
    cfg.service(update_availability);
    cfg.service(
        web::scope("/products")
            .configure(descriptions_protected::configure)
            .default_service(web::route().to(crate::routes::api_not_found)),
    );
}

#[derive(OpenApi)]
#[openapi(
    paths(
        get_all_products,
        create_product,
        update_product,
    ),
    components(
        schemas(
            Product,
            NewProductForm,
        )
    ),
    tags(
        (
            name = "Products",
            description = "Api endpoints for products"
        ),
    ),
)]
pub struct ProductsApiDoc;

/// Get all products.
///
/// Includes products that are not available.
#[utoipa::path(
    context_path = "/api/priv",
    get,
    tag = "Products",
    responses(
    (status = 200, description = "List of all products", body = Vec<Product>),
    (status = 500, description = "Internal Server Error"),
)
)]
#[get("/products")]
pub async fn get_all_products(pool: web::Data<Pool<Postgres>>, req: HttpRequest) -> impl Responder {
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

    match product::get_products(&pool, false).await {
        Ok(products) => HttpResponse::Ok().json(products),
        Err(e) => {
            error!("{}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[derive(Deserialize, Serialize, ToSchema)]
struct NewProductForm {
    product_name: String,
    price_per_unit: i32,
    short_description: String,
    #[schema(value_type = String, format = Binary)]
    image: Vec<u8>,
}
/// Create a new product.
#[utoipa::path(
    context_path = "/api/priv",
    post,
    responses(
        (status = 200, description = "Product created", body = Product),
        (status = 403, description = "Forbidden - no permission to create products"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "Products",
    request_body(
        content_type = "multipart/form-data",
        description = "Product creation form",
        content = NewProductForm,
    ),
)]
#[post("/products")]
pub async fn create_product(
    payload: Multipart,
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

    let (extracted_image, text_fields) =
        match description_utils::extract_image_and_texts_from_multipart(
            payload,
            vec!["product_name", "price_per_unit", "short_description"],
        )
        .await
        {
            Ok((extracted_image, text_fields)) => (extracted_image, text_fields),
            Err(e) => return match e {
                ImageExtractorError::Utf8Error(e) => HttpResponse::BadRequest().json(format!("Couldnt parse utf8: {}", e)),
                ImageExtractorError::MultipartError(e) => HttpResponse::InternalServerError().json(format!("Couldnt extract multipart: {}", e)),
                ImageExtractorError::MissingContentDisposition(field) => HttpResponse::BadRequest().json(format!("Missing field: {}", field)),
                ImageExtractorError::MissingData => HttpResponse::BadRequest().json("Missing data, expected 'product_name', 'price_per_unit', 'short_description', 'image'"),
                ImageExtractorError::UnexpectedField(field) => HttpResponse::BadRequest().json(format!("Unexpected field! Expected expected 'product_name', 'price_per_unit', 'short_description', 'image', got '{}'",field)),
                ImageExtractorError::FileTooLarge => HttpResponse::PayloadTooLarge().json("File too large"),
            },
        };
    let extracted_img = match extracted_image {
        Some(extracted_image) => extracted_image,
        None => return HttpResponse::BadRequest().json("Missing image"),
    };
    let image = match description_utils::parse_img(extracted_img.img_buffer) {
        Ok(image) => image,
        Err(e) => {
            return match e {
                ImageParsingError::DecodeError(e) => {
                    HttpResponse::UnsupportedMediaType().json(format!("Decode error: {}", e))
                }
                ImageParsingError::NoFormatFound => HttpResponse::BadRequest().json(format!(
                    "No format found. Supported formats: {:?}",
                    descriptions_protected::ALLOWED_FORMATS
                )),
                ImageParsingError::UnsuppoertedFormat(e) => HttpResponse::UnsupportedMediaType()
                    .json(format!(
                        "Unsupported format, found {:?}. Supported formats: {:?}",
                        e,
                        descriptions_protected::ALLOWED_FORMATS
                    )),
                ImageParsingError::IoError(e) => {
                    HttpResponse::InternalServerError().json(format!("Image reader error: {}", e))
                }
            }
        }
    };

    let prod_name = match text_fields.get("product_name") {
        Some(prod_name) => prod_name,
        None => return HttpResponse::InternalServerError().finish(),
    };
    let price_per_unit = match text_fields.get("price_per_unit") {
        Some(price_per_unit) => match price_per_unit.parse::<f32>() {
            Ok(price_per_unit) => price_per_unit,
            Err(_) => return HttpResponse::BadRequest().body("Price must be a number."),
        },
        None => return HttpResponse::InternalServerError().finish(),
    };
    if price_per_unit <= 0.0 {
        return HttpResponse::BadRequest().body("Price must be greater than 0.");
    }
    let short_description = match text_fields.get("short_description") {
        Some(short_description) => short_description,
        None => return HttpResponse::InternalServerError().finish(),
    };

    let product_id = product::generate_id(prod_name);

    let path = format!("{}/{}", descriptions_protected::IMAGE_DIR, product_id);
    let file_name = match description_utils::save_image(image, &path, &extracted_img.file_name) {
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

    let new_product = Product::new(
        &product_id,
        prod_name,
        price_per_unit,
        short_description,
        &file_name,
        false,
    );

    match product::create_product(&pool, new_product).await {
        Ok(product) => HttpResponse::Created().json(product),
        Err(e) => {
            if let Err(io_e) = description_utils::remove_image(&file_name) {
                log::error!("Couldnt remove image from file system: {}", io_e);
                return HttpResponse::InternalServerError().json("Internal Server Error");
            };
            match e {
                sqlx::Error::Database(e) => match PostgresDBError::from_str(e) {
                    PostgresDBError::UniqueViolation => {
                        HttpResponse::Conflict().json("Product with this name already exists")
                    }
                    _ => HttpResponse::InternalServerError().json("Internal Server Error"),
                },
                _ => HttpResponse::InternalServerError().json("Internal Server Error"),
            }
        }
    }
}

/// Updates a product
#[utoipa::path(
    context_path = "/api/priv",
    put,
    tag = "Products",
    responses(
        (status = 200, description = "Product updated", body = Product),
        (status = 403, description = "Forbidden - no permission to update products"),
        (status = 500, description = "Internal Server Error")
)
)]
#[put("/products/{product_id}")]
pub async fn update_product(
    payload: Multipart,
    pool: web::Data<Pool<Postgres>>,
    req: HttpRequest,
    product_id: web::Path<String>,
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

    let unupadted_product = match product::get_product_by_id(&pool, &product_id).await {
        Ok(product) => product,
        Err(e) => match e {
            sqlx::Error::RowNotFound => return HttpResponse::NotFound().json("Product not found"),
            _ => return HttpResponse::InternalServerError().json("Internal Server Error"),
        },
    };

    let (extracted_image, text_fields) =
        match description_utils::extract_image_and_texts_from_multipart(
            payload,
            vec!["product_name", "price_per_unit", "short_description"],
        )
        .await
        {
            Ok((extracted_image, text_fields)) => (extracted_image, text_fields),
            Err(e) => return match e {
                ImageExtractorError::Utf8Error(e) => HttpResponse::BadRequest().json(format!("Couldnt parse utf8: {}", e)),
                ImageExtractorError::MultipartError(e) => HttpResponse::InternalServerError().json(format!("Couldnt extract multipart: {}", e)),
                ImageExtractorError::MissingContentDisposition(field) => HttpResponse::BadRequest().json(format!("Missing field: {}", field)),
                ImageExtractorError::MissingData => HttpResponse::BadRequest().json("Missing data, expected 'product_name', 'price_per_unit', 'short_description', 'image'"),
                ImageExtractorError::UnexpectedField(field) => HttpResponse::BadRequest().json(format!("Unexpected field! Expected expected 'product_name', 'price_per_unit', 'short_description', 'image', got '{}'",field)),
                ImageExtractorError::FileTooLarge => HttpResponse::PayloadTooLarge().json("File too large"),
            },
        };

    let new_image_path = match extracted_image {
        Some(extracted_image) => {
            // new image was provided, remove old one and save new one
            let new_img = match description_utils::parse_img(extracted_image.img_buffer) {
                Ok(image) => image,
                Err(e) => {
                    return match e {
                        ImageParsingError::DecodeError(e) => HttpResponse::UnsupportedMediaType()
                            .json(format!("Decode error: {}", e)),
                        ImageParsingError::NoFormatFound => {
                            HttpResponse::BadRequest().json(format!(
                                "No format found. Supported formats: {:?}",
                                descriptions_protected::ALLOWED_FORMATS
                            ))
                        }
                        ImageParsingError::UnsuppoertedFormat(e) => {
                            HttpResponse::UnsupportedMediaType().json(format!(
                                "Unsupported format, found {:?}. Supported formats: {:?}",
                                e,
                                descriptions_protected::ALLOWED_FORMATS
                            ))
                        }
                        ImageParsingError::IoError(e) => HttpResponse::InternalServerError()
                            .json(format!("Image reader error: {}", e)),
                    }
                }
            };
            let path = format!("{}/{}", descriptions_protected::IMAGE_DIR, product_id);
            let new_path =
                match description_utils::save_image(new_img, &path, &extracted_image.file_name) {
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
            if let Err(e) = description_utils::remove_image(unupadted_product.main_image()) {
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
        // no new image was provided, use old one
        None => String::from(unupadted_product.main_image()),
    };

    let prod_name = match text_fields.get("product_name") {
        Some(prod_name) => prod_name,
        None => return HttpResponse::InternalServerError().finish(),
    };
    let price_per_unit = match text_fields.get("price_per_unit") {
        Some(price_per_unit) => match price_per_unit.parse::<f32>() {
            Ok(price_per_unit) => price_per_unit,
            Err(_) => return HttpResponse::BadRequest().json("Price must be a number."),
        },
        None => return HttpResponse::InternalServerError().finish(),
    };
    if price_per_unit <= 0.0 {
        return HttpResponse::BadRequest().body("Price must be greater than 0.");
    }

    let short_description = match text_fields.get("short_description") {
        Some(short_description) => short_description,
        None => return HttpResponse::InternalServerError().finish(),
    };

    let product_to_update = Product::new(
        unupadted_product.product_id(),
        prod_name,
        price_per_unit,
        short_description,
        &new_image_path,
        unupadted_product.available(),
    );

    match product::update_product(&pool, &product_to_update).await {
        Ok(updated_product) => HttpResponse::Ok().json(updated_product),
        Err(e) => {
            log::error!("Couldnt update product: {}", e);
            HttpResponse::InternalServerError().json("Internal Server Error")
        }
    }
}

#[patch("/products/{product_id}/available")]
pub async fn update_availability(
    pool: web::Data<Pool<Postgres>>,
    product_id: web::Path<String>,
    available: web::Json<bool>,
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

    match product::update_product_available(&pool, &product_id, available.into_inner()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => match e {
            sqlx::Error::RowNotFound => HttpResponse::NotFound().json("Product not found"),
            _ => {
                error!("{}", e);
                HttpResponse::InternalServerError().finish()
            }
        },
    }
}

#[delete("/products/{product_id}")]
pub async fn delete_product(
    pool: web::Data<Pool<Postgres>>,
    product_id: web::Path<String>,
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

    let product_id = product_id.into_inner();

    match product::product_exists(&pool, &product_id).await {
        Ok(exists) => {
            if !exists {
                return HttpResponse::NotFound().json("Product not found");
            }
        }
        Err(_) => return HttpResponse::InternalServerError().json("Internal Server Error"),
    }

    let images = match product::description::get_all_image_paths(&pool, &product_id).await {
        Ok(images) => images,
        Err(_) => return HttpResponse::InternalServerError().json("Internal Server Error"),
    };

    for image in images {
        if let Err(e) = description_utils::remove_image(&image) {
            log::error!("Couldnt remove image from file system: {}", e);
        };
    }

    match product::delete_product(&pool, &product_id).await {
        Ok(img_path) => {
            if let Err(e) = description_utils::remove_image(&img_path) {
                log::error!("Couldnt remove main image from file system: {}", e);
            };
            if let Err(e) = fs::remove_dir(format!(
                "{}/{}",
                descriptions_protected::IMAGE_DIR,
                product_id
            )) {
                log::error!("Couldnt remove image folder from file system: {}", e);
            };
            HttpResponse::NoContent().finish()
        }
        Err(e) => match e {
            sqlx::Error::Database(db_e) => match PostgresDBError::from_str(db_e) {
                PostgresDBError::ForeignKeyViolation => {
                    HttpResponse::Conflict().json("Product is licensed to some user or company")
                }
                _ => HttpResponse::InternalServerError().json("Internal Server Error"),
            },
            _ => HttpResponse::InternalServerError().json("Internal Server Error"),
        },
    }
}
