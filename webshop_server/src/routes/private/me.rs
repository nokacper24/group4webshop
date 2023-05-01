use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use log::error;
use sqlx::{Pool, Postgres};

use crate::{utils::auth::{validate_user, AuthError}, SharedData};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(me);
}

#[derive(serde::Serialize)]
struct MeUser {
    user_id: i32,
    email: String,
    role: String,
    company_id: i32,
}

impl From<crate::data_access::user::User> for MeUser {
    fn from(user: crate::data_access::user::User) -> Self {
        Self {
            user_id: user.user_id,
            email: user.email,
            role: user.role.to_string(),
            company_id: user.company_id,
        }
    }
}

#[get("/me")]
async fn me(shared_data: web::Data<SharedData>, req: HttpRequest) -> impl Responder {
    let pool = &shared_data.db_pool;
    let user = validate_user(req, &pool).await;
    match user {
        Ok(user) => HttpResponse::Ok().json(MeUser::from(user)),
        Err(e) => match e {
            AuthError::Unauthorized => HttpResponse::Unauthorized().finish(),
            AuthError::SqlxError(e) => {
                error!("{}", e);
                HttpResponse::InternalServerError().finish()
            }
        },
    }
}
