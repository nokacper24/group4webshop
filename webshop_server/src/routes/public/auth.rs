use actix_web::{post, Responder, web, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::data_access::{user::get_user_by_username, auth::create_cookie};

pub fn configure(cfg: &mut web::ServiceConfig) {

    cfg.service(login);

}

#[derive(Deserialize, Serialize)]
struct Login {
    username: String,
    password: String,
}

#[post("/login")]
async fn login(user: web::Json<Login>, pool: web::Data<Pool<Postgres>>) -> impl Responder {
    // check if user exists
    let db_user = get_user_by_username(&pool, &user.username).await;
    match db_user {
        Ok(v) => {
            //check if password is correct
            if v.pass_hash != user.password {
                return HttpResponse::Unauthorized().json("Wrong password");
            }

            let cookie_string = create_cookie(&pool, &v.user_id).await;
            match cookie_string {
                Ok(v) => {
                    // set cookie
                    let cookie = actix_web::cookie::Cookie::build("Bearer", v)
                        .path("/")
                        .domain("localhost")
                        .secure(false)
                        .http_only(true)
                        .finish();
                    return HttpResponse::Ok().cookie(cookie).json("Logged in");
                },
                Err(_e) => {
                    return HttpResponse::InternalServerError().json("Internal Server Error");
                }
            }

        },
        Err(_e) => {
            return HttpResponse::InternalServerError().json("Internal Server Error");
        }
    }
}
