use serde::{Deserialize, Serialize};
use sqlx::{
    query_as, {Pool, Postgres},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    product_id: String,
    display_name: String,
    price_per_user: f32,
    short_description: String,
    main_image: String,
}

/// Returns all products, in **simple** form, no description components.
pub async fn get_products(pool: &Pool<Postgres>) -> Result<Vec<Product>, sqlx::Error> {
    let products = query_as!(Product, "SELECT * FROM product")
        .fetch_all(pool)
        .await?;
    Ok(products)
}

/// Returns a product, in **simple** form, no description components.
pub async fn get_product_by_id(
    pool: &Pool<Postgres>,
    product_id: &str,
) -> Result<Product, sqlx::Error> {
    let product = query_as!(
        Product,
        "SELECT * FROM product WHERE product_id = $1",
        product_id
    )
    .fetch_one(pool)
    .await?;
    Ok(product)
}
