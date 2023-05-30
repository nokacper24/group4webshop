use std::fs;

use actix_multipart::Multipart;
use actix_web::{delete, get, patch, post, put, web, HttpRequest, HttpResponse, Responder};
use image::ImageError;
use log::error;
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};

pub mod descriptions_protected;

use crate::{
    data_access::{error_handling::PostgresDBError, user},
    SharedData,
    {
        data_access::product::{self, Product},
        utils::{
            auth,
            img_multipart::{
                self, ImageExtractorError, ImageParsingError, ALLOWED_FORMATS, IMAGES_DIR,
            },
        },
    },
};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_products);
    cfg.service(create_product);
    cfg.service(delete_product);
    cfg.service(update_product);
    cfg.service(update_availability);
    cfg.service(web::scope("/products").configure(descriptions_protected::configure));
}

#[derive(OpenApi)]
#[openapi(
    paths(
        get_all_products,
        create_product,
        update_product,
        update_availability,
        delete_product,
    ),
    components(
        schemas(
            Product,
        )
    ),
    tags(
        (
            name = "Products",
            description = "Api endpoints for product management."
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
async fn get_all_products(shared_data: web::Data<SharedData>, req: HttpRequest) -> impl Responder {
    let pool = &shared_data.db_pool;
    match auth::validate_user(req, pool).await {
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

    match product::get_products(pool, false).await {
        Ok(products) => HttpResponse::Ok().json(products),
        Err(e) => {
            error!("{}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Form data for creating a new product, all fields are required.
#[derive(Deserialize, Serialize, ToSchema)]
struct NewProductForm {
    product_name: String,
    price_per_unit: i32,
    short_description: String,
    image: Vec<u8>,
}
/// Create a new product.
///
/// Availablity is set to false by default. Must be **PATCH**ed to `true` in a separate request.
#[utoipa::path(
    context_path = "/api/priv",
    post,
    responses(
        (status = 201, description = "Product created", body = Product),
        (status = 401, description = "Unauthorized - no valid authentification"),
        (status = 403, description = "Forbidden - no permission to create products"),
        (status = 400, description = "Bad request - invalid form data"),
        (status = 409, description = "Conflict - product with same name already exists"),
        (status = 413, description = "Payload too large - image too large"),
        (status = 415, description = "Unsupoorted media type - unsupported image format"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "Products",
    request_body(
        content_type = "multipart/form-data",
        description = "Product creation form",
        content = inline(NewProductForm),
    ),
)]
#[post("/products")]
async fn create_product(
    payload: Multipart,
    shared_data: web::Data<SharedData>,
    req: HttpRequest,
) -> impl Responder {
    let pool = &shared_data.db_pool;
    match auth::validate_user(req, pool).await {
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
        match img_multipart::extract_image_and_texts_from_multipart(
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
    if short_description.len() > 256 {
        return HttpResponse::BadRequest()
            .body("Short description must be less than 256 characters.");
    }

    let product_id = product::generate_id(prod_name);

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

    let new_product = Product::new(
        &product_id,
        prod_name,
        price_per_unit,
        short_description,
        &file_name,
        false,
    );

    match product::create_product(pool, new_product).await {
        Ok(product) => HttpResponse::Created().json(product),
        Err(e) => {
            if let Err(io_e) = img_multipart::remove_image(&file_name) {
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

/// Form for updating a product, image is optional (nullable).
#[derive(Deserialize, Serialize, ToSchema)]
struct UpdateProductForm {
    product_name: String,
    price_per_unit: i32,
    short_description: String,
    image: Option<Vec<u8>>,
}

/// Update a product.
#[utoipa::path(
    context_path = "/api/priv",
    put,
    tag = "Products",
    responses(
        (status = 200, description = "Product updated", body = Product),
        (status = 400, description = "Bad request - invalid form data"),
        (status = 401, description = "Unauthorized - no valid authentification"),
        (status = 403, description = "Forbidden - no permission to update products"),
        (status = 404, description = "Not found - product not found"),
        (status = 413, description = "Payload too large - image too large"),
        (status = 415, description = "Unsupoorted media type - unsupported image format"),
        (status = 500, description = "Internal Server Error")
    ),
    params(
        ("product_id", description = "Product ID of the product to update", example = "new_product")
    ),
    request_body(
        content_type = "multipart/form-data",
        description = "Product creation form",
        content = inline(UpdateProductForm),
    ),
)]
#[put("/products/{product_id}")]
async fn update_product(
    payload: Multipart,
    shared_data: web::Data<SharedData>,
    req: HttpRequest,
    product_id: web::Path<String>,
) -> impl Responder {
    let pool = &shared_data.db_pool;
    match auth::validate_user(req, pool).await {
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

    let unupadted_product = match product::get_product_by_id(pool, &product_id).await {
        Ok(product) => product,
        Err(e) => match e {
            sqlx::Error::RowNotFound => return HttpResponse::NotFound().json("Product not found"),
            _ => return HttpResponse::InternalServerError().json("Internal Server Error"),
        },
    };

    let (extracted_image, text_fields) =
        match img_multipart::extract_image_and_texts_from_multipart(
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
            if let Err(e) = img_multipart::remove_image(unupadted_product.main_image()) {
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
    if short_description.len() > 256 {
        return HttpResponse::BadRequest()
            .body("Short description must be less than 256 characters.");
    }

    let product_to_update = Product::new(
        unupadted_product.product_id(),
        prod_name,
        price_per_unit,
        short_description,
        &new_image_path,
        unupadted_product.available(),
    );

    match product::update_product(pool, &product_to_update).await {
        Ok(updated_product) => HttpResponse::Ok().json(updated_product),
        Err(e) => {
            log::error!("Couldnt update product: {}", e);
            HttpResponse::InternalServerError().json("Internal Server Error")
        }
    }
}

#[derive(Deserialize, ToSchema)]
struct AvailibilityBody {
    available: bool,
}
/// Update the availability of a product.
#[utoipa::path(
    context_path = "/api/priv",
    patch,
    tag = "Products",
    responses(
        (status = 204, description = "Product availability updated"),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized - no valid authentification"),
        (status = 403, description = "Forbidden - no permission update products"),
        (status = 404, description = "Not found - product not found"),
        (status = 500, description = "Internal Server Error")
    ),
    params(
        ("product_id", description = "Product ID of the product to update", example = "new_product")
    ),
    request_body(
        content_type = "application/json",
        description = "New availability of the product",
        content = inline(AvailibilityBody),
    ),
)]
#[patch("/products/{product_id}/available")]
async fn update_availability(
    shared_data: web::Data<SharedData>,
    product_id: web::Path<String>,
    req_body: web::Json<AvailibilityBody>,
    req: HttpRequest,
) -> impl Responder {
    let available = req_body.available;
    let pool = &shared_data.db_pool;
    match auth::validate_user(req, pool).await {
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

    match product::update_product_available(pool, &product_id, available).await {
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

/// Delete a product.
#[utoipa::path(
    context_path = "/api/priv",
    delete,
    tag = "Products",
    responses(
        (status = 204, description = "Product deleted"),
        (status = 401, description = "Unauthorized - no valid authentification"),
        (status = 403, description = "Forbidden - no permission delete a product"),
        (status = 404, description = "Not found - product not found"),
        (status = 409, description = "Conflict - product is used in a license"),
        (status = 500, description = "Internal Server Error")
    ),
    params(
        ("product_id", description = "Product ID of the product to delete", example = "new_product")
    )
)]
#[delete("/products/{product_id}")]
async fn delete_product(
    shared_data: web::Data<SharedData>,
    product_id: web::Path<String>,
    req: HttpRequest,
) -> impl Responder {
    let pool = &shared_data.db_pool;
    match auth::validate_user(req, pool).await {
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

    match product::product_exists(pool, &product_id).await {
        Ok(exists) => {
            if !exists {
                return HttpResponse::NotFound().json("Product not found");
            }
        }
        Err(_) => return HttpResponse::InternalServerError().json("Internal Server Error"),
    }

    let all_images = match product::get_all_image_paths(pool, &product_id).await {
        Ok(images) => images,
        Err(e) => {
            log::error!("Couldnt get all image paths: {}", e);
            return HttpResponse::InternalServerError().json("Internal Server Error");
        }
    };

    match product::delete_product(pool, &product_id).await {
        Ok(_) => {
            for image in all_images {
                if let Err(e) = img_multipart::remove_image(&image) {
                    match e.kind() {
                        std::io::ErrorKind::NotFound => {} // Image already deleted
                        _ => {
                            error!("Couldnt remove image from file system: {}", e);
                        }
                    }
                };
            }
            if let Err(e) = fs::remove_dir(format!("{}/{}", IMAGES_DIR, product_id)) {
                log::error!("Couldnt remove image folder from file system: {}", e);
            };
            HttpResponse::NoContent().finish()
        }
        Err(e) => {
            log::debug!("Couldnt delete product: {}", e);
            match e {
                sqlx::Error::Database(db_e) => match PostgresDBError::from_str(db_e) {
                    PostgresDBError::ForeignKeyViolation => {
                        HttpResponse::Conflict().json("Product is licensed to some user or company")
                    }
                    _ => HttpResponse::InternalServerError().json("Internal Server Error"),
                },
                _ => HttpResponse::InternalServerError().json("Internal Server Error"),
            }
        }
    }
}
