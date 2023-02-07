use crate::data_access::user::{get_all_users, get_user_by_id};
use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::{Pool, Postgres};
use http::StatusCode;

#[get("/users")]
async fn get_all_users(pool: web::Data<Pool<Postgres>>) -> impl Responder {
    return get_all_users(&pool);
}
