use actix_web::{web, Responder, HttpResponse, get, HttpRequest};
use log::error;
use sqlx::{Postgres, Pool};

use crate::middlewares::auth::{validate_user, AuthError};

pub fn configure(cfg: &mut web::ServiceConfig) {

    cfg.service(me);

}

#[derive(serde::Serialize)]
struct MeUser {
    email: String,
    role: String,
    company_id: i32
}

impl From<crate::data_access::user::User> for MeUser {
    fn from(user: crate::data_access::user::User) -> Self {
        Self {
            email: user.email,
            role: user.role.to_string(),
            company_id: user.company_id
        }
    }
}

#[get("/me")]
pub async fn me(pool: web::Data<Pool<Postgres>>, req: HttpRequest) -> impl Responder {

    let user = validate_user(req, &pool).await;
    match user {
        Ok(user) => HttpResponse::Ok().json(MeUser::from(user)),
        Err(e) => match e {
            AuthError::Unauthorized => HttpResponse::Unauthorized().finish(),
            AuthError::SqlxError(e) => {
                error!("{}", e);
                HttpResponse::InternalServerError().finish()
            },
        }
    }
}