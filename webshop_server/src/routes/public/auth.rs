use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::{OpenApi, ToSchema};

use crate::{
    data_access::{
        self,
        auth::create_cookie,
        user::{create_invite, get_by_username_with_pass},
    },
    utils::{self, auth::COOKIE_KEY_SECRET},
    SharedData,
};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
    cfg.service(create_user);
    cfg.service(valid_verify);
    cfg.service(verify);
}

#[derive(OpenApi)]
#[openapi(
    paths(
        login,
        create_user,
        valid_verify,
        verify
    ),
    tags(
        (name = "Auth", description = "API endpoints for authentication")
    ),
)]
pub struct AuthOpenApi;

#[derive(Deserialize, Serialize, ToSchema)]
struct Login {
    email: String,
    password: String,
}

#[utoipa::path(
    context_path = "/api",
    tag = "Auth",
    post,
    responses(
        (status = 200, description = "Login successful", body = Login),
        (status = 401, description = "Incorrect username or password"),
        (status = 500, description = "Internal Server Error")
    )
)]
#[post("/login")]
async fn login(user: web::Json<Login>, shared_data: web::Data<SharedData>) -> impl Responder {
    let pool = &shared_data.db_pool;
    // check if user exists
    let db_user = get_by_username_with_pass(pool, &user.email).await;
    match db_user {
        Ok(v) => {
            let hash = data_access::user::verify(&user.password, &v.pass_hash);
            match hash {
                Ok(hash) => {
                    if !hash {
                        return HttpResponse::Unauthorized().json(
                            json!({"success": false, "message": "Incorrect username or password"}),
                        );
                    }
                }
                Err(e) => {
                    log::error!("Error hashing password: {}", e);
                    return HttpResponse::InternalServerError().json("Internal Server Error");
                }
            }

            let cookie_string = create_cookie(pool, &v.user_id).await;
            match cookie_string {
                Ok(v) => {
                    // set cookie
                    let cookie = actix_web::cookie::Cookie::build(COOKIE_KEY_SECRET, v)
                        .path("/")
                        .secure(true)
                        .http_only(true)
                        .expires(None)
                        .finish();
                    return HttpResponse::Ok().cookie(cookie).json(json!(
                        {"success": true, "message": "Login successful"}
                    ));
                }
                Err(e) => {
                    log::error!("Error creating cookie: {}", e);
                    HttpResponse::InternalServerError().json("Internal Server Error")
                }
            }
        }
        Err(e) => match e {
            sqlx::Error::RowNotFound => HttpResponse::Unauthorized()
                .json(json!({"success": false, "message": "Incorrect username or password"})),
            _ => {
                log::error!("Error getting user: {}", e);
                HttpResponse::InternalServerError().json("Internal Server Error")
            }
        },
    }
}

#[derive(Deserialize, Serialize)]
struct Email {
    email: String,
}

#[utoipa::path(
    context_path = "/api",
    tag = "Auth",
    post,
    responses(
        (status = 200, description = "Invite created", body = Email),
        (status = 400, description = "User already exists"),
        (status = 500, description = "Internal Server Error")
    )
)]
#[post("/create-user")]
async fn create_user(
    email: web::Json<Email>,
    shared_data: web::Data<SharedData>,
) -> impl Responder {
    let pool = &shared_data.db_pool;
    let mailer = &shared_data.mailer;
    // check if user exists
    let db_user = get_by_username_with_pass(pool, &email.email).await;
    match db_user {
        Ok(_v) => HttpResponse::BadRequest().json("User already exists"),
        Err(_e) => {
            // create partial user
            let partial_user = data_access::user::create_partial_user(&email.email, pool).await;
            match partial_user {
                Ok(v) => {
                    // create invite
                    let invite = create_invite(Some(v.id), None, pool).await;
                    match invite {
                        Ok(_v) => {
                            //print invite temporarely TODO: send email
                            let email = utils::email::Email {
                                recipient_email: email.email.clone(),
                                mail_type: utils::email::EmailType::RegisterUser,
                                invite_code: Some(_v.id),
                            };
                            let outcome = utils::email::send_email(email, mailer).await;

                            match outcome {
                                Ok(_v) => {
                                    HttpResponse::Ok().json("Invite created, check your email")
                                }
                                Err(e) => {
                                    log::error!("Error sending email: {}", e);
                                    HttpResponse::InternalServerError()
                                        .json("Internal Server Error")
                                }
                            }
                        }
                        Err(_e) => {
                            HttpResponse::InternalServerError().json("Internal Server Error")
                        }
                    }
                }
                Err(_e) => HttpResponse::InternalServerError().json("Internal Server Error"),
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

#[utoipa::path(
    context_path = "/api",
    tag = "Auth",
    get,
    responses(
        (status = 200, description = "Invite is valid", body = AddUserData),
        (status = 404, description = "Invite not found"),
        (status = 500, description = "Internal Server Error")
    )
)]
#[get("/verify/{invite_id}")]
async fn valid_verify(
    invite_id: web::Path<String>,
    shared_data: web::Data<SharedData>,
) -> impl Responder {
    let pool = &shared_data.db_pool;
    let invite = data_access::user::get_invite_by_id(&invite_id, pool).await;
    match invite {
        Ok(v) => match v.company_user_id {
            Some(v) => {
                let partial_comp_user = data_access::user::get_partial_company_user(&v, pool).await;
                match partial_comp_user {
                    Ok(u) => {
                        //return both partial company user and info about company
                        let company =
                            data_access::company::get_company_by_id(pool, &u.company_id).await;
                        match company {
                            Ok(c) => {
                                let data = json!({
                                    "partial_company_user": u,
                                    "company": c
                                });
                                HttpResponse::Found().json(data)
                            }
                            Err(_e) => HttpResponse::NotFound().json("No company found"),
                        }
                    }
                    Err(_e) => HttpResponse::NotFound().json("No Partial user found"),
                }
            }
            None => match v.user_id {
                Some(v) => {
                    let partial_user = data_access::user::get_partial_user(&v, pool).await;
                    match partial_user {
                        Ok(u) => HttpResponse::Found().json(u),
                        Err(_e) => HttpResponse::NotFound().json("No Partial user found"),
                    }
                }
                None => HttpResponse::NotFound().json("No Partial user found"),
            },
        },
        Err(_e) => HttpResponse::InternalServerError().json("Internal Server Error"),
    }
}

#[utoipa::path(
    context_path = "/api",
    tag = "Auth",
    post,
    responses(
        (status = 200, description = "User created"),
        (status = 400, description = "Company name and address are required"),
        (status = 400, description = "Company name is required"),
        (status = 400, description = "Company address is required"),
        (status = 500, description = "Internal Server Error")
    )
)]
#[post("/verify/{invite_id}")]
async fn verify(
    invite_id: web::Path<String>,
    data: web::Json<AddUserData>,
    shared_data: web::Data<SharedData>,
) -> impl Responder {
    let pool = &shared_data.db_pool;
    let invite = data_access::user::get_invite_by_id(&invite_id, pool).await;
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
                    data_access::user::get_partial_company_user(&id, pool).await;
                match partial_company_user {
                    Ok(v) => {
                        // get company
                        let company =
                            data_access::company::get_company_by_id(pool, &v.company_id).await;

                        match company {
                            Ok(c) => {
                                //create user with company
                                let user = data_access::user::create_user(
                                    &v.email,
                                    &data.password,
                                    c.company_id,
                                    data_access::user::Role::Default,
                                    pool,
                                )
                                .await;

                                match user {
                                    Ok(_v) => {
                                        // delete invite
                                        let delete =
                                            data_access::user::delete_invite(&invite_id, pool)
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
                    let company = data_access::company::create_company(pool, name, address).await;
                    match company {
                        Ok(c) => {
                            // create user with company from partial user
                            if let Some(id) = v.user_id {
                                let partial_user =
                                    data_access::user::get_partial_user(&id, pool).await;
                                match partial_user {
                                    Ok(v) => {
                                        // create user with company
                                        let user = data_access::user::create_user(
                                            &v.email,
                                            &data.password,
                                            c.company_id,
                                            data_access::user::Role::CompanyItHead,
                                            pool,
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
                }
                (None, None) => {
                    return HttpResponse::BadRequest().json("Company name and address are required")
                }
                (None, _) => return HttpResponse::BadRequest().json("Company name is required"),
                (_, None) => return HttpResponse::BadRequest().json("Company address is required"),
            }
            HttpResponse::InternalServerError().json("Internal Server Error")
        }

        Err(_e) => HttpResponse::InternalServerError().json("Internal Server Error"),
    }
}
