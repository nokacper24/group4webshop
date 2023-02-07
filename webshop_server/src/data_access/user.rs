use serde::{Serialize, Deserialize};
use sqlx::{query_as,{Postgres, Pool}};


pub struct User {
    user_id: i32,
    email: String,
    pass_hash: String,
    company_id: i32,
    role: Role
}

#[derive(sqlx::Type)]
#[sqlx(rename = "role", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Role {
    ADMIN,
    COMPANYITHEAD,
    COMPANYIT,
    DEFAULT,
}

async fn get_all_users(pool: &Pool<Postgres>) -> Result<Vec<User>, sqlx::Error> {
    let users = query_as!(
        User,
        r#"SELECT user_id, email, pass_hash, company_id, role as "role: _" FROM app_user"#
    )
        .fetch_all(pool)
        .await?;
    Ok(users)
}

pub async fn get_user_by_id(pool: &Pool<Postgres>, user_id: &str) -> Result<User, sqlx::Error> {
    let product = query_as!(User, r#"SELECT user_id, email, pass_hash, company_id, role as "role: _" FROM app_user WHERE user_id = $1"#, user_id)
        .fetch_one(pool)
        .await?;
    Ok(product)
}