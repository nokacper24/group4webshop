//! This module contains all the database access functions for the webshop server.
//! 
//! The functions are split into separate submodules, each containing functions for a specific data structure.
//! (Not necessarily a table in the database, but a logical grouping of data,
//! e.g. product descriptions have separate tables for different description types, but are grouped together in the descriptions module.)
use sqlx::{Pool, Postgres};
pub mod auth;
pub mod category;
pub mod company;
pub mod error_handling;
pub mod license;
pub mod product;
pub mod testimonial;
pub mod user;

/// Creates a connection pool to the database
/// This should be called once and a reference to the pool
/// should be passed to the necessary functions.
///
/// # Arguments
///
/// * `dburl` - Postgres database connection url
///
/// # Example use
///
/// ```
/// use sqlx::{Postgres, Pool};
/// use crate::data_access::product::Product;
///
/// pub async fn get_products(pool: &Pool<Postgres>) -> Result<Vec<Product>, sqlx::Error> {
///     let products = query_as!(Product, "SELECT * FROM product")
///     .fetch_all(pool)
///     .await?;
///     Ok(products)
/// }
/// ```
pub async fn create_pool(dburl: &str) -> Result<Pool<Postgres>, sqlx::Error> {
    sqlx::PgPool::connect(dburl).await
}
