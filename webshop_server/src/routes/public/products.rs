use actix_web::{get, web, HttpResponse, Responder};
use log::error;
use utoipa::OpenApi;

pub mod descriptions;

use crate::{
    data_access::product::{self, Product},
    SharedData,
};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(all_available_products);
    cfg.service(product_by_id);
    cfg.service(web::scope("/products").configure(descriptions::configure));
}

#[derive(OpenApi)]
#[openapi(
    paths(
        all_available_products,
        product_by_id,
    ),
    components(
        schemas(Product)
    ),
    tags(
        (name = "Products", description = "Api endpoints for products"),
    ),
)]
pub struct ProductsApiDoc;

/// Get all available products
#[utoipa::path(
    context_path = "/api",
    get,
    tag = "Products",
    responses(
    (status = 200, description = "List of all available products", body = Vec<Product>),
    (status = 500, description = "Internal Server Error"),
)
)]
#[get("/products")]
async fn all_available_products(shared_data: web::Data<SharedData>) -> impl Responder {
    let pool = &shared_data.db_pool;
    match product::get_products(pool, true).await {
        Ok(products) => HttpResponse::Ok().json(products),
        Err(e) => {
            error!("Error: {}", e);
            HttpResponse::InternalServerError().json("Internal Server Error")
        }
    }
}

/// Get a specific product by id
#[utoipa::path (
    context_path = "/api",
    get,
    tag = "Products",
    responses(
        (status = 200, description = "Returns a specific product", body = Product),
        (status = 404, description = "Product not found"),
        (status = 500, description = "Internal Server Error"),
        ),
    params(
        ("product_id", description = "The id of the product"),
        )
    )
]
#[get("/products/{product_id}")]
async fn product_by_id(
    shared_data: web::Data<SharedData>,
    product_id: web::Path<String>,
) -> impl Responder {
    let pool = &shared_data.db_pool;
    let product = product::get_product_by_id(pool, product_id.as_str()).await;

    match product {
        Ok(product) => HttpResponse::Ok().json(product),
        Err(e) => match e {
            sqlx::Error::RowNotFound => HttpResponse::NotFound().json("Product not found"),
            _ => HttpResponse::InternalServerError().json("Internal Server Error"),
        },
    }
}
