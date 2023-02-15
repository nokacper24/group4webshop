use serde::{Deserialize, Serialize};
use sqlx::{
    query_as, {Pool, Postgres},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Cookies {
    id: i32,
    cookie: String,
    exp: DateTime<Utc>,
    user_id: i32,
}

/// Returns Auth Cookies by cookie, and checks if it is valid.
pub async fn is_valid_cookie(pool: &Pool<Postgres>, cookie: &str) -> Result<bool, sqlx::Error> {
    let cookies = query_as!(Cookies, "SELECT * FROM cookies WHERE cookie = $1", cookie)
        .fetch_all(pool)
        .await?;
    if cookies.len() == 0 {
        return Ok(false);
    }
    let cookie = cookies[0];
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
    let cookies = query_as!(Cookies, "SELECT * FROM cookies WHERE cookie = $1", cookie)
        .fetch_all(pool)
        .await?;
    if cookies.len() == 0 {
        return Ok(false);
    }
    sqlx::query!("DELETE FROM cookies WHERE cookie = $1", cookie)
        .execute(pool)
        .await?;
    Ok(true)
}

/// Get the cookie by cookie string.
pub async fn get_cookie(pool: &Pool<Postgres>, cookie: &str) -> Result<Cookies, sqlx::Error> {
    let cookies = query_as!(Cookies, "SELECT * FROM cookies WHERE cookie = $1", cookie)
        .fetch_all(pool)
        .await?;
    if cookies.len() == 0 {
        return Err(sqlx::Error::RowNotFound);
    }
    Ok(cookies[0])
}