use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::{
    query_as, {Pool, Postgres},
};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct License {
    license_id: i32,
    valid: bool,
    #[schema(value_type = String)]
    start_date: NaiveDate,
    #[schema(value_type = String)]
    end_date: NaiveDate,
    amount: i32,
    company_id: i32,
    product_id: String,
}

/// Returns all licenses
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
