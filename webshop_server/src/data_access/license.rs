use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::{
    query_as, {Pool, Postgres},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct License {
    license_id: i32,
    valid: bool,
    start_date: NaiveDate,
    end_date: NaiveDate,
    amount: i32,
    company_id: i32,
    product_id: String,
}

/// Returns all licenses.
pub async fn get_licenses(pool: &Pool<Postgres>) -> Result<Vec<License>, sqlx::Error> {
    let licenses = query_as!(License, "SELECT * FROM license")
        .fetch_all(pool)
        .await?;
    Ok(licenses)
}

/// Returns a license.
pub async fn get_license_by_id(pool: &Pool<Postgres>, id: &i32) -> Result<License, sqlx::Error> {
    let license = query_as!(License, "SELECT * FROM license WHERE license_id = $1", id)
        .fetch_one(pool)
        .await?;
    Ok(license)
}
