
use crate::{
    data_access::{
        self, company, error_handling, license,
        user::{self, LicenseUser, PartialRegisterCompanyUser, Role, User, UserID, UserRole},
    },
    utils::{self, auth, email::SupportEmail},
    SharedData,
};
use actix_multipart::{Multipart, MultipartError};
use actix_web::{delete, get, patch, post, web, HttpRequest, HttpResponse, Responder};

use futures::StreamExt;

use log::error;
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(users);
    cfg.service(user_by_id);
    cfg.service(users_by_company);
    cfg.service(users_by_license);
    cfg.service(generate_invite_new);
    cfg.service(generate_invite);
    cfg.service(generate_invites);
    cfg.service(add_license_users);
    cfg.service(remove_license_users);
    cfg.service(get_users_by_role);
    cfg.service(update_user_roles);
    cfg.service(delete_users);
    cfg.service(update_user);
    cfg.service(support);
    cfg.service(invite_type);
    cfg.service(register_new_company_user);
    cfg.service(register_new_user);
    cfg.service(get_invite_info);
}

#[derive(OpenApi)]
#[openapi(
    paths(
        users,
        user_by_id,
        users_by_company,
        users_by_license,
        generate_invite_new,
        generate_invite,
        generate_invites,
        add_license_users,
        remove_license_users,
        get_users_by_role,
        update_user_roles,
        delete_users,
        update_user,
        support,
        invite_type,
        register_new_company_user,
        register_new_user,
        get_invite_info

    ),
    components(
        schemas(User, Role, UserRole, LicenseUser, LicenseUsers, UserIDs, UserID)
    ),
    tags(
        (name = "Users", description = "API endpoints for users"),
        (name = "Invite", description = "API endpoints for invites"),
        (name = "Support", description = "API endpoints for support"),
    ),
)]
pub struct UserApiDoc;

#[utoipa::path(
    context_path = "/api/priv",
    tag = "Users",
    responses(
    (status = 200, description = "List of all available users", body = Vec<User>),
    (status = 500, description = "Internal Server Error"),
    )
)]
#[get("/users")]
async fn users(shared_data: web::Data<SharedData>, req: HttpRequest) -> impl Responder {
    let pool = &shared_data.db_pool;
    match auth::validate_user(req, pool).await {
        Ok(user) => {
            if user.role != user::Role::Admin {
                return HttpResponse::Forbidden().finish();
            }
        }
        Err(e) => {
            return match e {
                auth::AuthError::Unauthorized => HttpResponse::Unauthorized().finish(),
                auth::AuthError::SqlxError(e) => {
                    error!("{}", e);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
    };

    let users = user::get_all_users(pool).await;

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

#[utoipa::path(
    context_path = "/api/priv",
    tag = "Users",
    responses(
    (status = 200, description = "User with specific ID", body = User),
    (status = 400, description = "User ID not recognized"),
    (status = 500, description = "Internal Server Error"),
    )
)]
#[get("/users/{id}")]
async fn user_by_id(
    shared_data: web::Data<SharedData>,
    id: web::Path<String>,
    req: HttpRequest,
) -> impl Responder {
    let pool = &shared_data.db_pool;

    let id = match id.parse::<i32>() {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().json("Bad Request"),
    };
    let user = user::get_user_by_id(pool, &id).await;

    let auth_user = match auth::validate_user(req, pool).await {
        Ok(auth_user) => {
            if auth_user.role != user::Role::Admin
                && auth_user.role != user::Role::CompanyItHead
                && auth_user.role != user::Role::CompanyIt
            {
                return HttpResponse::Forbidden().finish();
            }
            auth_user
        }
        Err(e) => {
            return match e {
                auth::AuthError::Unauthorized => HttpResponse::Unauthorized().finish(),
                auth::AuthError::SqlxError(e) => {
                    error!("{}", e);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
    };

    //parse to json
    match user {
        Ok(user) => {
            if auth_user.company_id != user.company_id && auth_user.role != user::Role::Admin {
                return HttpResponse::Forbidden().finish();
            }
            HttpResponse::Ok().json(user)
        }
        Err(e) => match e {
            sqlx::Error::RowNotFound => HttpResponse::NotFound().json("User not found"),
            _ => HttpResponse::InternalServerError().json("Internal Server Error"),
        },
    }
}

/// Get all the users that work for a specific company.
#[utoipa::path(
    context_path = "/api/priv",
    tag = "Users",
    responses(
    (status = 200, description = "List of all users in a specific company", body = Vec<User>),
    (status = 400, description = "Company ID not recognized"),
    (status = 500, description = "Internal Server Error"),
    )
)]
#[get("/companies/{company_id}/users")]
async fn users_by_company(
    shared_data: web::Data<SharedData>,
    company_id: web::Path<String>,
    req: HttpRequest,
) -> impl Responder {
    let pool = &shared_data.db_pool;
    let auth_user = match auth::validate_user(req, pool).await {
        Ok(auth_user) => {
            if auth_user.role != user::Role::Admin
                && auth_user.role != user::Role::CompanyItHead
                && auth_user.role != user::Role::CompanyIt
            {
                return HttpResponse::Forbidden().finish();
            }
            auth_user
        }
        Err(e) => {
            return match e {
                auth::AuthError::Unauthorized => HttpResponse::Unauthorized().finish(),
                auth::AuthError::SqlxError(e) => {
                    error!("{}", e);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
    };
    let company_id = match company_id.parse::<i32>() {
        Ok(company_id) => company_id,
        Err(_) => return HttpResponse::BadRequest().json("Bad Request"),
    };

    if auth_user.company_id != company_id && auth_user.role != user::Role::Admin {
        return HttpResponse::Forbidden().finish();
    }

    let other_users = user::get_users_by_company(pool, &company_id).await;

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
#[utoipa::path(
    context_path = "/api/priv",
    tag = "Users",
    responses(
    (status = 200, description = "List of all users with access to a speecific license", body = Vec<User>),
    (status = 400, description = "License ID not recognized"),
    (status = 500, description = "Internal Server Error"),
    )
)]
#[get("/licenses/{license_id}/users")]
async fn users_by_license(
    shared_data: web::Data<SharedData>,
    license_id: web::Path<String>,
    req: HttpRequest,
) -> impl Responder {
    let pool = &shared_data.db_pool;
    let license_id = match license_id.parse::<i32>() {
        Ok(license_id) => license_id,
        Err(_) => return HttpResponse::BadRequest().json("Bad Request"),
    };

    let auth_user = match auth::validate_user(req, pool).await {
        Ok(auth_user) => {
            if auth_user.role != user::Role::Admin
                && auth_user.role != user::Role::CompanyItHead
                && auth_user.role != user::Role::CompanyIt
            {
                return HttpResponse::Forbidden().finish();
            }
            auth_user
        }
        Err(e) => {
            return match e {
                auth::AuthError::Unauthorized => HttpResponse::Unauthorized().finish(),
                auth::AuthError::SqlxError(e) => {
                    error!("{}", e);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
    };

    let license = license::get_license_by_id(pool, &license_id).await;

    match license {
        Ok(license) => {
            if auth_user.company_id != license.company_id && auth_user.role != user::Role::Admin {
                return HttpResponse::Forbidden().finish();
            }
        }
        Err(e) => match e {
            sqlx::Error::RowNotFound => return HttpResponse::NotFound().json("License not found"),
            _ => return HttpResponse::InternalServerError().json("Internal Server Error"),
        },
    }

    let other_users = user::get_users_by_license(pool, &license_id).await;

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
struct NewUser {
    email: String,
}

#[utoipa::path(
    context_path = "/api/priv",
    tag = "Users",
    responses(
    (status = 200, description = "Invite info", body = User),
    (status = 400, description = "Invite ID not recognized"),
    (status = 500, description = "Internal Server Error"),
    )
)]
#[post("/generate_invite_new")]
async fn generate_invite_new(
    shared_data: web::Data<SharedData>,
    invite: web::Json<NewUser>,
) -> impl Responder {
    let pool = &shared_data.db_pool;
    let mailer = &shared_data.mailer;
    let email = &invite.email;

    // check if user exists
    match user::user_exists(email, pool).await {
        Ok(true) => HttpResponse::BadRequest().json("User already exists"),
        Ok(false) => {
            let partial_user = user::create_partial_user(email, pool).await;
            match partial_user {
                Ok(partial_user) => {
                    let invite_obj = user::create_invite(Some(partial_user.id), None, pool).await;
                    match invite_obj {
                        Ok(invite_obj) => {
                            let email = utils::email::Email::new(
                                partial_user.email,
                                utils::email::EmailType::RegisterUser,
                                Some(invite_obj.id),
                            );
                            let res = utils::email::send_email(email, mailer).await;
                            match res {
                                Ok(_) => HttpResponse::Ok().json("Invite sent"),
                                Err(e) => {
                                    log::error!("Error: {}", e);
                                    HttpResponse::InternalServerError()
                                        .json("Internal Server Error")
                                }
                            }
                        }
                        Err(e) => {
                            log::error!("Error: {}", e);
                            HttpResponse::InternalServerError().json("Internal Server Error")
                        }
                    }
                }
                Err(e) => {
                    log::error!("Error: {}", e);
                    HttpResponse::InternalServerError().json("Internal Server Error")
                }
            }
        }
        Err(e) => {
            log::error!("Error: {}", e);
            HttpResponse::InternalServerError().json("Internal Server Error")
        }
    }
}

#[derive(Deserialize, Serialize)]
struct Invite {
    email: String,
    company_id: Option<i32>,
}

#[utoipa::path(
    context_path = "/api/priv",
    tag = "Users",
    responses(
    (status = 200, description = "Invite info", body = User),
    (status = 400, description = "Invite ID not recognized"),
    (status = 500, description = "Internal Server Error"),
    )
)]
#[post("/generate_invite")]
async fn generate_invite(
    shared_data: web::Data<SharedData>,
    invite: web::Json<Invite>,
    req: HttpRequest,
) -> impl Responder {
    let pool = &shared_data.db_pool;
    let mailer = &shared_data.mailer;
    let company = match invite.company_id {
        Some(company_id) => match company::get_company_by_id(pool, &company_id).await {
            Ok(company) => Ok(company),
            Err(e) => {
                log::error!("Error: {}", e);
                return HttpResponse::InternalServerError().json("Internal Server Error");
            }
        },
        None => Err("No company ID provided"),
    };

    let user = match auth::validate_user(req.clone(), pool).await {
        Ok(user) => user,
        Err(e) => match e {
            auth::AuthError::Unauthorized => {
                return HttpResponse::Unauthorized().json("Must be logged in to invite new users");
            }
            auth::AuthError::SqlxError(e) => {
                log::error!("Error: {}", e);
                return HttpResponse::InternalServerError().json("Internal Server Error");
            }
        },
    };

    match user.role {
        user::Role::Admin => match company {
            Ok(company) => {
                let partial =
                    user::create_partial_company_user(&invite.email, company.company_id, pool)
                        .await;
                match partial {
                    Ok(partial) => {
                        let invite =
                            user::create_invite(None, Some(partial.id), pool).await;
                        match invite {
                            Ok(invite) => {
                                let email = utils::email::Email::new(
                                    partial.email,
                                    utils::email::EmailType::RegisterUserCompany,
                                    Some(invite.id),
                                );
                                let res = utils::email::send_email(email,mailer).await;
                                match res {
                                    Ok(_) => {
                                        HttpResponse::Ok().json("Invite sent")
                                    }
                                    Err(e) => {
                                        log::error!("Error: {}", e);
                                        HttpResponse::InternalServerError()
                                            .json("Internal Server Error")
                                    }
                                }
                            }
                            Err(e) => {
                                log::error!("Error: {}", e);
                                HttpResponse::InternalServerError()
                                    .json("Internal Server Error")
                            }
                        }
                    }
                    Err(e) => {
                        log::error!("Error: {}", e);
                        HttpResponse::InternalServerError().json("Internal Server Error")
                    }
                }
            }
            Err(e) => {
                log::error!("Error: {}", e);
                HttpResponse::InternalServerError().json("No company ID provided")
            }
        },

        user::Role::CompanyItHead | user::Role::CompanyIt => {
            match company {
                Ok(company) => {
                    if company.company_id == user.company_id {
                        let partial = user::create_partial_company_user(
                            &invite.email,
                            company.company_id,
                            pool,
                        )
                        .await;
                        match partial {
                            Ok(partial) => {
                                let invite =
                                    user::create_invite(None, Some(partial.id), pool)
                                        .await;
                                match invite {
                                    Ok(invite) => {
                                        let email = utils::email::Email::new(
                                            partial.email,
                                            utils::email::EmailType::RegisterUserCompany,
                                            Some(invite.id),
                                        );
                                        let res = utils::email::send_email(email,mailer).await;
                                        match res {
                                            Ok(_) => {
                                                HttpResponse::Ok().json("Invite sent")
                                            }
                                            Err(e) => {
                                                log::error!("Error: {}", e);
                                                HttpResponse::InternalServerError()
                                                    .json("Internal Server Error")
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        log::error!("Error: {}", e);
                                        HttpResponse::InternalServerError()
                                            .json("Internal Server Error")
                                    }
                                }
                            }
                            Err(e) => {
                                log::error!("Error: {}", e);
                                HttpResponse::InternalServerError()
                                    .json("Internal Server Error")
                            }
                        }
                    } else {
                        HttpResponse::Forbidden()
                            .json("You don't have permission to invite users to this company")
                    }
                }
                Err(_e) => {
                    //if no id is provided, then the user is trying to invite a user to their company
                    let comp_id = user.company_id;
                    let partial =
                        user::create_partial_company_user(&invite.email, comp_id, pool).await;
                    match partial {
                        Ok(partial) => {
                            let invite =
                                user::create_invite(None, Some(partial.id), pool).await;
                            match invite {
                                Ok(invite) => {
                                    let email = utils::email::Email::new(
                                        partial.email,
                                        utils::email::EmailType::RegisterUserCompany,
                                        Some(invite.id),
                                    );
                                    let res = utils::email::send_email(email,mailer).await;
                                    match res {
                                        Ok(_) => {
                                            HttpResponse::Ok().json("Invite sent")
                                        }
                                        Err(e) => {
                                            log::error!("Error: {}", e);
                                            HttpResponse::InternalServerError()
                                                .json("Internal Server Error")
                                        }
                                    }
                                }
                                Err(e) => {
                                    log::error!("Error: {}", e);
                                    HttpResponse::InternalServerError()
                                        .json("Internal Server Error")
                                }
                            }
                        }
                        Err(e) => {
                            log::error!("Error: {}", e);
                            HttpResponse::InternalServerError()
                                .json("Internal Server Error")
                        }
                    }
                }
            }
        }
        user::Role::Default => {
            HttpResponse::Forbidden().json("Normal users don't have permission to generate a new user, please contact your company's IT department.")
        }
    }
}

#[utoipa::path(
    context_path = "/api/priv",
    tag = "Users",
    responses(
    (status = 200, description = "Invite info", body = User),
    (status = 400, description = "Invite ID not recognized"),
    (status = 500, description = "Internal Server Error"),
    )
)]
#[post("/generate_invites")]
async fn generate_invites(
    shared_data: web::Data<SharedData>,
    payload: Multipart,
    req: HttpRequest,
) -> impl Responder {
    let pool = &shared_data.db_pool;
    let user = match auth::validate_user(req, pool).await {
        Ok(user) => user,
        Err(e) => {
            return match e {
                auth::AuthError::Unauthorized => HttpResponse::Unauthorized().finish(),
                auth::AuthError::SqlxError(_) => HttpResponse::InternalServerError().finish(),
            }
        }
    };
    if (user.role != user::Role::Admin)
        && (user.role != user::Role::CompanyItHead)
        && (user.role != user::Role::CompanyIt)
    {
        return HttpResponse::Forbidden().finish();
    }

    match extract_text_from_multipart(payload).await {
        Ok(text) => {
            let email_list = csv_string_to_list(text);
            let mut other_users = Vec::<PartialRegisterCompanyUser>::new();

            for email in email_list.as_slice() {
                if !email.is_empty() {
                    other_users.push(PartialRegisterCompanyUser {
                        email: email.to_string(),
                        company_id: user.company_id,
                    });
                }
            }

            match user::create_partial_company_users(&other_users, pool).await {
                Ok(created_users) => HttpResponse::Ok().json(created_users),
                Err(_) => HttpResponse::InternalServerError().finish(),
            }
        }
        Err(e) => HttpResponse::UnprocessableEntity().json(e.to_string()),
    }
}

/// Extract the text from a `Multipart`.
async fn extract_text_from_multipart(mut payload: Multipart) -> Result<String, MultipartError> {
    let mut buffer = Vec::new();

    while let Some(item) = payload.next().await {
        let mut field = match item {
            Ok(field) => field,
            Err(e) => return Err(e),
        };

        let name = match field.content_disposition().get_name() {
            Some(name) => name,
            None => return Err(MultipartError::NoContentDisposition),
        };

        if name == "csv" {
            while let Some(chunk) = field.next().await {
                let data = match chunk {
                    Ok(data) => data,
                    Err(e) => return Err(e),
                };

                buffer.extend_from_slice(&data);
            }
        } else {
            return Err(MultipartError::Incomplete);
        }
    }

    match String::from_utf8(buffer) {
        Ok(text) => Ok(text),
        Err(e) => Err(MultipartError::Parse(e.into())),
    }
}

/// Parses a `String` that is separated by commas into a `Vec` of `String` and each `String` is trimmed.
/// # Arguments
/// * `text` - The text to be split
/// # Returns
/// * `Vec<String>` - A list of Strings
/// # Example
/// ```rust
/// let hello = String::from("Hello, world!");
/// let list = csv_string_to_list(hello);
///
/// assert_eq!(list, vec!["Hello", "world!"])
/// ```
fn csv_string_to_list(text: String) -> Vec<String> {
    let list: Vec<&str> = text.split(',').collect();
    let mut string_list = Vec::<String>::new();
    for word in list {
        string_list.push(word.trim().to_string());
    }
    string_list
}

#[derive(Serialize, Deserialize, ToSchema)]
struct LicenseUsers {
    users: Vec<LicenseUser>,
}

/// Add access to licenses for users.
#[utoipa::path(
    context_path = "/api/priv",
    tag = "Users",
    responses(
    (status = 201, description = "License user successfully added", body = Vec<User>),
    (status = 409, description = "License user already existed"),
    (status = 500, description = "Internal Server Error"),
    ),
    request_body(
        description = "Pairs of user_id and license_id to grant access to licenses",
        content = LicenseUsers,
    )
)]
#[post("/license_users")]
async fn add_license_users(
    shared_data: web::Data<SharedData>,
    other_users: web::Json<LicenseUsers>,
) -> impl Responder {
    let pool = &shared_data.db_pool;
    let other_users = &other_users.users;
    match user::add_license_users(pool, other_users).await {
        Ok(_) => HttpResponse::Created().json(other_users),

        Err(e) => match e {
            sqlx::Error::Database(e) => match error_handling::PostgresDBError::from_str(e) {
                error_handling::PostgresDBError::UniqueViolation => {
                    HttpResponse::Conflict().json("Record already exists")
                }
                _ => HttpResponse::InternalServerError().json("Internal Server Error"),
            },
            _ => HttpResponse::InternalServerError().json("Internal Server Error"),
        },
    }
}

/// Remove access to licenses from users.
#[utoipa::path(
    context_path = "/api/priv",
    tag = "Users",
    responses(
    (status = 200, description = "License users successfully removed", body = Vec<User>),
    (status = 500, description = "Internal Server Error"),
    ),
    request_body (
        description = "Pairs of user_id and license_id to remove access to licenses",
        content = LicenseUsers,
        
    )

)]
#[delete("/license_users")]
async fn remove_license_users(
    shared_data: web::Data<SharedData>,
    other_users: web::Json<LicenseUsers>,
) -> impl Responder {
    let pool = &shared_data.db_pool;
    let other_users = &other_users.users;
    match user::remove_license_users(pool, other_users).await {
        Ok(_) => HttpResponse::Ok().json(other_users),
        Err(_e) => HttpResponse::InternalServerError().json("Internal Server Error"),
    }
}

/// Get all users with a specific role.
#[utoipa::path(
    context_path = "/api/priv",
    tag = "Users",
    responses(
        (status = 200, description = "List of all users with a specific role", body = Vec<User>),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 500, description = "Internal Server Error"),
    )
)]
#[get("/users/role/{role}")]
async fn get_users_by_role(
    shared_data: web::Data<SharedData>,
    role: web::Path<Role>,
    request: HttpRequest,
) -> impl Responder {
    let pool = &shared_data.db_pool;
    match auth::validate_user(request, pool).await {
        Ok(user) => {
            if user.role != user::Role::Admin {
                return HttpResponse::Forbidden().finish();
            }
        }
        Err(e) => {
            return match e {
                auth::AuthError::Unauthorized => HttpResponse::Unauthorized().finish(),
                auth::AuthError::SqlxError(e) => {
                    error!("{}", e);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
    }
    let found_users = user::get_users_by_role(pool, &role).await;
    match found_users {
        Ok(found_users) => HttpResponse::Ok().json(found_users),
        Err(e) => {
            log::error!("Error: {}", e);
            HttpResponse::InternalServerError().json("Internal Server Error")
        }
    }
}

#[derive(Deserialize, Serialize)]
struct UserRoles {
    users: Vec<UserRole>,
}

/// Update users' roles.
#[utoipa::path (
    context_path = "/api/priv",
    patch,
    tag = "Users",
    responses(
        (status = 200, description = "Users' roles have been updated", body = Vec<UserRole>),
        (status = 500, description = "Internal Server Error"),
        ),
    )
  ]
#[patch("/user_roles")]
async fn update_user_roles(
    shared_data: web::Data<SharedData>,
    other_users: web::Json<UserRoles>,
) -> impl Responder {
    let pool = &shared_data.db_pool;
    let other_users = &other_users.users;
    match user::update_user_roles(pool, other_users).await {
        Ok(_) => HttpResponse::Ok().json(other_users),
        Err(_e) => HttpResponse::InternalServerError().json("Internal Server Error"),
    }
}

#[derive(Deserialize, Serialize, ToSchema)]
struct UserIDs {
    users: Vec<UserID>,
}

/// Delete users
#[utoipa::path (
    context_path = "/api/priv",
    delete,
    tag = "Users",
    responses(
        (status = 200, description = "Users have been deleted.", body = Vec<UserID>),
        (status = 500, description = "Internal Server Error"),
        ),
    request_body (
        description = "List of user IDs to delete",
        content = UserIDs,
    ),
    )
  ]
#[delete("/users")]
async fn delete_users(
    shared_data: web::Data<SharedData>,
    other_users: web::Json<UserIDs>,
) -> impl Responder {
    let pool = &shared_data.db_pool;
    let other_users = &other_users.users;
    match user::delete_users(pool, other_users).await {
        Ok(_) => HttpResponse::Ok().json(other_users),
        Err(_e) => HttpResponse::InternalServerError().json("Internal Server Error"),
    }
}

#[derive(Deserialize, Serialize)]
struct PartialUser {
    email: Option<String>,
}

#[utoipa::path(
    context_path = "/api/priv",
    tag = "Users",
    patch,
    responses(
    (status = 200, description = "User updated", body = User),
    (status = 400, description = "User ID not recognized"),
    (status = 401, description = "Unauthorized"),
    (status = 500, description = "Internal Server Error"),
    )
)]
#[patch("/users/{id}")]
async fn update_user(
    shared_data: web::Data<SharedData>,
    id: web::Path<String>,
    body: web::Json<PartialUser>,
    req: HttpRequest,
) -> impl Responder {
    let pool = &shared_data.db_pool;
    let mail = &body.email;

    let user = match auth::validate_user(req, pool).await {
        Ok(user) => user,
        Err(e) => {
            return match e {
                auth::AuthError::Unauthorized => HttpResponse::Unauthorized().finish(),
                auth::AuthError::SqlxError(e) => {
                    error!("{}", e);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
    };


    let id: i32 = match id.parse::<i32>() {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };


    if user.role != user::Role::Admin && user.user_id != id
    {
        return HttpResponse::Forbidden().finish();
    }


    let returned_user = match mail {
        Some(email) => match user::update_email(pool, email, &id).await {
            Ok(user) => user,
            Err(_) => return HttpResponse::InternalServerError().finish(),
        },
        None => return HttpResponse::InternalServerError().finish(),
    };
    HttpResponse::Ok().json(returned_user)
}

#[derive(Deserialize, Serialize)]
struct SupportRequest {
    product: String,
    subject: String,
    message: String,
}

#[utoipa::path(
    context_path = "/api/priv",
    tag = "Support",
    request_body(
        content_type = "application/json",
        description = "The parameters of the mail",
    content = SupportRequest,
example = json!({
    "product": "Product name",
    "subject": "Subject",
    "message": "Message"
})),
    responses(
        (status = 200, description = "Support request sent"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error"),
    )
)]
#[post("/support")]
async fn support(
    shared_data: web::Data<SharedData>,
    body: web::Json<SupportRequest>,
    req: HttpRequest,
) -> impl Responder {
    let pool = &shared_data.db_pool;
    let mailer = &shared_data.mailer;
    let user = match auth::validate_user(req, pool).await {
        Ok(user) => user,
        Err(e) => {
            return match e {
                auth::AuthError::Unauthorized => HttpResponse::Unauthorized().finish(),
                auth::AuthError::SqlxError(e) => {
                    error!("{}", e);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
    };
    let support_email: SupportEmail = SupportEmail::new(
        user.email.clone(),
        body.product.clone(),
        body.subject.clone(),
        body.message.clone(),
    );
    let email = utils::email::Email::new(
        user.email,
        utils::email::EmailType::Support(support_email),
        None,
    );
    let res = utils::email::send_email(email, mailer).await;
    match res {
        Ok(_) => HttpResponse::Ok().json("Support request sent"),
        Err(e) => {
            log::error!("Error: {}", e);
            HttpResponse::InternalServerError().json("Internal Server Error")
        }
    }
}

#[utoipa::path(
    context_path = "/api/priv",
    get,
    tag = "Invite",
    params(
        ("invite_id", description = "The ID of the invite", example = "1234567890"),
    ),
    responses(
        (status = 200, description = "Invite type", body = String),
        (status = 404, description = "Invite not found"),
        (status = 500, description = "Internal Server Error"),
    )
)]
#[get("/invite-type/{invite_id}")]
async fn invite_type(
    invite_id: web::Path<String>,
    shared_data: web::Data<SharedData>,
) -> impl Responder {
    let pool = &shared_data.db_pool;

    let invite = match data_access::user::get_invite_by_id(&invite_id, pool).await {
        Ok(invite) => invite,
        Err(e) => match e {
            sqlx::Error::RowNotFound => {
                return HttpResponse::NotFound().json("Invite not found");
            }
            _ => {
                log::error!("Error: {}", e);
                return HttpResponse::InternalServerError().json("Internal Server Error");
            }
        },
    };

    let invite_type = match invite.company_user_id {
        Some(_) => "company",
        None => "user",
    };

    HttpResponse::Ok().json(invite_type)
}

#[derive(Deserialize, Serialize)]
struct InviteInfo {
    company_name: String,
    company_address: String,
    email: String,
    role: String,
}

#[utoipa::path(
    context_path = "/api/priv",
    tag = "Invite",
    get,
    params(
        ("invite_id", description = "The ID of the invite", example = "1234567890"),
    ),
    responses(
        (status = 200, description = "Invite info", body = InviteInfo),
        (status = 500, description = "Internal Server Error"),
    )
)]
#[get("/invite/info/{invite_id}")]
async fn get_invite_info(
    shared_data: web::Data<SharedData>,
    invite_id: web::Path<String>,
) -> impl Responder {
    let pool = &shared_data.db_pool;

    let invite = match data_access::user::get_invite_by_id(&invite_id, pool).await {
        Ok(invite) => invite,
        Err(e) => {
            log::error!("Error: {}", e);
            return HttpResponse::InternalServerError().json("Internal Server Error");
        }
    };

    let invite_info = if let Some(user_id) = invite.user_id {
        let user = match data_access::user::get_partial_user(&user_id, pool).await {
            Ok(user) => user,
            Err(e) => {
                log::error!("Error: {}", e);
                return HttpResponse::InternalServerError().json("Internal Server Error");
            }
        };
        InviteInfo {
            company_name: "".to_string(),
            company_address: "".to_string(),
            email: user.email,
            role: "".to_string(),
        }
    } else if let Some(company_urs_id) = invite.company_user_id {
        let company_user =
            match data_access::user::get_partial_company_user(&company_urs_id, pool).await {
                Ok(company_user) => company_user,
                Err(e) => {
                    log::error!("Error: {}", e);
                    return HttpResponse::InternalServerError().json("Internal Server Error");
                }
            };
        let company =
            match data_access::company::get_company_by_id(pool, &company_user.company_id).await {
                Ok(company) => company,
                Err(e) => {
                    log::error!("Error: {}", e);
                    return HttpResponse::InternalServerError().json("Internal Server Error");
                }
            };
        InviteInfo {
            company_name: company.company_name,
            company_address: company.company_address,
            email: company_user.email,
            role: "default".to_string(),
        }
    } else {
        log::error!("Error: invite has no user_id or company_user_id");
        return HttpResponse::InternalServerError().json("Internal Server Error");
    };

    return HttpResponse::Ok().json(invite_info);
}

#[derive(Deserialize, Serialize, ToSchema)]
struct RegisterUser {
    invite_id: String,
    company_name: String,
    company_address: String,
    password: String,
}
#[utoipa::path(
    context_path = "/api/priv",
    post,
    tag = "Users",
    responses(
        (status = 200, description = "User successfully registered", body = User),
        (status = 500, description = "Internal Server Error"),
    )
)]
#[post("/register_new_user")]
async fn register_new_user(
    shared_data: web::Data<SharedData>,
    register_user: web::Json<RegisterUser>,
) -> impl Responder {
    let pool = &shared_data.db_pool;

    let invite = match data_access::user::get_invite_by_id(&register_user.invite_id, pool).await {
        Ok(invite) => invite,
        Err(e) => {
            log::error!("Error: {}", e);
            return HttpResponse::InternalServerError().json("Internal Server Error");
        }
    };

    let company = match data_access::company::create_company(
        pool,
        &register_user.company_name,
        &register_user.company_address,
    )
    .await
    {
        Ok(company) => company,
        Err(e) => {
            log::error!("Error: {}", e);
            return HttpResponse::InternalServerError().json("Internal Server Error");
        }
    };

    // unpack Some invite.user_id
    let user_id = match invite.user_id {
        Some(user_id) => user_id,
        None => {
            return HttpResponse::InternalServerError().json("Internal Server Error");
        }
    };

    // Get partial user from invite
    let partial_user = match data_access::user::get_partial_user(&user_id, pool).await {
        Ok(partial_user) => partial_user,
        Err(e) => {
            log::error!("Error: {}", e);
            return HttpResponse::InternalServerError().json("Internal Server Error");
        }
    };

    let user = match data_access::user::create_user(
        &partial_user.email,
        &register_user.password,
        company.company_id,
        Role::CompanyItHead,
        pool,
    )
    .await
    {
        Ok(user) => user,
        Err(e) => {
            log::error!("Error: {:?}", e);
            return HttpResponse::InternalServerError().json("Internal Server Error");
        }
    };

    // Delete the invite
    match data_access::user::delete_invite(&invite.id, pool).await {
        Ok(_) => (),
        Err(e) => {
            log::error!("Error: {}", e);
            return HttpResponse::InternalServerError().json("Internal Server Error");
        }
    }

    // Delete the partial user
    match data_access::user::delete_partial_user(&partial_user.id, pool).await {
        Ok(_) => (),
        Err(e) => {
            log::error!("Error: {}", e);
            return HttpResponse::InternalServerError().json("Internal Server Error");
        }
    }

    HttpResponse::Ok().json(user)
}

#[derive(Deserialize, Serialize, ToSchema)]
struct RegisterCompanyUser {
    invite_id: String,
    password: String,
}
#[utoipa::path(
    context_path = "/api/priv",
    post,
    tag = "Users",
    responses(
        (status = 200, description = "User successfully registered", body = User),
        (status = 500, description = "Internal Server Error"),
    )
)]
#[post("/register_new_company_user")]
async fn register_new_company_user(
    shared_data: web::Data<SharedData>,
    register_user: web::Json<RegisterCompanyUser>,
) -> impl Responder {
    let pool = &shared_data.db_pool;

    let invite = match data_access::user::get_invite_by_id(&register_user.invite_id, pool).await {
        Ok(invite) => invite,
        Err(e) => {
            log::error!("Error: {}", e);
            return HttpResponse::InternalServerError().json("Internal Server Error");
        }
    };

    // unpack Some invite.company_user_id
    let company_user_id = match invite.company_user_id {
        Some(company_user_id) => company_user_id,
        None => {
            return HttpResponse::InternalServerError().json("Internal Server Error");
        }
    };

    let company = match data_access::company::get_company_by_id(pool, &company_user_id).await {
        Ok(company) => company,
        Err(e) => {
            log::error!("Error: {}", e);
            return HttpResponse::InternalServerError().json("Internal Server Error");
        }
    };

    // unpack Some invite.user_id
    let user_id = match invite.company_user_id {
        Some(user_id) => user_id,
        None => {
            return HttpResponse::InternalServerError().json("Internal Server Error");
        }
    };

    // Get partial user from invite
    let partial_user = match data_access::user::get_partial_company_user(&user_id, pool).await {
        Ok(partial_user) => partial_user,
        Err(e) => {
            log::error!("Error: {}", e);
            return HttpResponse::InternalServerError().json("Internal Server Error");
        }
    };

    let user = match data_access::user::create_user(
        &partial_user.email,
        &register_user.password,
        company.company_id,
        Role::Default,
        pool,
    )
    .await
    {
        Ok(user) => user,
        Err(e) => {
            log::error!("Error: {:?}", e);
            return HttpResponse::InternalServerError().json("Internal Server Error");
        }
    };

    // Delete the invite
    match data_access::user::delete_invite(&invite.id, pool).await {
        Ok(_) => (),
        Err(e) => {
            log::error!("Error: {}", e);
            return HttpResponse::InternalServerError().json("Internal Server Error");
        }
    }

    // Delete the partial user
    match data_access::user::delete_partial_company_user(&partial_user.id, pool).await {
        Ok(_) => (),
        Err(e) => {
            log::error!("Error: {}", e);
            return HttpResponse::InternalServerError().json("Internal Server Error");
        }
    }

    HttpResponse::Ok().json(user)
}
