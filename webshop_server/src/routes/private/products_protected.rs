use std::fs;

use actix_multipart::Multipart;
use actix_web::{delete, post, put, web, HttpResponse, Responder};
use sqlx::{Pool, Postgres};
use utoipa::OpenApi;

pub mod descriptions_protected;

use crate::{
    data_access::{error_handling::PostgresDBError, user},
    routes::private::products_protected::descriptions_protected::description_utils::{
        self, ImageExtractorError, ImageParsingError,
    },
    {
        data_access::product::{self, Product},
        middlewares::auth,
    },
};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(create_product);
    cfg.service(delete_product);
    cfg.service(update_product);
    cfg.service(web::scope("/products").configure(descriptions_protected::configure));
}

#[derive(OpenApi)]
#[openapi(
    paths(
        create_product,
        update_product,
    ),
    components(
        schemas(Product)
    ),
    tags(
        (name = "Products", description = "Api endpoints for products")
    ),
)]
pub struct ProtectedProductsApiDoc;

#[utoipa::path(
    context_path = "/api",
    post,
    responses(
    (status = 200, description = "Product created", body = Product),
    (status = 403, description = "Forbidden - no permission to create products"),
    (status = 500, description = "Internal Server Error")
)
)]
#[post("/products")]
pub async fn create_product(
    payload: Multipart,
    pool: web::Data<Pool<Postgres>>,
    req_token: Option<web::ReqData<auth::Token>>,
) -> impl Responder {
    match auth::check_role(req_token, &pool).await {
        Ok(role) => {
            if role != user::Role::Admin {
                return HttpResponse::Forbidden().json("Forbidden");
            }
        }
        Err(e) => match e {
            auth::AuthError::BadToken => return HttpResponse::Unauthorized().json("Unauthorized"),
            auth::AuthError::SqlxError(_) => {
                return HttpResponse::InternalServerError().json("Internal Server Error")
            }
        },
    };

    let (image_buffer, file_name, text_fields) =
        match description_utils::extract_image_and_texts_from_multipart(
            payload,
            vec!["product_name", "price_per_unit", "short_description"],
        )
        .await
        {
            Ok((image_buffer, file_name, text_fields)) => (image_buffer, file_name, text_fields),
            Err(e) => return match e {
                ImageExtractorError::Utf8Error(e) => HttpResponse::BadRequest().json(format!("Couldnt parse utf8: {}", e)),
                ImageExtractorError::MultipartError(e) => HttpResponse::InternalServerError().json(format!("Couldnt extract multipart: {}", e)),
                ImageExtractorError::MissingField(field) => HttpResponse::BadRequest().json(format!("Missing field: {}", field)),
                ImageExtractorError::MissingData => HttpResponse::BadRequest().json("Missing data, expected 'product_name', 'price_per_unit', 'short_description', 'image'"),
                ImageExtractorError::UnexpectedField(field) => HttpResponse::BadRequest().json(format!("Unexpected field! Expected expected 'product_name', 'price_per_unit', 'short_description', 'image', got '{}'",field)),
                ImageExtractorError::FileTooLarge => HttpResponse::PayloadTooLarge().json("File too large"),
            },
        };
    let image = match description_utils::parse_img(image_buffer) {
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
            Err(_) => return HttpResponse::InternalServerError().finish(),
        },
        None => return HttpResponse::InternalServerError().finish(),
    };
    let short_description = match text_fields.get("short_description") {
        Some(short_description) => short_description,
        None => return HttpResponse::InternalServerError().finish(),
    };

    let product_id = product::generate_id(&prod_name);

    let path = format!("{}/{}", descriptions_protected::IMAGE_DIR, product_id);
    let file_name = match description_utils::save_image(image, &path, &file_name) {
        Ok(file_name) => file_name,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let new_product = Product::new(
        product_id,
        prod_name.to_string(),
        price_per_unit,
        short_description.to_string(),
        file_name.to_string(),
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

#[utoipa::path(
    context_path = "/api",
    post,
    responses(
    (status = 200, description = "Product updated", body = Product),
    (status = 403, description = "Forbidden - no permission to update products"),
    (status = 500, description = "Internal Server Error")
)
)]
#[put("/products")]
pub async fn update_product(
    pool: web::Data<Pool<Postgres>>,
    product: web::Json<Product>,
) -> impl Responder {
    match product::update_product(&pool, &product).await {
        Ok(product) => HttpResponse::Created().json(product),
        Err(e) => match e {
            sqlx::Error::RowNotFound => HttpResponse::NotFound().json("Product not found"),
            _ => HttpResponse::InternalServerError().json("Internal Server Error"),
        },
    }
}

#[delete("/products/{product_id}")]
pub async fn delete_product(
    pool: web::Data<Pool<Postgres>>,
    product_id: web::Path<String>,
    req_token: Option<web::ReqData<auth::Token>>,
) -> impl Responder {
    match auth::check_role(req_token, &pool).await {
        Ok(role) => {
            if role != user::Role::Admin {
                return HttpResponse::Forbidden().json("Forbidden");
            }
        }
        Err(e) => match e {
            auth::AuthError::BadToken => return HttpResponse::Unauthorized().json("Unauthorized"),
            auth::AuthError::SqlxError(_) => {
                return HttpResponse::InternalServerError().json("Internal Server Error")
            }
        },
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
            if let Err(e) = fs::remove_dir(format!("{}/{}", descriptions_protected::IMAGE_DIR, product_id)) {
                log::error!("Couldnt remove image folder from file system: {}", e);
            };
            HttpResponse::NoContent().finish()
        },
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
