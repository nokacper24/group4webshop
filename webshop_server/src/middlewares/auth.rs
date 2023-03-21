use crate::data_access::{
    self, auth,
    user::User,
};

use actix_web::{HttpRequest, Result};

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

/// Error type for authentication.
#[derive(Debug)]
pub enum AuthError {
    /// An error occured while querying the database
    /// - probably 500 Internal Server Error
    SqlxError(sqlx::Error),
    /// Cookie was either not found or is invalid
    /// - do 401 Unauthorized
    Unauthorized,
}
