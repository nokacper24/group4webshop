use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{Pool, Postgres};

use crate::data_access::{
    self,
    auth::create_cookie,
    user::{create_invite, get_user_by_username},
};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
    cfg.service(create_user);
    cfg.service(valid_verify);
    cfg.service(verify);
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
            //check if password is correct TODO: use hash verify function
            if v.pass_hash != user.password {
                return HttpResponse::Unauthorized().json(
                    json!({"success": false, "message": "Incorrect username or password"}),
                );
            }

            let cookie_string = create_cookie(&pool, &v.user_id).await;
            match cookie_string {
                Ok(v) => {
                    // set cookie
                    let cookie = actix_web::cookie::Cookie::build("Bearer", v)
                        .path("/")
                        .domain("localhost:5173")
                        .domain("127.0.0.1:8089")
                        .domain("localhost:8089")
                        .domain("127.0.0.1:5173")
                        .secure(true)
                        .http_only(true)
                        .expires(None)
                        .finish();
                    return HttpResponse::Ok().cookie(cookie).json(json!(
                        {"success": true, "message": "Login successful"}
                    ));
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

#[derive(Deserialize, Serialize)]
struct AddUserData {
    password: String,
    company_id: Option<i32>,
    company_name: Option<String>,
    company_address: Option<String>,
}

#[get("/verify/{invite_id}")]
async fn valid_verify(
    invite_id: web::Path<String>,
    pool: web::Data<Pool<Postgres>>,
) -> impl Responder {
    let invite = data_access::user::get_invite(&invite_id, &pool).await;
    match invite {
        Ok(v) => HttpResponse::Ok().json(v),
        Err(_e) => {
            return HttpResponse::InternalServerError().json("Internal Server Error");
        }
    }
}

#[post("/verify/{invite_id}")]
async fn verify(
    invite_id: web::Path<String>,
    data: web::Json<AddUserData>,
    pool: web::Data<Pool<Postgres>>,
) -> impl Responder {
    let invite = data_access::user::get_invite(&invite_id, &pool).await;
    match invite {
        Ok(v) => {
            // check if invite has company
            if v.company_user_id.is_some() {
                // check if company exists

                let id = match v.company_user_id {
                    Some(id) => id,
                    None => {
                        return HttpResponse::InternalServerError().json("Internal Server Error")
                    }
                };

                let partial_company_user =
                    data_access::user::get_partial_company_user(&id, &pool).await;
                match partial_company_user {
                    Ok(v) => {
                        // get company
                        let company =
                            data_access::company::get_company_by_id(&pool, &v.company_id).await;

                        match company {
                            Ok(c) => {
                                //create user with company
                                let user = data_access::user::create_user(
                                    &v.email,
                                    &data.password,
                                    c.company_id,
                                    data_access::user::Role::Default,
                                    &pool,
                                )
                                .await;

                                match user {
                                    Ok(_v) => {
                                        // delete invite
                                        let delete =
                                            data_access::user::delete_invite(&invite_id, &pool)
                                                .await;
                                        match delete {
                                            Ok(_v) => {
                                                return HttpResponse::Ok().json("User created");
                                            }
                                            Err(_e) => {
                                                return HttpResponse::InternalServerError()
                                                    .json("Internal Server Error")
                                            }
                                        }
                                    }
                                    Err(_e) => {
                                        return HttpResponse::InternalServerError()
                                            .json("Internal Server Error")
                                    }
                                }
                            }
                            Err(_e) => {
                                return HttpResponse::InternalServerError()
                                    .json("Internal Server Error")
                            }
                        }
                    }
                    Err(_e) => {
                        return HttpResponse::InternalServerError().json("Internal Server Error")
                    }
                }
            }
            // since no company is set, create a new company
            match (&data.company_name, &data.company_address) {
                (Some(name), Some(address)) => {
                    // create company
                    let company =
                        data_access::company::create_company(&pool, &name, &address).await;
                    match company {
                        Ok(c) => {
                            // create user with company from partial user
                            if let Some(id) = v.user_id {
                                let partial_user =
                                    data_access::user::get_partial_user(&id, &pool).await;
                                match partial_user {
                                    Ok(v) => {
                                        // create user with company
                                        let user = data_access::user::create_user(
                                            &v.email,
                                            &data.password,
                                            c.company_id,
                                            data_access::user::Role::Default,
                                            &pool,
                                        )
                                        .await;

                                        match user {
                                            Ok(_v) => {
                                                return HttpResponse::Ok().json("User created")
                                            }
                                            Err(_e) => {
                                                return HttpResponse::InternalServerError()
                                                    .json("Internal Server Error")
                                            }
                                        }
                                    }
                                    Err(_e) => {
                                        return HttpResponse::InternalServerError()
                                            .json("Internal Server Error")
                                    }
                                }
                            }
                        }
                        Err(_e) => {
                            return HttpResponse::InternalServerError()
                                .json("Internal Server Error")
                        }
                    }
                },
                (None, None) => {
                    return HttpResponse::BadRequest()
                        .json("Company name and address are required")
                }
                (None, _) => {
                    return HttpResponse::BadRequest().json("Company name is required")
                }
                (_, None) => {
                    return HttpResponse::BadRequest().json("Company address is required")
                }


            }
            return HttpResponse::InternalServerError().json("Internal Server Error")
        }

        Err(_e) => {
            return HttpResponse::InternalServerError().json("Internal Server Error");
        }
    }
}

