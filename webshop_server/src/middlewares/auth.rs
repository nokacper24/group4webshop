use crate::data_access::{
    self, auth,
    user::{Role, User},
};

use actix_web::{web::ReqData, HttpRequest, Result};

use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

pub const COOKIE_KEY_SECRET: &str = "Secret";

pub async fn check_auth(cookie: &str, pool: &Pool<Postgres>) -> bool {
    match auth::is_valid_cookie(pool, cookie).await {
        Ok(valid) => valid,
        Err(_) => false,
    }
}

pub async fn get_cookie(cookie: &str, pool: &Pool<Postgres>) -> Result<auth::Cookie, sqlx::Error> {
    auth::get_cookie(pool, cookie).await
}

/// Returns the user associated with the the "Secret" cookie in the request.
pub async fn validate_user(req: HttpRequest, pool: &Pool<Postgres>) -> Result<User, AuthError> {
    match req.cookie(COOKIE_KEY_SECRET) {
        Some(cookie) => {
            let cookie = cookie.value();
            let valid = match auth::is_valid_cookie(pool, cookie).await {
                Ok(valid) => valid,
                Err(e) => return Err(AuthError::SqlxError(e)),
            };
            if valid {
                let cookie = get_cookie(cookie, &pool).await;
                return match cookie {
                    Ok(cookie) => {
                        let user = data_access::user::get_user_by_id(&pool, cookie.user_id).await;
                        return match user {
                            Ok(user) => Ok(user),
                            Err(e) => Err(AuthError::SqlxError(e)),
                        };
                    }
                    Err(e) => Err(AuthError::SqlxError(e)),
                };
            } else {
                return Err(AuthError::Unauthorized);
            }
        }
        None => {
            return Err(AuthError::Unauthorized);
        }
    }
}

/// Extracts the secret cookie from the request and returns it.
/// If there is no cookie or the cookie is invalid, returns `AuthError::Unauthorized`.
pub async fn extract_valid_cookie(
    req: HttpRequest,
    pool: &Pool<Postgres>,
) -> Result<String, AuthError> {
    match req.cookie(COOKIE_KEY_SECRET) {
        Some(cookie) => {
            let cookie = cookie.value();
            let valid = match auth::is_valid_cookie(pool, cookie).await {
                Ok(valid) => valid,
                Err(e) => return Err(AuthError::SqlxError(e)),
            };
            if valid {
                return Ok(cookie.to_string());
            } else {
                return Err(AuthError::Unauthorized);
            }
        }
        None => {
            return Err(AuthError::Unauthorized);
        }
    }
}
#[derive(Debug, Clone, Deserialize, Serialize)]
#[deprecated]
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
#[deprecated = "Use validate_user instead"]
pub async fn check_role(
    req_token: Option<ReqData<Token>>,
    pool: &Pool<Postgres>,
) -> Result<Role, AuthErrorOld> {
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
                        Err(e) => Err(AuthErrorOld::SqlxError(e)),
                    }
                }
                Err(_) => Err(AuthErrorOld::BadToken),
            }
        }
        _ => Err(AuthErrorOld::BadToken),
    }
}
/// Error type for auth
/// BadToken: The token was not found or is invalid - do 401 Unauthorized
/// SqlxError: An error occured while querying the database - probably 500 Internal Server Error
#[deprecated]
pub enum AuthErrorOld {
    SqlxError(sqlx::Error),
    NoSecret,
    BadToken,
    InvalidCookie,
}

pub enum AuthError {
    SqlxError(sqlx::Error),
    Unauthorized,
}
