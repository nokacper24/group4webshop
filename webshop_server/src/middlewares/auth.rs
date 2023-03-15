use crate::data_access::{
    self,
    auth::{self},
    user::{Role, User},
};

use actix_web::{
    dev::ServiceRequest,
    web::{self, ReqData},
    Error, HttpMessage, HttpResponse, Result, HttpRequest,
};

use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

pub async fn check_auth(cookie: &str, pool: &Pool<Postgres>) -> bool {
    match auth::is_valid_cookie(pool, cookie).await {
        Ok(valid) => valid,
        Err(_) => false,
    }
}

pub async fn get_cookie(cookie: &str, pool: &Pool<Postgres>) -> Result<auth::Cookie, sqlx::Error> {
    auth::get_cookie(pool, cookie).await
}

/// Middleware to check if the token is valid and if the user is logged in

pub async fn validator(req: HttpRequest) -> Result<User, Error> {
    let pool = req
        .app_data::<web::Data<Pool<Postgres>>>()
        .expect("No pool found");

    // get the token from the request e.g. cookie: Bearer=token
    match req.cookie("Bearer") {
        None => {
            return Err(actix_web::error::ErrorUnauthorized("No token found"));
        }
        Some(cookie) => {
            let cookie = cookie.value();
            let valid = check_auth(cookie, &pool).await;

            match valid {
                true => {
                    let cookie = get_cookie(cookie, &pool).await;
                    return match cookie {
                        Ok(cookie) => {
                            let user =
                                data_access::user::get_user_by_id(&pool, cookie.user_id).await;
                            return match user {
                                Ok(user) => Ok(user),
                                Err(e) => Err(actix_web::error::ErrorUnauthorized(e.to_string())),
                            };
                        }
                        Err(e) => Err(actix_web::error::ErrorUnauthorized(e.to_string())),
                    };
                }
                false => {
                    return Err(actix_web::error::ErrorUnauthorized("Invalid token"));
                }
            }
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Token {
    pub token: String,
}


/// Extracts the token from the request and returns the role
///  shorthand for getting role from a logged in user
/// # Example
/// ```
/// use webshop_server::middlewares::auth;
///
/// #[get("/")]
/// async fn index(req_token: Option<ReqData<auth::Token>>, pool: &Pool<Postgres>) -> Result<HttpResponse, String> {
///   let role = auth::check_role(req_token, pool).await;
///  match role {
///     Ok(role) => Ok(HttpResponse::Ok().json(role)),
///    Err(e) => Err(e),
///     }
/// }
pub async fn check_role(
    req_token: Option<ReqData<Token>>,
    pool: &Pool<Postgres>,
) -> Result<Role, AuthError> {
    match req_token {
        Some(token) => {
            let token = token.into_inner();
            let token = token.token;
            let cookie = get_cookie(&token, &pool).await;
            match cookie {
                Ok(cookie) => {
                    let user = data_access::user::get_user_by_id(&pool, cookie.user_id).await;
                    match user {
                        Ok(role) => Ok(role.role),
                        Err(e) => Err(AuthError::SqlxError(e)),
                    }
                }
                Err(_) => Err(AuthError::BadToken),
            }
        }
        _ => Err(AuthError::BadToken),
    }
}
/// Error type for auth
/// BadToken: The token was not found or is invalid - do 401 Unauthorized
/// SqlxError: An error occured while querying the database - probably 500 Internal Server Error
pub enum AuthError {
    SqlxError(sqlx::Error),
    BadToken,
}
