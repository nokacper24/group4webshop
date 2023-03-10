use actix_web::{get, post, put, web, HttpResponse, Responder};
use sqlx::{Pool, Postgres};
use utoipa::OpenApi;

pub mod descriptions;

use crate::data_access::product::{self, PartialProduct, Product};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(all_products);
    cfg.service(product_by_id);
    cfg.service(web::scope("/products").configure(descriptions::configure));
}

#[derive(OpenApi)]
#[openapi(
    paths(
        all_products,
        product_by_id,
    ),
    components(
        schemas(Product)
    ),
    tags(
        (name = "Products", description = "Api endpoints for products")
    ),
)]
pub struct ProductsApiDoc;

#[utoipa::path(
    context_path = "/api",
    get,
    responses(
    (status = 200, description = "List of all available products", body = Vec<Product>),
    (status = 500, description = "Internal Server Error"),
)
)]
#[get("/products")]
pub async fn all_products(pool: web::Data<Pool<Postgres>>) -> impl Responder {
    let products = product::get_products(&pool).await;

    //error check
    if products.is_err() {
        return HttpResponse::InternalServerError().json("Internal Server Error");
    }

    //parse to json
    if let Ok(products) = products {
        return HttpResponse::Ok().json(products);
    }

    HttpResponse::InternalServerError().json("Internal Server Error")
}

/// Get a specific product by name
#[utoipa::path (
    context_path = "/api",
    get,
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
pub async fn product_by_id(
    pool: web::Data<Pool<Postgres>>,
    product_id: web::Path<String>,
) -> impl Responder {
    let product = product::get_product_by_id(&pool, product_id.as_str()).await;

    match product {
        Ok(product) => HttpResponse::Ok().json(product),
        Err(e) => match e {
            sqlx::Error::RowNotFound => HttpResponse::NotFound().json("Product not found"),
            _ => HttpResponse::InternalServerError().json("Internal Server Error"),
        },
    }
}