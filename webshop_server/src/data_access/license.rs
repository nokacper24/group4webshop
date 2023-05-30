use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{
    query, query_as, Executor, {Pool, Postgres},
};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct License {
    license_id: i32,
    valid: bool,
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
    amount: i32,
    pub company_id: i32,
    product_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartialLicense {
    valid: bool,
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
    amount: i32,
    company_id: i32,
    product_id: String,
}
impl PartialLicense {
    pub fn company_id(&self) -> i32 {
        self.company_id
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FullLicenseInfo {
    license_id: i32,
    valid: bool,
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
    amount: i32,
    company_id: i32,
    product_id: String,
    company_name: String,
    display_name: String,
    active_users: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LicenseValidation {
    license_id: i32,
    valid: bool,
}

/// Returns all info from all licenses
pub async fn get_licenses(pool: &Pool<Postgres>) -> Result<Vec<License>, sqlx::Error> {
    let licenses = query_as!(
        License,
        r#"SELECT *
        FROM license"#
    )
    .fetch_all(pool)
    .await?;
    Ok(licenses)
}

/// Returns full info from all licenses
pub async fn get_licenses_full(pool: &Pool<Postgres>) -> Result<Vec<FullLicenseInfo>, sqlx::Error> {
    let licenses = query_as!(
        FullLicenseInfo,
        r#"SELECT license_id, valid, start_date, end_date, amount, company_id, product_id, company_name, display_name, coalesce(cast(active_users as INTEGER), 0) AS active_users
        FROM license
        JOIN product USING (product_id)
        JOIN company USING (company_id)
        LEFT JOIN (
            SELECT count(*) AS active_users, license_id
            FROM user_license
            GROUP BY license_id
        ) AS count USING (license_id)"#
    )
    .fetch_all(pool)
    .await?;
    Ok(licenses)
}

/// Returns a license
pub async fn get_license_by_id(
    pool: &Pool<Postgres>,
    license_id: &i32,
) -> Result<License, sqlx::Error> {
    let license = query_as!(
        License,
        r#"SELECT * FROM license WHERE license_id = $1"#,
        license_id
    )
    .fetch_one(pool)
    .await?;
    Ok(license)
}

/// Returns all licenses for a company
pub async fn get_licenses_full_by_company(
    pool: &Pool<Postgres>,
    company_id: &i32,
) -> Result<Vec<FullLicenseInfo>, sqlx::Error> {
    let licenses = query_as!(
        FullLicenseInfo,
        r#"SELECT license_id, valid, start_date, end_date, amount, company_id, product_id, company_name, display_name, coalesce(cast(active_users as INTEGER), 0) AS active_users
        FROM license
        JOIN product USING (product_id)
        JOIN company USING (company_id)
        LEFT JOIN (
            SELECT count(*) AS active_users, license_id
            FROM user_license
            GROUP BY license_id
        ) AS count USING (license_id)
        WHERE company_id = $1"#,
        company_id
    )
    .fetch_all(pool)
    .await?;

    Ok(licenses)
}

/// Create a license
pub async fn create_license(
    pool: &Pool<Postgres>,
    license: &PartialLicense,
) -> Result<License, sqlx::Error> {
    query_as!(
        License,
        r#"INSERT INTO license
        (valid, start_date, end_date, amount, company_id, product_id)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *"#,
        license.valid,
        license.start_date,
        license.end_date,
        license.amount,
        license.company_id,
        license.product_id,
    )
    .fetch_one(pool)
    .await
}

/// Update the validation of licenses
pub async fn update_license_validations(
    pool: &Pool<Postgres>,
    licenses: &[LicenseValidation],
) -> Result<(), sqlx::Error> {
    let mut transaction = pool.begin().await?;
    for license in licenses.iter() {
        transaction
            .execute(query!(
                r#"UPDATE license
                SET valid = $1
                WHERE license_id = $2"#,
                license.valid,
                license.license_id,
            ))
            .await?;
    }
    transaction.commit().await?;

    Ok(())
}

/// Returns all licenses that a user has access to
pub async fn get_licenses_for_user(
    pool: &Pool<Postgres>,
    user_id: &i32,
) -> Result<Vec<FullLicenseInfo>, sqlx::Error> {
    let licenses = query_as!(
        FullLicenseInfo,
        r#"SELECT license_id, valid, start_date, end_date, amount, company_id, product_id, company_name, display_name, coalesce(cast(active_users as INTEGER), 0) AS active_users
        FROM license
        JOIN product USING (product_id)
        JOIN company USING (company_id)
        JOIN (
            SELECT count(*) AS active_users, license_id
                FROM user_license
                WHERE license_id IN (
                    SELECT license_id
                    FROM user_license
                    WHERE user_id = $1
                )
            GROUP BY license_id
            ) AS count USING (license_id)"#,
        user_id
    )
    .fetch_all(pool)
    .await?;

    Ok(licenses)
}

/// Returns all of a user's company's licenses that the user has no access to
pub async fn get_licenses_for_user_no_access(
    pool: &Pool<Postgres>,
    company_id: &i32,
    user_id: &i32,
) -> Result<Vec<FullLicenseInfo>, sqlx::Error> {
    let licenses = query_as!(
        FullLicenseInfo,
        r#"SELECT license_id, valid, start_date, end_date, amount, company_id, product_id, company_name, display_name, coalesce(cast(active_users as INTEGER), 0) AS active_users
        FROM license
        JOIN product USING (product_id)
        JOIN company USING (company_id)
        LEFT JOIN (
            SELECT count(*) AS active_users, license_id
            FROM user_license
            GROUP BY license_id
        ) AS count USING (license_id)
        WHERE company_id = $1
        AND license_id NOT IN(
            SELECT license_id
            FROM license
            JOIN user_license USING (license_id)
            WHERE user_id = $2)
        "#,
        company_id,
        user_id
    )
    .fetch_all(pool)
    .await?;

    Ok(licenses)
}
