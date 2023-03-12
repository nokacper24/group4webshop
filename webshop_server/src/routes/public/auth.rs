use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::data_access::{
    self,
    auth::create_cookie,
    user::{create_invite, get_user_by_username},
};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
}

#[derive(Deserialize, Serialize)]
struct Login {
    email: String,
    password: String,
}

#[post("/login")]
async fn login(user: web::Json<Login>, pool: web::Data<Pool<Postgres>>) -> impl Responder {
    // check if user exists
    let db_user = get_user_by_username(&pool, &user.email).await;
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
                }
                Err(_e) => {
                    return HttpResponse::InternalServerError().json("Internal Server Error");
                }
            }
        }
        Err(_e) => {
            return HttpResponse::InternalServerError().json("Internal Server Error");
        }
    }
}

#[derive(Deserialize, Serialize)]
struct Email {
    email: String,
}

#[post("/create-user")]
async fn create_user(email: web::Json<Email>, pool: web::Data<Pool<Postgres>>) -> impl Responder {
    // check if user exists
    let db_user = get_user_by_username(&pool, &email.email).await;
    match db_user {
        Ok(_v) => {
            return HttpResponse::BadRequest().json("User already exists");
        }
        Err(_e) => {
            // create partial user
            let partial_user = data_access::user::create_partial_user(&email.email, &pool).await;
            match partial_user {
                Ok(v) => {
                    // create invite
                    let invite = create_invite(Some(v.id), None, &pool).await;
                    match invite {
                        Ok(_v) => {
                            //print invite temporarely TODO: send email
                            println!("Invite: {}", v.id);

                            return HttpResponse::Ok().json("Invite created, check your email");
                        }
                        Err(_e) => {
                            return HttpResponse::InternalServerError()
                                .json("Internal Server Error");
                        }
                    }
                }
                Err(_e) => {
                    return HttpResponse::InternalServerError().json("Internal Server Error");
                }
            }
        }
    }
}
