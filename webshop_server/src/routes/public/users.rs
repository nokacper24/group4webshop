use crate::data_access::user::{get_all_users, get_user_by_id};
use actix_web::{get, web, HttpResponse, Responder};
use sqlx::{Pool, Postgres};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(users);
    cfg.service(user_by_id);
}

#[get("/users")]
async fn users(pool: web::Data<Pool<Postgres>>) -> impl Responder {
    let users = get_all_users(&pool).await;

    //error check
    if users.is_err() {
        return HttpResponse::InternalServerError().json("Internal Server Error");
    }

    //parse to json
    if let Ok(users) = users {
        return HttpResponse::Ok().json(users);
    }

    HttpResponse::InternalServerError().json("Internal Server Error")
}

#[get("/users/{id}")]
async fn user_by_id(pool: web::Data<Pool<Postgres>>, id: web::Path<String>) -> impl Responder {
    let id = match id.parse::<i32>() {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().json("Bad Request"),
        
    };
    let user = get_user_by_id(&pool, id).await;

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
