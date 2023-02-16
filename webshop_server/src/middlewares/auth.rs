use crate::data_access::auth::{self, Cookie};

use actix_web::{
    dev::Service, dev::ServiceRequest, dev::ServiceResponse, web, Error, HttpMessage, HttpRequest,
    HttpResponse, Responder, Result,
};

use crate::data_access::user::{get_user_by_id, Role};
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

pub struct AuthenticatedUser {
    pub role: Role,
    pub user_id: i32,
}

pub async fn validate_auth<S>(
    req: ServiceRequest,
    service: &mut S,
    pool: &Pool<Postgres>,
) -> Result<ServiceResponse, actix_web::Error> 
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = actix_web::Error> + 'static,
{
    // Get the authorization header from the request
    let auth_header = req.headers().get("Authorization");

    // If the header isn't present, return an error
    let auth_header = match auth_header {
        Some(header_value) => header_value,
        None => return Err(actix_web::error::ErrorUnauthorized("Unauthorized")),
    };

    // Extract the token from the header value
    let token = match auth_header.to_str() {
        Ok(s) => s.replace("Bearer ", ""),
        Err(_) => return Err(actix_web::error::ErrorUnauthorized("Unauthorized")),
    };

    // Look up the token in the cookies table
    let cookie = match get_cookie(&token, &pool).await {
        Ok(cookie) => cookie,
        Err(_) => return Err(actix_web::error::ErrorUnauthorized("Unauthorized")),
    };
    // Look up the associated user in the users table
    let user = match get_user_by_id(&pool, cookie.user_id).await {
        Ok(user) => user,
        Err(_) => return Err(actix_web::error::ErrorUnauthorized("Unauthorized")),
    };

    // Create the AuthenticatedUser struct and return it
    let auth_user = AuthenticatedUser {
        role: user.role,
        user_id: user.user_id,
    };

    // Set the authenticated user in the request extensions
    req.extensions_mut().insert(auth_user);

    // Call the next middleware or handler
    let res = req.into_response(HttpResponse::Ok().finish());
    Ok(res)
}
