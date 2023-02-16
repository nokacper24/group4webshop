use serde::{Deserialize, Serialize};
use sqlx::{
    query_as, {Pool, Postgres},
};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct User {
    pub user_id: i32,
    email: String,
    pass_hash: String,
    company_id: i32,
    pub role: Role,
}

#[derive(sqlx::Type, Serialize, Deserialize, Debug)]
#[sqlx(type_name = "role", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Role {
    Admin,
    CompanyItHead,
    CompanyIt,
    Default,
}

pub struct RoleStruct {
    role: Role,
}

pub async fn get_all_users(pool: &Pool<Postgres>) -> Result<Vec<User>, sqlx::Error> {
    let users = query_as!(
        User,
        r#"SELECT user_id, email, pass_hash, company_id, role as "role: _" FROM app_user"#
    )
    .fetch_all(pool)
    .await?;
    Ok(users)
}

pub async fn get_user_by_id(pool: &Pool<Postgres>, user_id: i32) -> Result<User, sqlx::Error> {
    let product = query_as!(User, r#"SELECT user_id, email, pass_hash, company_id, role as "role: _" FROM app_user WHERE user_id = $1"#, user_id)
        .fetch_one(pool)
        .await?;
    Ok(product)
}

pub async fn get_role_by_id(pool: &Pool<Postgres>, user_id: i32) -> Result<Role, sqlx::Error> {
    let role = query_as!(RoleStruct, r#"SELECT role as "role: _" FROM app_user WHERE user_id = $1"#, user_id)
        .fetch_one(pool)
        .await?;
    Ok(role.role)
}