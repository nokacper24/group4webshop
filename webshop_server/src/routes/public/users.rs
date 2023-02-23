use actix_web::{delete, get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use utoipa::OpenApi;

use crate::data_access::user::{self, LicenseUser, User};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(users);
    cfg.service(user_by_id);
    cfg.service(users_by_company);
    cfg.service(users_by_license);
    cfg.service(add_license_users);
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
        Ok(other_users) => HttpResponse::Created().json(other_users),

        Err(e) => match e {
            sqlx::Error::Database(e) => match e.code() {
                Some(cow) => match cow.as_ref() {
                    "23505" => HttpResponse::BadRequest().json("Record already exists"),
                    _ => HttpResponse::InternalServerError().json("Internal Server Error"),
                },
                _ => HttpResponse::InternalServerError().json("Internal Server Error"),
            },
            _ => HttpResponse::InternalServerError().json("Internal Server Error"),
        },
    }
}
