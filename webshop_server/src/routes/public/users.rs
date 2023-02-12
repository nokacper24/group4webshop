use crate::data_access::user::{get_all_users, get_user_by_id};
use actix_web::{get, post, web, HttpResponse, Responder};
use http::StatusCode;
use sqlx::{Pool, Postgres};

#[get("/users")]
async fn get_all_users(pool: web::Data<Pool<Postgres>>) -> impl Responder {
    return get_all_users(&pool);
}

#[get("/users/{id}")]
async fn user_by_id(pool: web::Data<Pool<Postgres>>, id: web::Path<String>) -> impl Responder {
    let user = get_user_by_id(&pool, product_name.as_str()).await;

    //error check
    if user.is_err() {
        return HttpResponse::InternalServerError().json("Internal Server Error");
    }

    //parse to json
    if let Ok(user) = user {
        return HttpResponse::Ok().json(user);
    }

    HttpResponse::InternalServerError().json("Internal Server Error")
}
