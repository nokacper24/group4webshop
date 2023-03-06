use crate::{
    data_access::{
        auth,
        user::{self, create_invite, create_partial_user, LicenseUser, User},
    },
    middlewares::auth::{check_role, Token},
};
use actix_web::{
    delete, get, post,
    web::{self, ReqData},
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use utoipa::OpenApi;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(users);
    cfg.service(user_by_id);
    cfg.service(users_by_company);
    cfg.service(users_by_license);

    cfg.service(generate_invite);

    cfg.service(remove_license_users);
}

#[derive(OpenApi)]
#[openapi(
    paths(
        users,
    ),
    components(
        schemas(User)
    ),
    tags(
        (name = "Users", description = "API endpoints for users")
    ),
)]
pub struct UserApiDoc;

#[utoipa::path(
    context_path = "/api",
    responses(
    (status = 200, description = "List of all available users", body = Vec<User>),
    (status = 500, description = "Internal Server Error"),
    )
)]
#[get("/users")]
async fn users(pool: web::Data<Pool<Postgres>>) -> impl Responder {
    let users = user::get_all_users(&pool).await;

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
    let user = user::get_user_by_id(&pool, id).await;

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

/// Get all the users that work for a specific company.
#[get("/companies/{company_id}/users")]
async fn users_by_company(
    pool: web::Data<Pool<Postgres>>,
    company_id: web::Path<String>,
) -> impl Responder {
    let company_id = match company_id.parse::<i32>() {
        Ok(company_id) => company_id,
        Err(_) => return HttpResponse::BadRequest().json("Bad Request"),
    };
    let other_users = user::get_users_by_company(&pool, &company_id).await;

    // Error check
    if other_users.is_err() {
        return HttpResponse::InternalServerError().json("Internal Server Error");
    }

    // Parse to JSON
    if let Ok(other_users) = other_users {
        return HttpResponse::Ok().json(other_users);
    }

    HttpResponse::InternalServerError().json("Internal Server Error")
}

/// Get all the users that have access to a specific license.
#[get("/licenses/{license_id}/users")]
async fn users_by_license(
    pool: web::Data<Pool<Postgres>>,
    license_id: web::Path<String>,
) -> impl Responder {
    let license_id = match license_id.parse::<i32>() {
        Ok(license_id) => license_id,
        Err(_) => return HttpResponse::BadRequest().json("Bad Request"),
    };
    let other_users = user::get_users_by_license(&pool, &license_id).await;

    // Error check
    if other_users.is_err() {
        return HttpResponse::InternalServerError().json("Internal Server Error");
    }

    // Parse to JSON
    if let Ok(other_users) = other_users {
        return HttpResponse::Ok().json(other_users);
    }

    HttpResponse::InternalServerError().json("Internal Server Error")
}

#[derive(Deserialize, Serialize)]
struct Invite {
    email: String,
}

#[post("/generate_invite")]
async fn generate_invite(
    pool: web::Data<Pool<Postgres>>,
    invite: web::Json<Invite>,
    req: Option<ReqData<Token>>,
) -> impl Responder {
    let role = check_role(req, &pool).await;

    match role {
        Ok(role) => match role {
            user::Role::Admin => {
                return HttpResponse::Ok().json("Can't generate a new user for proflex yet!");
            }
            user::Role::CompanyItHead => {
                return HttpResponse::Ok().json("Can't generate a new user for proflex yet!");
            }
            user::Role::CompanyIt => {
                return HttpResponse::Ok().json("Can't generate a new user for proflex yet!");
            }
            user::Role::Default => {
                return HttpResponse::Unauthorized().json("Normal users don't have permission to generate a new user, please contact your company's IT department.");
            }
        },
        Err(_) => {
            // since the user is not logged in, we can't check the role, so it must be a new user
            // so we can generate a new user and a company for them
            let partial_user = create_partial_user(&invite.email, &pool.clone()).await;
            match partial_user {
                Ok(partial_user) => {
                    let invite = create_invite(Some(partial_user.id), None, &pool).await;
                    match invite {
                        Ok(invite) => {
                            return HttpResponse::Ok().json(invite);
                        }
                        Err(_) => {
                            return HttpResponse::InternalServerError()
                                .json("Internal Server Error");
                        }
                    }
                }
                Err(_) => {
                    return HttpResponse::InternalServerError().json("Internal Server Error");
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
struct LicenseUsers {
    users: Vec<LicenseUser>,
}

/// Add rows into the user license table.
/// The JSON for `other_users` can be like this:
/// ```
/// {
///     "users": [
///         {
///             "user_id": 1,
///             "license_id": 1
///         }
///     ]
/// }
/// ```
#[post("/license_users")]
async fn add_license_users(
    pool: web::Data<Pool<Postgres>>,
    other_users: web::Json<LicenseUsers>,
) -> impl Responder {
    let other_users = &other_users.users;
    match user::add_license_users(&pool, &other_users).await {
        Ok(_) => HttpResponse::Created().json(other_users),

        Err(e) => match e {
            sqlx::Error::Database(e) => match e.code() {
                Some(e) => match e.as_ref() {
                    "23505" => HttpResponse::BadRequest().json("Record already exists"),
                    _ => HttpResponse::InternalServerError().json("Internal Server Error"),
                },
                _ => HttpResponse::InternalServerError().json("Internal Server Error"),
            },
            _ => HttpResponse::InternalServerError().json("Internal Server Error"),
        },
    }
}

/// Delete from into the user license table.
/// The JSON for `other_users` can be like this:
/// ```
/// {
///     "users": [
///         {
///             "user_id": 1,
///             "license_id": 1
///         }
///     ]
/// }
/// ```
#[delete("/license_users")]
async fn remove_license_users(
    pool: web::Data<Pool<Postgres>>,
    other_users: web::Json<LicenseUsers>,
) -> impl Responder {
    let other_users = &other_users.users;
    match user::remove_license_users(&pool, &other_users).await {
        Ok(_) => HttpResponse::Ok().json(other_users),
        Err(e) => match e {
            _ => HttpResponse::InternalServerError().json("Internal Server Error"),
        },
    }
}
