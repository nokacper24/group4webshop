use actix_web::{cookie::time::Duration, get, post, web, HttpRequest, HttpResponse, Responder};
use log::error;

use sqlx::{Pool, Postgres};

use crate::{
    data_access::auth::delete_cookie,
    utils::auth::{self, AuthError, COOKIE_KEY_SECRET},
};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(logout);
    cfg.service(logged_in);
}

#[post("/logout")]
async fn logout(pool: web::Data<Pool<Postgres>>, req: HttpRequest) -> impl Responder {
    let cookie = match auth::extract_valid_cookie(req, &pool).await {
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

    let cookie = delete_cookie(&pool, &cookie).await;
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

#[get("/logged_in")]
async fn logged_in(req: HttpRequest, pool: web::Data<Pool<Postgres>>) -> impl Responder {
    let valid = auth::validate_user(req, &pool).await;
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
