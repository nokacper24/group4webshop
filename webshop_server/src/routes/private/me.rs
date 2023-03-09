use actix_web::{web::{self, ReqData}, Responder, HttpResponse, get};
use sqlx::{Postgres, Pool};

use crate::middlewares::auth::{get_user_from_token, Token};

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
pub async fn me(pool: web::Data<Pool<Postgres>>, req: Option<ReqData<Token>>) -> impl Responder {

    let user = get_user_from_token(req, &pool).await;
    match user {
        Ok(user) => HttpResponse::Ok().json(MeUser::from(user)),
        Err(e) => match e {
            crate::middlewares::auth::AuthError::SqlxError(_) => HttpResponse::InternalServerError().finish(),
            crate::middlewares::auth::AuthError::BadToken => HttpResponse::Unauthorized().finish(), }
            
        }
    }
