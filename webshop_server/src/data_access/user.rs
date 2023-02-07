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
