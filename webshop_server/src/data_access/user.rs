use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::{DateTime, Duration, Utc};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use sqlx::{
    query, query_as, Executor, {Pool, Postgres},
};
use std::fmt::Display;
use utoipa::ToSchema;
use uuid::Uuid;

/// User struct with their password hash
/// Use this only if you **need** the password hash.
#[derive(Debug)]
pub struct UserWithPass {
    pub user_id: i32,
    pub email: String,
    pub pass_hash: String,
    pub company_id: i32,
    pub role: Role,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct User {
    pub user_id: i32,
    pub email: String,
    pub company_id: i32,
    pub role: Role,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LicenseUser {
    user_id: i32,
    license_id: i32,
}

#[derive(sqlx::Type, Serialize, Deserialize, Debug, PartialEq, ToSchema)]
#[sqlx(type_name = "role_enum", rename_all = "snake_case")]
pub enum Role {
    Admin,
    CompanyItHead,
    CompanyIt,
    Default,
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::Admin => write!(f, "Admin"),
            Role::CompanyItHead => write!(f, "CompanyItHead"),
            Role::CompanyIt => write!(f, "CompanyIt"),
            Role::Default => write!(f, "Default"),
        }
    }
}

pub struct RoleStruct {
    role: Role,
}

pub async fn get_all_users(pool: &Pool<Postgres>) -> Result<Vec<User>, sqlx::Error> {
    let users = query_as!(
        User,
        r#"SELECT user_id, email, company_id, role as "role: _" FROM app_user"#
    )
    .fetch_all(pool)
    .await?;
    Ok(users)
}

pub async fn get_user_by_id(pool: &Pool<Postgres>, user_id: &i32) -> Result<User, sqlx::Error> {
    let user = query_as!(
        User,
        r#"SELECT user_id, email, company_id, role as "role: _" FROM app_user WHERE user_id = $1"#,
        user_id
    )
    .fetch_one(pool)
    .await?;
    Ok(user)
}

pub async fn get_by_username_with_pass(
    pool: &Pool<Postgres>,
    username: &str,
) -> Result<UserWithPass, sqlx::Error> {
    let user = query_as!(UserWithPass, r#"SELECT user_id, email, pass_hash, company_id, role as "role: _" FROM app_user WHERE email = $1"#, username)
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
        r#"SELECT user_id, email, company_id, role as "role: _" 
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
        r#"SELECT app_user.user_id, email, company_id, role as "role: _"
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
    users: &[LicenseUser],
) -> Result<(), sqlx::Error> {
    let mut transaction = pool.begin().await?;
    for user in users.iter() {
        transaction
            .execute(query!(
                r#"INSERT INTO user_license(license_id, user_id)
                VALUES ($1, $2)"#,
                user.license_id,
                user.user_id,
            ))
            .await?;
    }
    transaction.commit().await?;

    Ok(())
}

/// Remove users' access to licenses
pub async fn remove_license_users(
    pool: &Pool<Postgres>,
    users: &[LicenseUser],
) -> Result<(), sqlx::Error> {
    let mut transaction = pool.begin().await?;
    for user in users.iter() {
        transaction
            .execute(query!(
                r#"DELETE FROM user_license
                WHERE license_id = $1 AND user_id = $2"#,
                user.license_id,
                user.user_id,
            ))
            .await?;
    }
    transaction.commit().await?;

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

pub fn verify(pass: &str, hash: &str) -> Result<bool, argon2::password_hash::Error> {
    //pass to bytes
    let pass = pass.as_bytes();

    // Hash to verify against
    let hash = PasswordHash::new(hash);
    let hash = match hash {
        Ok(hash) => hash,
        Err(e) => return Err(e),
    };

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Verify password against the hash
    match argon2.verify_password(pass, &hash) {
        Ok(_) => Ok(true),
        Err(e) => Err(e),
    }
}

/// A struct to represent a user that is registering themselves and a company.
#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterUser {
    pub id: i32,
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
    let _key = Uuid::new_v4().to_string();
    let exp_date = Utc::now() + Duration::days(1);

    let insert = query!(
        r#"INSERT INTO register_user (email, exp_date)
        VALUES ($1, $2)"#,
        email,
        exp_date
    )
    .execute(pool)
    .await;

    match insert {
        Ok(_) => {
            let user = query_as!(
                RegisterUser,
                r#"SELECT id, email, exp_date FROM register_user WHERE email = $1"#,
                email
            )
            .fetch_one(pool)
            .await?;
            Ok(user)
        }
        Err(e) => Err(e),
    }
}

pub async fn get_partial_user(
    id: &i32,
    pool: &Pool<Postgres>,
) -> Result<RegisterUser, sqlx::Error> {
    let user = query_as!(
        RegisterUser,
        r#"SELECT id, email, exp_date FROM register_user WHERE id = $1"#,
        id
    )
    .fetch_one(pool)
    .await?;
    Ok(user)
}

pub async fn delete_partial_user(id: &i32, pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    let result = query!(r#"DELETE FROM register_user WHERE id = $1"#, id)
        .execute(pool)
        .await;
    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

#[derive(Serialize, Deserialize, Debug)]
/// A struct to represent a user that is registering themselves and linking to a company.
pub struct RegisterCompanyUser {
    pub id: i32,
    pub email: String,
    pub company_id: i32,
    pub exp_date: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PartialRegisterCompanyUser {
    pub email: String,
    pub company_id: i32,
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
    let exp_date = Utc::now() + Duration::days(1);

    let insert = query!(
        r#"INSERT INTO register_company_user (email, company_id, exp_date)
        VALUES ($1, $2, $3)"#,
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
                r#"SELECT id, email, company_id, exp_date FROM register_company_user WHERE email = $1"#,
                email
            )
            .fetch_one(pool)
            .await?;
            Ok(user)
        }
        Err(e) => Err(e),
    }
}

/// Creates new users that is registering themselves and linking to a company.
/// Returns the partial users that were created.
/// # Arguments
/// * `email` - The email of the user
/// * `company_id` - The id of the company the user is registering for
/// * `pool` - The database pool
/// # Returns
/// * `RegisterCompanyUser` - The partial user that was created
pub async fn create_partial_company_users(
    users: &[PartialRegisterCompanyUser],
    pool: &Pool<Postgres>,
) -> Result<Vec<RegisterCompanyUser>, sqlx::Error> {
    let exp_date = Utc::now() + Duration::days(1);

    let mut transaction = pool.begin().await?;
    let mut email_list = Vec::<String>::new();

    for user in users.iter() {
        transaction
            .execute(query!(
                r#"INSERT INTO register_company_user (email, company_id, exp_date)
                VALUES ($1, $2, $3)"#,
                user.email,
                user.company_id,
                exp_date
            ))
            .await?;

        email_list.push(user.email.clone());
    }
    transaction.commit().await?;

    query_as!(
        RegisterCompanyUser,
        r#"SELECT id, email, company_id, exp_date
            FROM register_company_user
            WHERE email = ANY($1)"#,
        &email_list
    )
    .fetch_all(pool)
    .await
}

pub async fn get_partial_company_user(
    id: &i32,
    pool: &Pool<Postgres>,
) -> Result<RegisterCompanyUser, sqlx::Error> {
    let user = query_as!(
        RegisterCompanyUser,
        r#"SELECT id, email, company_id, exp_date FROM register_company_user WHERE id = $1"#,
        id
    )
    .fetch_one(pool)
    .await?;
    Ok(user)
}

/// Deletes a user that is registering themselves and linking to a company.
/// # Arguments
/// * `id` - The id of the user
/// * `pool` - The database pool
/// # Returns
/// * `()` - An empty tuple
/// # Errors
/// * `sqlx::Error` - An error from the database
pub async fn delete_partial_company_user(
    id: &i32,
    pool: &Pool<Postgres>,
) -> Result<(), sqlx::Error> {
    let result = query!(r#"DELETE FROM register_company_user WHERE id = $1"#, id)
        .execute(pool)
        .await;
    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

/// Fetches a user by their email address and returns a boolean indicating if the user exists.
/// # Arguments
/// * `email` - The email of the user
/// * `pool` - The database pool
/// # Returns
/// * `bool` - A boolean indicating if the user exists
/// # Errors
/// * `sqlx::Error` - An error from the database
/// # Example
/// ```rust
/// let user_exists: bool = user_exists("bob@name.com", &pool).await?;
/// ```
pub async fn user_exists(email: &str, pool: &Pool<Postgres>) -> Result<bool, sqlx::Error> {
    let user = query!(r#"SELECT user_id FROM app_user WHERE email = $1"#, email)
        .fetch_optional(pool)
        .await?;

    match user {
        Some(_) => Ok(true),
        None => Ok(false),
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

pub async fn get_invite_by_id(id: &str, pool: &Pool<Postgres>) -> Result<Invite, sqlx::Error> {
    let invite = query_as!(
        Invite,
        r#"SELECT id, user_id, company_user_id FROM invite_user WHERE id = $1"#,
        id
    )
    .fetch_one(pool)
    .await?;
    Ok(invite)
}

pub async fn delete_invite(id: &str, pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    let result = query!(r#"DELETE FROM invite_user WHERE id = $1"#, id)
        .execute(pool)
        .await;
    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

#[derive(Debug)]
pub enum UserCreationError {
    Database(sqlx::Error),
    Hashing(argon2::Error),
    Hashing2(argon2::password_hash::Error),
}

pub async fn create_user(
    email: &str,
    pass: &str,
    company_id: i32,
    role: Role,
    pool: &Pool<Postgres>,
) -> Result<User, UserCreationError> {
    let pass_hash = match hash(pass) {
        Ok(hash) => hash,
        Err(e) => return Err(UserCreationError::Hashing2(e)),
    };

    let insert = query!(
        r#"INSERT INTO app_user (email, pass_hash, company_id, role)
        VALUES ($1, $2, $3, $4)"#,
        email,
        pass_hash,
        company_id,
        role as _
    )
    .execute(pool)
    .await;

    match insert {
        Ok(_) => {
            let user = query_as!(
                User,
                r#"SELECT user_id, email, company_id, role as "role: _"
                FROM app_user
                WHERE email = $1"#,
                email
            )
            .fetch_one(pool)
            .await;
            match user {
                Ok(user) => Ok(user),
                Err(e) => Err(UserCreationError::Database(e)),
            }
        }
        Err(e) => Err(UserCreationError::Database(e)),
    }
}

/// Get all users with a specific role
pub async fn get_users_by_role(
    pool: &Pool<Postgres>,
    role: &Role,
) -> Result<Vec<User>, sqlx::Error> {
    let users = query_as!(
        User,
        r#"SELECT user_id, email, company_id, role as "role: _"
        FROM app_user
        WHERE role = $1"#,
        role as _
    )
    .fetch_all(pool)
    .await?;
    Ok(users)
}

#[derive(Deserialize, Serialize, Debug, ToSchema)]
pub struct UserRole {
    user_id: i32,
    role: Role,
}

/// Update users' roles
pub async fn update_user_roles(
    pool: &Pool<Postgres>,
    users: &[UserRole],
) -> Result<(), sqlx::Error> {
    let mut transaction = pool.begin().await?;
    for user in users.iter() {
        transaction
            .execute(query!(
                r#"UPDATE app_user
                SET role = $1
                WHERE user_id = $2"#,
                user.role as _,
                user.user_id
            ))
            .await?;
    }
    transaction.commit().await?;

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserID {
    user_id: i32,
}

/// Delete users
pub async fn delete_users(pool: &Pool<Postgres>, users: &[UserID]) -> Result<(), sqlx::Error> {
    let mut transaction = pool.begin().await?;
    for user in users.iter() {
        transaction
            .execute(query!(
                r#"DELETE FROM app_user
                   WHERE user_id = $1"#,
                user.user_id,
            ))
            .await?;
    }
    transaction.commit().await?;

    Ok(())
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct UserWithoutHash {
    pub user_id: i32,
    pub email: String,
    pub company_id: i32,
    pub role: Role,
}

pub async fn update_email(
    pool: &Pool<Postgres>,
    email: &str,
    id: &i32,
) -> Result<UserWithoutHash, sqlx::Error> {
    query_as!(
        UserWithoutHash,
        r#"UPDATE app_user 
        SET email = $1
        WHERE user_id = $2
        RETURNING app_user.user_id, email, company_id, role as "role: _";"#,
        email,
        id,
    )
    .fetch_one(pool)
    .await
}
