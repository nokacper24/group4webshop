use actix_web::{post, Responder, web::{self, ReqData}, HttpResponse, cookie::time::Duration, get, HttpRequest};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::{data_access::{user::get_user_by_username, auth::{create_cookie, delete_cookie}}, middlewares::auth::{Token, check_auth, validate_user, COOKIE_KEY_SECRET}};

pub fn configure(cfg: &mut web::ServiceConfig) {

    cfg.service(logout);
    cfg.service(logged_in);

}


#[post("/logout")]
async fn logout( pool: web::Data<Pool<Postgres>>, req: Option<ReqData<Token>>) -> impl Responder {
    let token = match req {
        Some(token) => token.into_inner(),
        None => return HttpResponse::Unauthorized().finish()
    };
        // delete cookie from db
    let token = token.token;
    
    let cookie = delete_cookie(&pool, &token).await;
    match cookie {
        Ok(_) => {
            HttpResponse::Ok().cookie(
                actix_web::cookie::Cookie::build(COOKIE_KEY_SECRET, "")
                    .path("/")
                    .max_age(Duration::seconds(0))
                    .finish()
            ).finish()
        },
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}
            
#[get("/logged_in")]
async fn logged_in(req: HttpRequest, pool: web::Data<Pool<Postgres>>) -> impl Responder {
   let valid = validate_user(req, &pool).await;
    match valid {
         Ok(_) => HttpResponse::Ok().finish(),
         Err(e) => match e {
            InvalidCookie => HttpResponse::Unauthorized().finish(),
            _ => HttpResponse::InternalServerError().finish()
         }
    }        
}