use chrono::{DateTime, Duration, Utc, NaiveDate};
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

/// Returns Auth Cookies by cookie, and checks if it is valid.
pub async fn is_valid_cookie(pool: &Pool<Postgres>, cookie: &str) -> Result<bool, sqlx::Error> {
    let cookie = query_as!(
        Cookie,
        "SELECT * FROM cookies WHERE cookie = $1 LIMIT 1",
        cookie
    )
    .fetch_one(pool)
    .await?;

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


pub async fn create_invite(
    pool: &Pool<Postgres>,
    email: &str,
) -> Result<UserInvite, sqlx::Error> {
    // create a 64 character long key based on A-Z, a-z, 0-9. wihtout using uuid.
    let mut key = String::new();
    for _ in 0..64 {
        //generate a random ASCII character
        let ascii = rand::random::<u8>() % 26 + 65;
        key.push(ascii as char);
    }

    let exp_date = Utc::now() + Duration::days(1);
    let invite = sqlx::query_as!(
        UserInvite,
        "INSERT INTO register_user (key, email, exp_date) VALUES ($1, $2, $3) RETURNING *",
        key,
        email,
        exp_date
    )
    .fetch_one(pool)
    .await?;
    Ok(invite)
}

pub async fn get_invite(pool: &Pool<Postgres>, key: &str) -> Result<UserInvite, sqlx::Error> {
    let invite = sqlx::query_as!(
        UserInvite,
        "SELECT * FROM register_user WHERE key = $1 LIMIT 1",
        key
    )
    .fetch_one(pool)
    .await?;
    if invite.exp_date < Utc::now() {
        sqlx::query!("DELETE FROM register_user WHERE key = $1", key)
            .execute(pool)
            .await?;
        return Err(sqlx::Error::RowNotFound);
    }
    Ok(invite)
}