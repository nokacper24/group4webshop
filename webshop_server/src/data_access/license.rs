use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{
    query, query_as, {Pool, Postgres},
};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct License {
    license_id: i32,
    valid: bool,
    #[schema(value_type = String)]
    start_date: DateTime<Utc>,
    #[schema(value_type = String)]
    end_date: DateTime<Utc>,
    amount: i32,
    company_id: i32,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct LicenseVitalInfo {
    license_id: i32,
    company_id: i32,
    company_name: String,
    product_id: String,
    display_name: String,
    valid: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InvalidLicense {
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

/// Returns vital info from all licenses
pub async fn get_licenses_vital_info(
    pool: &Pool<Postgres>,
) -> Result<Vec<LicenseVitalInfo>, sqlx::Error> {
    let licenses = query_as!(
        LicenseVitalInfo,
        r#"SELECT license_id, license.company_id, company_name, license.product_id, display_name, valid
        FROM license
        JOIN product USING (product_id)
        JOIN company USING (company_id)"#
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
pub async fn get_licenses_by_company(
    pool: &Pool<Postgres>,
    company_id: &i32,
) -> Result<Vec<License>, sqlx::Error> {
    let licenses = query_as!(
        License,
        r#"SELECT * FROM license WHERE company_id = $1"#,
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
) -> Result<(), sqlx::Error> {
    query!(
        r#"INSERT INTO license
        (valid, start_date, end_date, amount, company_id, product_id)
        VALUES ($1, $2, $3, $4, $5, $6)"#,
        license.valid,
        license.start_date,
        license.end_date,
        license.amount,
        license.company_id,
        license.product_id,
    )
    .execute(pool)
    .await?;
    Ok(())
}

/// Update the validation of licenses
pub async fn update_license_validations(
    pool: &Pool<Postgres>,
    licenses: &Vec<InvalidLicense>,
) -> Result<(), sqlx::Error> {
    for license in licenses.iter() {
        query!(
            r#"UPDATE license
            SET valid = $1
            WHERE license_id = $2"#,
            license.valid,
            license.license_id,
        )
        .execute(pool)
        .await?;
    }
    Ok(())
}
