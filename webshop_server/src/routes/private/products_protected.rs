use actix_web::{post, put, web, HttpResponse, Responder};
use sqlx::{Pool, Postgres};
use utoipa::OpenApi;

pub mod descriptions_protected;

use crate::data_access::product::{self, PartialProduct, Product};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(create_product);
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
    pool: web::Data<Pool<Postgres>>,
    product: web::Json<PartialProduct>,
) -> impl Responder {
    let product = product::create_product(&pool, &product).await;

    //error check
    if product.is_err() {
        return HttpResponse::InternalServerError().json("Internal Server Error");
    }

    //parse to json
    if let Ok(product) = product {
        return HttpResponse::Ok().json(product);
    }

    HttpResponse::InternalServerError().json("Internal Server Error")
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
