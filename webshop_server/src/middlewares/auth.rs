use crate::{data_access::auth::{self}, Token};

use actix_web::{
    dev::ServiceRequest, web, Error, Result, HttpMessage,
};

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

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {

    let pool = req.app_data::<web::Data<Pool<Postgres>>>().expect("No pool found");
let cookie = credentials.token();
    let valid = check_auth(cookie, &pool).await;
    if valid {
        req.extensions_mut().insert(Token { token: cookie.to_string() });
        Ok(req)
    } else {
        Err((actix_web::error::ErrorUnauthorized("Unauthorized"), req))
    }
}
