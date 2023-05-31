use actix_web::{get, web, HttpResponse, Responder};
use utoipa::OpenApi;

use crate::{
    data_access::category::{get_categories, get_category_by_id},
    SharedData,
};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(categories);
    cfg.service(category_by_id);
}

#[derive(OpenApi)]
#[openapi(
    paths(
        categories,
        category_by_id
    ),
    tags(
        (name = "Category", description = "API endpoints for categories")
    ),
)]
pub struct CategoryOpenApi;

/// Get all categories.
#[utoipa::path(
    context_path = "/api",
    get,
    tag = "Category",
    responses(
        (status = 200, description = "List of all categories", body = Vec<crate::data_access::category::Category>),
        (status = 500, description = "Internal Server Error"),
    ),
)]
#[get("/categories")]
async fn categories(shared_data: web::Data<SharedData>) -> impl Responder {
    let pool = &shared_data.db_pool;
    let categories = get_categories(pool).await;

    // Error check
    if categories.is_err() {
        return HttpResponse::InternalServerError().json("Internal Server Error");
    }

    // Parse to JSON
    if let Ok(categories) = categories {
        return HttpResponse::Ok().json(categories);
    }

    HttpResponse::InternalServerError().json("Internal Server Error")
}

/// Get a specific category by ID.
#[utoipa::path(
    context_path = "/api",
    get,
    tag = "Category",
    responses(
        (status = 200, description = "Category", body = crate::data_access::category::Category),
        (status = 500, description = "Internal Server Error"),
    ),
    params(
        ("category_id", description = "The id of the category"),
    ),
)]
#[get("/categories/{category_id}")]
async fn category_by_id(
    shared_data: web::Data<SharedData>,
    category_id: web::Path<i32>,
) -> impl Responder {
    let pool = &shared_data.db_pool;
    let category_id = category_id.into_inner();
    let category = get_category_by_id(pool, &category_id).await;

    // Error check
    if category.is_err() {
        return HttpResponse::InternalServerError().json("Internal Server Error");
    }

    // Parse to JSON
    if let Ok(category) = category {
        return HttpResponse::Ok().json(category);
    }

    HttpResponse::InternalServerError().json("Internal Server Error")
}
