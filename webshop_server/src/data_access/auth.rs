use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

use sqlx::{
    query_as, {Pool, Postgres},
};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Cookie {
    pub id: i32,
    pub cookie: String,
    pub exp: DateTime<Utc>,
    pub user_id: i32,
}

pub struct UserInvite {
    pub id: i32,
    pub key: String,
    pub email: String,
    pub exp_date: DateTime<Utc>,
}

/// Returns true if the cookie is valid, false if it is not.
/// Errors if the database query fails.
pub async fn is_valid_cookie(pool: &Pool<Postgres>, cookie: &str) -> Result<bool, sqlx::Error> {
    let cookie = match query_as!(
        Cookie,
        "SELECT * FROM cookies WHERE cookie = $1 LIMIT 1",
        cookie
    )
    .fetch_one(pool)
    .await{
        Ok(cookie) => cookie,
        Err(e) => match e {
            sqlx::Error::RowNotFound => return Ok(false),
            _ => return Err(e),
        }
    };

    if cookie.exp < Utc::now() {
        sqlx::query!("DELETE FROM cookies WHERE cookie = $1", cookie.cookie)
            .execute(pool)
            .await?;
        return Ok(false);
    }
    Ok(true)
}

/// creates a new cookie for a user with a 1 day expiration date.
pub async fn create_cookie(pool: &Pool<Postgres>, user_id: &i32) -> Result<String, sqlx::Error> {
    let cookie = Uuid::new_v4().to_string();
    let exp = Utc::now() + Duration::days(1);
    sqlx::query!(
        "INSERT INTO cookies (cookie, exp, user_id) VALUES ($1, $2, $3)",
        cookie,
        exp,
        user_id
    )
    .execute(pool)
    .await?;
    Ok(cookie)
}

/// deletes a cookie from the database.
/// returns true if the cookie was deleted, false if it was not found.
/// returns an error if the database query failed.
pub async fn delete_cookie(pool: &Pool<Postgres>, cookie: &str) -> Result<bool, sqlx::Error> {
    let delete = sqlx::query!("DELETE FROM cookies WHERE cookie = $1", cookie)
        .execute(pool)
        .await?;

    if delete.rows_affected() == 0 {
        return Ok(false);
    }
    Ok(true)
}

/// Get the cookie by cookie string.
pub async fn get_cookie(pool: &Pool<Postgres>, cookie: &str) -> Result<Cookie, sqlx::Error> {
    let cookie = query_as!(Cookie, "SELECT * FROM cookies WHERE cookie = $1", cookie)
        .fetch_one(pool)
        .await?;
    if cookie.exp < Utc::now() {
        sqlx::query!("DELETE FROM cookies WHERE cookie = $1", cookie.cookie)
            .execute(pool)
            .await?;
        return Err(sqlx::Error::RowNotFound);
    }
    Ok(cookie)
}


