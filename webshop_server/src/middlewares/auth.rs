use crate::data_access::{
    self,
    auth::{self},
    user::Role,
};

use actix_web::{
    dev::ServiceRequest,
    web::{self, ReqData},
    Error, HttpMessage, Result,
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

use actix_web_httpauth::extractors::bearer::BearerAuth;

/// Middleware to check if the token is valid and if the user is logged in
pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let pool = req
        .app_data::<web::Data<Pool<Postgres>>>()
        .expect("No pool found");
    let cookie = credentials.token();
    let valid = check_auth(cookie, &pool).await;
    if valid {
        req.extensions_mut().insert(Token {
            token: cookie.to_string(),
        });
        Ok(req)
    } else {
        Ok(req)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Token {
    pub token: String,
}

/// Extracts the token from the request and returns the user
/// # Example
/// ```
/// use webshop_server::middlewares::auth;
///
/// #[get("/")]
/// async fn index(req_token: Option<ReqData<auth::Token>>, pool: &Pool<Postgres>) -> Result<HttpResponse, String> {
///    let user = auth::get_user_from_token(req_token, pool).await;
///   match user {
///       Ok(user) => Ok(HttpResponse::Ok().json(user)),
///      Err(e) => Err(e),
///  }
/// }
/// ```
/// # Errors
/// Returns an error if the token is invalid or the user is not found
pub async fn get_user_from_token(
    req_token: Option<ReqData<Token>>,
    pool: &Pool<Postgres>,
) -> Result<data_access::user::User, String> {
    match req_token {
        Some(token) => {
            let token = token.into_inner();
            let token = token.token;
            let cookie = get_cookie(&token, &pool).await;
            match cookie {
                Ok(cookie) => {
                    let user = data_access::user::get_user_by_id(&pool, cookie.user_id).await;
                    match user {
                        Ok(user) => Ok(user),
                        Err(e) => Err(e.to_string()),
                    }
                }
                Err(_) => Err("Unauthorized".to_string()),
            }
        }
        _ => Err("Unauthorized".to_string()),
    }
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
) -> Result<Role, String> {
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
                        Err(e) => Err(e.to_string()),
                    }
                }
                Err(_) => Err("Unauthorized".to_string()),
            }
        }
        _ => Err("Unauthorized".to_string()),
    }
}
