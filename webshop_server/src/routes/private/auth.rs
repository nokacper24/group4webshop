use actix_web::{post, Responder, web, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::data_access::{user::get_user_by_username, auth::create_cookie};

pub fn configure(cfg: &mut web::ServiceConfig) {

    cfg.service(logout);

}


#[post("/logout")]
async fn logout( pool: web::Data<Pool<Postgres>>) -> impl Responder {
    "Hello world!"
}
