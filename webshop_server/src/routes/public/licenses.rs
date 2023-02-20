use actix_web::{get, web, HttpResponse, Responder};
use sqlx::{Pool, Postgres};

use crate::data_access::license::{get_license_by_id, get_licenses};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(licenses);
    cfg.service(license_by_id);
}

/// Get all licenses.
#[get("licenses")]
pub async fn licenses(pool: web::Data<Pool<Postgres>>) -> impl Responder {
    let licenses = get_licenses(&pool).await;

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

/// Get a specific license by ID.
#[get("licenses/{license_id}")]
async fn license_by_id(
    pool: web::Data<Pool<Postgres>>,
    license_id: web::Path<i32>,
) -> impl Responder {
    let license_id = license_id.into_inner();
    let license = get_license_by_id(&pool, &license_id).await;

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
