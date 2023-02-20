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
        Err((actix_web::error::ErrorUnauthorized("Unauthorized"), req))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Token {
    token: String,
}

pub async fn check_role(
    req_token: Option<ReqData<Token>>,
    pool: web::Data<Pool<Postgres>>,
) -> Result<Role, String> {
    match req_token {
        Some(token) => {
            let token = token.into_inner();
            let token = token.token;
            let pool = pool.get_ref();
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
