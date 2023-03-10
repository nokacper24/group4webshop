use crate::data_access::testimonial::{self, Testimonial};

use actix_web::{get, web, HttpResponse, Responder};
use sqlx::{Pool, Postgres};
use utoipa::OpenApi;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_testimonials_by_product);
}

#[derive(OpenApi)]
#[openapi(
    paths(
        get_testimonials_by_product
    ),
    components(
        schemas(Testimonial)
    ),
    tags(
        (name = "Testimonials", description = "API endpoints for testimonials")
    ),
)]
pub struct TestimonialsOpenApi;

/// Get all testimonials for a product.
#[utoipa::path(
  context_path = "/api",
  get,
  responses(
  (status = 200, description = "List of all testimonials for a product", body = Vec<Testimonial>),
  (status = 500, description = "Internal Server Error"),
)
)]
#[get("/testimonials/{product_id}")]
pub async fn get_testimonials_by_product(
    pool: web::Data<Pool<Postgres>>,
    product_id: web::Path<String>,
) -> impl Responder {
    let testimonials = testimonial::get_testimonials_by_product(&pool, &product_id).await;

    // Error check
    if testimonials.is_err() {
        return HttpResponse::InternalServerError().json("Internal Server Error");
    }

    // Parse to JSON
    if let Ok(testimonials) = testimonials {
        return HttpResponse::Ok().json(testimonials);
    }

    HttpResponse::InternalServerError().json("Internal Server Error")
}
