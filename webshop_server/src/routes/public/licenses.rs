use crate::data_access::license::{self, License};

use actix_web::{get, web, HttpResponse, Responder};
use sqlx::{Pool, Postgres};
use utoipa::OpenApi;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(licenses);
    cfg.service(licenses_vital);
    cfg.service(license_by_id);
    cfg.service(licenses_by_company);
}

#[derive(OpenApi)]
#[openapi(
    paths(
        licenses,
        licenses_vital,
        license_by_id,
        licenses_by_company
    ),
    components(
        schemas(License)
    ),
    tags(
        (name = "Licenses", description = "API endpoints for licenses")
    ),
)]
pub struct LicensesOpenApi;

/// Get all licenses.
#[utoipa::path(
    context_path = "/api",
    get,
    responses(
    (status = 200, description = "List of all licenses", body = Vec<License>),
    (status = 500, description = "Internal Server Error"),
)
)]
#[get("/licenses")]
pub async fn licenses(pool: web::Data<Pool<Postgres>>) -> impl Responder {
    let licenses = license::get_licenses(&pool).await;

    // Error check
    if licenses.is_err() {
        return HttpResponse::InternalServerError().json("Internal Server Error");
    }

    // Parse to JSON
    if let Ok(licenses) = licenses {
        return HttpResponse::Ok().json(licenses);
    }

    HttpResponse::InternalServerError().json("Internal Server Error")
}

/// Get the most vital information about a license.
#[utoipa::path(
    context_path = "/api",
    get,
    responses(
    (status = 200, description = "List of all licenses with only their vital information", body = Vec<License>),
    (status = 500, description = "Internal Server Error"),
)
)]
#[get("/licenses_vital")]
pub async fn licenses_vital(pool: web::Data<Pool<Postgres>>) -> impl Responder {
    let other_licenses = license::get_licenses_vital_info(&pool).await;

    // Error check
    if other_licenses.is_err() {
        return HttpResponse::InternalServerError().json("Internal Server Error");
    }

    // Parse to JSON
    if let Ok(other_licenses) = other_licenses {
        return HttpResponse::Ok().json(other_licenses);
    }

    HttpResponse::InternalServerError().json("Internal Server Error")
}

/// Get a specific license by ID.
#[utoipa::path (
    context_path = "/api",
    get,
    responses(
        (status = 200, description = "Returns a specific license", body = License),
        (status = 404, description = "License not found"),
        (status = 500, description = "Internal Server Error"),
        ),
    params(
        ("license_id", description = "The ID of the license"),
        )
    )
]
#[get("/licenses/{license_id}")]
async fn license_by_id(
    pool: web::Data<Pool<Postgres>>,
    license_id: web::Path<String>,
) -> impl Responder {
    let license_id = match license_id.parse::<i32>() {
        Ok(license_id) => license_id,
        Err(_) => return HttpResponse::BadRequest().json("Bad Request"),
    };
    let license = license::get_license_by_id(&pool, &license_id).await;

    // Error check
    if license.is_err() {
        return HttpResponse::InternalServerError().json("Internal Server Error");
    }

    // Parse to JSON
    if let Ok(license) = license {
        return HttpResponse::Ok().json(license);
    }

    HttpResponse::InternalServerError().json("Internal Server Error")
}

/// Get all licenses for a company.
#[utoipa::path (
    context_path = "/api",
    get,
    responses(
        (status = 200, description = "Returns all licenses for a specific company", body = Vec<License>),
        (status = 500, description = "Internal Server Error"),
        ),
    params(
        ("company_id", description = "The ID of the company"),
        )
    )
]
#[get("/companies/{company_id}/licenses")]
async fn licenses_by_company(
    pool: web::Data<Pool<Postgres>>,
    company_id: web::Path<String>,
) -> impl Responder {
    let company_id = match company_id.parse::<i32>() {
        Ok(company_id) => company_id,
        Err(_) => return HttpResponse::BadRequest().json("Bad Request"),
    };
    let license = license::get_licenses_by_company(&pool, &company_id).await;

    // Error check
    if license.is_err() {
        return HttpResponse::InternalServerError().json("Internal Server Error");
    }

    // Parse to JSON
    if let Ok(license) = license {
        return HttpResponse::Ok().json(license);
    }

    HttpResponse::InternalServerError().json("Internal Server Error")
}
