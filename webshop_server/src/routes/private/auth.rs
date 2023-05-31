use actix_web::{cookie::time::Duration, get, post, web, HttpRequest, HttpResponse, Responder};
use log::error;
use utoipa::OpenApi;

use crate::{
    data_access::auth::delete_cookie,
    utils::auth::{self, AuthError, COOKIE_KEY_SECRET},
    SharedData,
};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(logout);
    cfg.service(logged_in);
}

#[derive(OpenApi)]
#[openapi(
    paths(
        logout,
        logged_in
    ),

    tags(
        (name = "auth", description = "Authentication related routes")
    ),
)]
pub struct AuthApiDoc;

#[utoipa::path(
    context_path = "/api/priv",
    tag = "auth",
    responses(
        (status = 200, description = "User logged out"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    )
)]
#[post("/logout")]
async fn logout(shared_data: web::Data<SharedData>, req: HttpRequest) -> impl Responder {
    let pool = &shared_data.db_pool;
    let cookie = match auth::extract_valid_cookie(req, pool).await {
        Ok(token) => token,
        Err(e) => {
            return match e {
                AuthError::Unauthorized => HttpResponse::Unauthorized().finish(),
                AuthError::SqlxError(e) => {
                    error!("{}", e);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
    };

    let cookie = delete_cookie(pool, &cookie).await;
    match cookie {
        Ok(_) => HttpResponse::Ok()
            .cookie(
                actix_web::cookie::Cookie::build(COOKIE_KEY_SECRET, "")
                    .path("/")
                    .max_age(Duration::seconds(0))
                    .finish(),
            )
            .finish(),
        Err(e) => {
            error!("{}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[utoipa::path(
    context_path = "/api/priv",
    tag = "auth",
    responses(
        (status = 200, description = "User logged in"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    )
)]
#[get("/logged_in")]
async fn logged_in(req: HttpRequest, shared_data: web::Data<SharedData>) -> impl Responder {
    let pool = &shared_data.db_pool;
    let valid = auth::validate_user(req, pool).await;
    match valid {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => match e {
            AuthError::Unauthorized => HttpResponse::Unauthorized().finish(),
            AuthError::SqlxError(e) => {
                error!("{}", e);
                HttpResponse::InternalServerError().finish()
            }
        },
    }
}
