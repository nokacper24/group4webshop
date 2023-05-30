use crate::{
    data_access::{
        license::{self, License, LicenseValidation, PartialLicense},
        user::{self, Role},
    },
    utils::auth,
    SharedData,
};

use actix_web::{get, patch, post, web, HttpRequest, HttpResponse, Responder};
use log::error;
use serde::{Deserialize, Serialize};
use utoipa::OpenApi;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(licenses);
    cfg.service(licenses_full);
    cfg.service(license_by_id);
    cfg.service(licenses_full_by_company);
    cfg.service(licenses_for_user);
    cfg.service(licenses_for_user_no_access);
    cfg.service(create_license);
    cfg.service(update_license_validations);
}

#[derive(OpenApi)]
#[openapi(
    paths(
        licenses,
        licenses_full,
        license_by_id,
        licenses_full_by_company,
        licenses_for_user,
        licenses_for_user_no_access,
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

/// Get all licenses.
#[utoipa::path(
    context_path = "/api",
    get,
    tag = "Licenses",
    responses(
    (status = 200, description = "List of all licenses", body = Vec<License>),
    (status = 500, description = "Internal Server Error"),
)
)]
#[get("/licenses")]
async fn licenses(shared_data: web::Data<SharedData>) -> impl Responder {
    let pool = &shared_data.db_pool;
    let licenses = license::get_licenses(pool).await;

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
    tag = "Licenses",
    responses(
    (status = 200, description = "List of all licenses with only their vital information", body = Vec<License>),
    (status = 500, description = "Internal Server Error"),
)
)]
#[get("/licenses_full")]
async fn licenses_full(shared_data: web::Data<SharedData>) -> impl Responder {
    let pool = &shared_data.db_pool;
    let other_licenses = license::get_licenses_full(pool).await;

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
    tag = "Licenses",
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
    shared_data: web::Data<SharedData>,
    license_id: web::Path<String>,
) -> impl Responder {
    let pool = &shared_data.db_pool;
    let license_id = match license_id.parse::<i32>() {
        Ok(license_id) => license_id,
        Err(_) => return HttpResponse::BadRequest().json("Bad Request"),
    };
    let license = license::get_license_by_id(pool, &license_id).await;

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
    tag = "Licenses",
    responses(
        (status = 200, description = "Returns all licenses for a specific company", body = Vec<License>),
        (status = 400, description = "Bad Request"),
        (status = 500, description = "Internal Server Error"),
        ),
    params(
        ("company_id", description = "The ID of the company"),
        )
    )
]
#[get("/companies/{company_id}/licenses_full")]
async fn licenses_full_by_company(
    shared_data: web::Data<SharedData>,
    company_id: web::Path<String>,
) -> impl Responder {
    let pool = &shared_data.db_pool;
    let company_id = match company_id.parse::<i32>() {
        Ok(company_id) => company_id,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    let license = license::get_licenses_full_by_company(pool, &company_id).await;

    // Error check
    if license.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    // Parse to JSON
    if let Ok(license) = license {
        return HttpResponse::Ok().json(license);
    }

    HttpResponse::InternalServerError().finish()
}

/// Get all licenses for a user.
#[utoipa::path (
    context_path = "/api",
    get,
    tag = "Licenses",
    responses(
        (status = 200, description = "Returns all licenses for a specific user", body = Vec<License>),
        (status = 400, description = "Bad Request"),
        (status = 500, description = "Internal Server Error"),
        ),
    params(
        ("user_id", description = "The ID of the user"),
        )
    )
]
#[get("/user_licenses/user/{user_id}")]
async fn licenses_for_user(
    shared_data: web::Data<SharedData>,
    user_id: web::Path<String>,
) -> impl Responder {
    let pool = &shared_data.db_pool;
    let user_id = match user_id.parse::<i32>() {
        Ok(user_id) => user_id,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    let other_licenses = license::get_licenses_for_user(pool, &user_id).await;

    // Error check
    if other_licenses.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    // Parse to JSON
    if let Ok(other_licenses) = other_licenses {
        return HttpResponse::Ok().json(other_licenses);
    }

    HttpResponse::InternalServerError().finish()
}

/// Get all licenses for user's company's that the user does not have access to.
#[utoipa::path (
    context_path = "/api",
    get,
    tag = "Licenses",
    responses(
        (status = 200, description = "Returns all company licenses that a specific user does not have access to", body = Vec<License>),
        (status = 400, description = "Bad Request"),
        (status = 500, description = "Internal Server Error"),
        ),
    params(
        ("user_id", description = "The ID of the user"),
        )
    )
]
#[get("/user_licenses/user/{user_id}/no_access")]
async fn licenses_for_user_no_access(
    shared_data: web::Data<SharedData>,
    user_id: web::Path<String>,
) -> impl Responder {
    let pool = &shared_data.db_pool;
    let user_id = match user_id.parse::<i32>() {
        Ok(user_id) => user_id,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    let company_id = match user::get_user_by_id(pool, &user_id).await {
        Ok(user) => user.company_id,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    let other_licenses =
        license::get_licenses_for_user_no_access(pool, &company_id, &user_id).await;

    // Error check
    if other_licenses.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    // Parse to JSON
    if let Ok(other_licenses) = other_licenses {
        return HttpResponse::Ok().json(other_licenses);
    }

    HttpResponse::InternalServerError().finish()
}

/// Create a license.
///
/// An admin can create a license for any company.
/// Default user cannot create a license.
/// CompanyIt or CompanyItHead can create a license for their company.
#[utoipa::path (
    context_path = "/api/priv",
    post,
    responses (
        (status = 201, description = "License created", body = License),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 500, description = "Internal Server Error"),
      ),
    request_body(
        description = "The license to create",
        content = PartialLicense,
    ),
  )
]
#[post("/licenses")]
async fn create_license(
    shared_data: web::Data<SharedData>,
    license: web::Json<PartialLicense>,
    req: HttpRequest,
) -> impl Responder {
    let pool = &shared_data.db_pool;
    let license = license.into_inner();
    let user = match auth::validate_user(req, pool).await {
        Ok(user) => user,
        Err(e) => {
            return match e {
                auth::AuthError::Unauthorized => HttpResponse::Unauthorized().finish(),
                auth::AuthError::SqlxError(e) => {
                    error!("{}", e);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
    };
    match user.role {
        Role::Default => {
            return HttpResponse::Forbidden().body("Cannot create licenses as a default user")
        }
        Role::CompanyIt | Role::CompanyItHead => {
            if user.company_id != license.company_id() {
                return HttpResponse::Forbidden()
                    .body("Cannot create licenses for other companies");
            }
        }
        Role::Admin => (),
    }

    match license::create_license(pool, &license).await {
        Ok(new_license) => HttpResponse::Created().json(new_license),
        Err(e) => {
            error!("{}", e);
            HttpResponse::InternalServerError().json("Internal Server Error")
        }
    }
}

#[derive(Serialize, Deserialize)]
struct LicenseValidations {
    licenses: Vec<LicenseValidation>,
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
    shared_data: web::Data<SharedData>,
    other_licenses: web::Json<LicenseValidations>,
) -> impl Responder {
    let pool = &shared_data.db_pool;
    let other_licenses = &other_licenses.licenses;
    match license::update_license_validations(pool, other_licenses).await {
        Ok(_) => HttpResponse::Ok().json(other_licenses),
        Err(_e) => HttpResponse::InternalServerError().json("Internal Server Error"),
    }
}
