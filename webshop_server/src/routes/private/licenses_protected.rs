use crate::data_access::license::{self, InvalidLicense, License, PartialLicense};

use actix_web::{get, patch, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use utoipa::OpenApi;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(create_license);
    cfg.service(update_license_validations);
}

#[derive(OpenApi)]
#[openapi(
    paths(
        create_license,
        update_license_validations
    ),
    components(
        schemas(License)
    ),
    tags(
        (name = "Licenses", description = "API endpoints for licenses")
    ),
)]
pub struct ProtectedLicensesOpenApi;

/// Create a license.
#[utoipa::path (
  context_path = "/api/priv",
  post,
  responses(
      (status = 201, description = "License has been created", body = License),
      (status = 500, description = "Internal Server Error"),
      ),
  )
]
#[post("/licenses")]
async fn create_license(
    pool: web::Data<Pool<Postgres>>,
    license: web::Json<PartialLicense>,
) -> impl Responder {
    match license::create_license(&pool, &license).await {
        Ok(_) => HttpResponse::Created().json(license),
        Err(_) => HttpResponse::InternalServerError().json("Internal Server Error"),
    }
}

#[derive(Serialize, Deserialize)]
struct InvalidLicenses {
    licenses: Vec<InvalidLicense>,
}

/// Update the validation of licenses.
#[utoipa::path (
  context_path = "/api/priv",
  patch,
  responses(
      (status = 200, description = "Licenses' validity have been updated", body = Vec<InvalidLicense>),
      (status = 500, description = "Internal Server Error"),
      ),
  )
]
#[patch("/licenses")]
async fn update_license_validations(
    pool: web::Data<Pool<Postgres>>,
    other_licenses: web::Json<InvalidLicenses>,
) -> impl Responder {
    let other_licenses = &other_licenses.licenses;
    match license::update_license_validations(&pool, &other_licenses).await {
        Ok(_) => HttpResponse::Ok().json(other_licenses),
        Err(e) => match e {
            _ => HttpResponse::InternalServerError().json("Internal Server Error"),
        },
    }
}
