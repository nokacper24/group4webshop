use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::{DateTime, Duration, Utc};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use sqlx::{
    query, query_as, {Pool, Postgres},
};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct User {
    pub user_id: i32,
    pub email: String,
    pub pass_hash: String,
    pub company_id: i32,
    pub role: Role,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LicenseUser {
    user_id: i32,
    license_id: i32,
}

#[derive(sqlx::Type, Serialize, Deserialize, Debug, PartialEq)]
#[sqlx(type_name = "role_enum", rename_all = "snake_case")]
pub enum Role {
    Admin,
    CompanyItHead,
    CompanyIt,
    Default,
}

impl Role {
    pub fn to_string(&self) -> String {
        match self {
            Role::Admin => "Admin".to_string(),
            Role::CompanyItHead => "CompanyItHead".to_string(),
            Role::CompanyIt => "CompanyIt".to_string(),
            Role::Default => "Default".to_string(),
        }
    }
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
    let user = query_as!(User, r#"SELECT user_id, email, pass_hash, company_id, role as "role: _" FROM app_user WHERE user_id = $1"#, user_id)
        .fetch_one(pool)
        .await?;
    Ok(user)
}

pub async fn get_user_by_username(
    pool: &Pool<Postgres>,
    username: &str,
) -> Result<User, sqlx::Error> {
    let user = query_as!(User, r#"SELECT user_id, email, pass_hash, company_id, role as "role: _" FROM app_user WHERE email = $1"#, username)
        .fetch_one(pool)
        .await?;
    Ok(user)
}

/// Returns all users for a company
pub async fn get_users_by_company(
    pool: &Pool<Postgres>,
    company_id: &i32,
) -> Result<Vec<User>, sqlx::Error> {
    let users = query_as!(
        User,
        r#"SELECT user_id, email, pass_hash, company_id, role as "role: _" 
        FROM app_user 
        WHERE company_id = $1"#,
        company_id
    )
    .fetch_all(pool)
    .await?;
    Ok(users)
}

/// Returns all the users for a license
pub async fn get_users_by_license(
    pool: &Pool<Postgres>,
    license_id: &i32,
) -> Result<Vec<User>, sqlx::Error> {
    let users = query_as!(
        User,
        r#"SELECT app_user.user_id, email, pass_hash, company_id, role as "role: _"
        FROM app_user
        INNER JOIN user_license USING (user_id)
        WHERE license_id = $1"#,
        license_id
    )
    .fetch_all(pool)
    .await?;
    Ok(users)
}

pub async fn get_role_by_id(pool: &Pool<Postgres>, user_id: i32) -> Result<Role, sqlx::Error> {
    let role = query_as!(
        RoleStruct,
        r#"SELECT role as "role: _" FROM app_user WHERE user_id = $1"#,
        user_id
    )
    .fetch_one(pool)
    .await?;
    Ok(role.role)
}

/// Give users' access to licenses
pub async fn add_license_users(
    pool: &Pool<Postgres>,
    users: &Vec<LicenseUser>,
) -> Result<(), sqlx::Error> {
    for user in users.iter() {
        query!(
            r#"INSERT INTO user_license(license_id, user_id)
            VALUES ($1, $2)"#,
            user.license_id,
            user.user_id,
        )
        .execute(pool)
        .await?;
    }
    Ok(())
}

/// Remove users' access to licenses
pub async fn remove_license_users(
    pool: &Pool<Postgres>,
    users: &Vec<LicenseUser>,
) -> Result<(), sqlx::Error> {
    for user in users.iter() {
        query!(
            r#"DELETE FROM user_license
            WHERE license_id = $1 AND user_id = $2"#,
            user.license_id,
            user.user_id,
        )
        .execute(pool)
        .await?;
    }
    Ok(())
}

pub fn hash(pass: &str) -> Result<String, argon2::password_hash::Error> {
    //pass to bytes
    let pass = pass.as_bytes();

    // Generate a random salt
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    match argon2.hash_password(pass, &salt) {
        Ok(hash) => Ok(hash.to_string()),
        Err(e) => Err(e),
    }
}

fn verify(pass: &str, hash: &str) -> Result<bool, argon2::password_hash::Error> {
    //pass to bytes
    let pass = pass.as_bytes();

    // Parse the hash string
    let parsed_hash = PasswordHash::new(hash)?;

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Verify password
    match argon2.verify_password(pass, &parsed_hash) {
        Ok(_) => Ok(true),
        Err(e) => Err(e),
    }
}

/// A struct to represent a user that is registering themselves and a company.
pub struct RegisterUser {
    pub id: i32,
    pub key: String,
    pub email: String,
    pub exp_date: DateTime<Utc>,
}

/// Creates a new user that is registering themselves and a company.
/// Returns the partial user that was created.
/// # Arguments
/// * `email` - The email of the user
/// * `pool` - The database pool
/// # Returns
/// * `RegisterUser` - The partial user that was created
pub async fn create_partial_user(
    email: &str,
    pool: &Pool<Postgres>,
) -> Result<RegisterUser, sqlx::Error> {
    let key = Uuid::new_v4().to_string();
    let exp_date = Utc::now() + Duration::days(1);

    let insert = query!(
        r#"INSERT INTO register_user (key, email, exp_date)
        VALUES ($1, $2, $3)"#,
        key,
        email,
        exp_date
    )
    .execute(pool)
    .await;

    match insert {
        Ok(_) => {
            let user = query_as!(
                RegisterUser,
                r#"SELECT id, key, email, exp_date FROM register_user WHERE email = $1"#,
                email
            )
            .fetch_one(pool)
            .await?;
            Ok(user)
        }
        Err(e) => Err(e),
    }
}

/// A struct to represent a user that is registering themselves and linking to a company.
pub struct RegisterCompanyUser {
    pub id: i32,
    pub key: String,
    pub email: String,
    pub company_id: i32,
    pub exp_date: DateTime<Utc>,
}

/// Creates a new user that is registering themselves and linking to a company.
/// Returns the partial user that was created.
/// # Arguments
/// * `email` - The email of the user
/// * `company_id` - The id of the company the user is registering for
/// * `pool` - The database pool
/// # Returns
/// * `RegisterCompanyUser` - The partial user that was created
pub async fn create_partial_company_user(
    email: &str,
    company_id: i32,
    pool: &Pool<Postgres>,
) -> Result<RegisterCompanyUser, sqlx::Error> {
    let key = Uuid::new_v4().to_string();
    let exp_date = Utc::now() + Duration::days(1);

    let insert = query!(
        r#"INSERT INTO register_company_user (key, email, company_id, exp_date)
        VALUES ($1, $2, $3, $4)"#,
        key,
        email,
        company_id,
        exp_date
    )
    .execute(pool)
    .await;

    match insert {
        Ok(_) => {
            let user = query_as!(
                RegisterCompanyUser,
                r#"SELECT id, key, email, company_id, exp_date FROM register_company_user WHERE email = $1"#,
                email
            )
            .fetch_one(pool)
            .await?;
            Ok(user)
        }
        Err(e) => Err(e),
    }
}

/// A struct representing an invite to a new user and a company.
/// This is used to create a new user and link them to a new or existing company.
///
/// # Fields
/// * `id` - The id of the invite
/// * `user_id` - The id of the user that is being invited
/// * `company_user_id` - The id of the company user that is inviting the user
#[derive(Debug, Serialize, Deserialize)]
pub struct Invite {
    pub id: String,
    pub user_id: Option<i32>,
    pub company_user_id: Option<i32>,
}

pub async fn create_invite(
    user_id: Option<i32>,
    company_user_id: Option<i32>,
    pool: &Pool<Postgres>,
) -> Result<Invite, sqlx::Error> {
    let id = Uuid::new_v4().to_string();

    let insert = query!(
        r#"INSERT INTO invite_user (id, user_id, company_user_id)
        VALUES ($1, $2, $3)"#,
        id,
        user_id,
        company_user_id
    )
    .execute(pool)
    .await;

    match insert {
        Ok(_) => {
            let invite = query_as!(
                Invite,
                r#"SELECT id, user_id, company_user_id FROM invite_user WHERE id = $1"#,
                id
            )
            .fetch_one(pool)
            .await?;
            Ok(invite)
        }
        Err(e) => Err(e),
    }
}

/// Get all users that are IT responsible for a company
pub async fn get_users_by_role(
    pool: &Pool<Postgres>,
    role: &Role,
) -> Result<Vec<User>, sqlx::Error> {
    let users = query_as!(
        User,
        r#"SELECT user_id, email, pass_hash, company_id, role as "role: _"
        FROM app_user
        WHERE role = $1"#,
        role as _
    )
    .fetch_all(pool)
    .await?;
    Ok(users)
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserRole {
    user_id: i32,
    role: Role,
}

/// Update users' roles
pub async fn update_user_roles(
    pool: &Pool<Postgres>,
    users: &Vec<UserRole>,
) -> Result<(), sqlx::Error> {
    for user in users.iter() {
        query!(
            r#"UPDATE app_user
            SET role = $1
            WHERE user_id = $2"#,
            user.role as _,
            user.user_id
        )
        .execute(pool)
        .await?;
    }
    Ok(())
}
