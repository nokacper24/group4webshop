use sqlx::{Pool, Postgres};
pub mod category;
pub mod product;
pub mod user;

/// Creates a connection pool to the database
/// This should be called once and a reference to the pool
/// should be passed to the necessary functions.
///
/// # Arguments
///
/// * `dburl` - A string slice that holds the database url
///
/// # Example use
///
/// ```
/// use sqlx::{Postgres, Pool};
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
