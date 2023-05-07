use crate::{
    data_access::testimonial::{self, Testimonial},
    SharedData,
};

use actix_web::{get, web, HttpResponse, Responder};
use log::error;
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
  tag = "Testimonials",
  responses(
  (status = 200, description = "List of all testimonials for a product", body = Vec<Testimonial>),
  (status = 500, description = "Internal Server Error"),
),
  params(
    ("product_id", description = "The id of the product"),
  ),
)]
#[get("/testimonials/{product_id}")]
async fn get_testimonials_by_product(
    shared_data: web::Data<SharedData>,
    product_id: web::Path<String>,
) -> impl Responder {
    let pool = &shared_data.db_pool;
    let testimonials = testimonial::get_testimonials_by_product(&pool, &product_id).await;
    match testimonials {
        Ok(testimonials) => HttpResponse::Ok().json(testimonials),
        Err(e) => {
            error!("Error getting testimonials: {}", e);
            HttpResponse::InternalServerError().json("Internal Server Error")
        }
    }
}
