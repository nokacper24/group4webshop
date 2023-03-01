use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Pool, query_as};

#[derive(Deserialize, Serialize)]
pub struct Company {
    company_id: i32,
    company_name: String,
    company_address: String,
}

/// Get all companies from the database.
/// returns a vector of all companies.
/// returns an error if the database query failed.
pub async fn get_all_companies(pool: &Pool<Postgres>) -> Result<Vec<Company>, sqlx::Error> {
    let companies = query_as!(Company, "SELECT * FROM company").fetch_all(pool).await?;
    Ok(companies)
}

/// Get a company by id.
/// returns the company if it was found.
/// returns an error if the database query failed.
/// returns an error if the company was not found.
pub async fn get_company_by_id(
    pool: &Pool<Postgres>,
    company_id: &i32,
) -> Result<Company, sqlx::Error> {
    let company = query_as!(Company, "SELECT * FROM company WHERE company_id = $1", company_id)
        .fetch_one(pool)
        .await?;
    Ok(company)
}

/// Create a new company.
/// returns the company if it was created.
/// returns an error if the database query failed.
/// returns an error if the company was not created.
/// returns an error if the company already exists.
/// returns an error if the company name is empty.
/// returns an error if the company address is empty.
pub async fn create_company(
    pool: &Pool<Postgres>,
    company_name: &str,
    company_address: &str,
) -> Result<Company, sqlx::Error> {
    if company_name.is_empty() {
        return Err(sqlx::Error::ColumnNotFound("Company name cannot be empty".to_string()));
    }
    if company_address.is_empty() {
        return Err(sqlx::Error::ColumnNotFound("Company address cannot be empty".to_string()));
    }
    let company = query_as!(
        Company,
        "INSERT INTO company (company_name, company_address) VALUES ($1, $2) RETURNING *",
        company_name,
        company_address
    )
    .fetch_one(pool)
    .await?;
    Ok(company)
}

/// removes a company from the database by id.
/// returns true if the company was deleted, false if it was not found.
/// returns an error if the database query failed.
/// returns an error if the company was not deleted.
pub async fn delete_company(pool: &Pool<Postgres>, company_id: &i32) -> Result<bool, sqlx::Error> {
    let delete = sqlx::query!("DELETE FROM company WHERE company_id = $1", company_id)
        .execute(pool)
        .await?;

    if delete.rows_affected() == 0 {
        return Ok(false);
    }
    Ok(true)
}