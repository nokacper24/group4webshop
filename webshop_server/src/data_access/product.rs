use serde::{Deserialize, Serialize};
use sqlx::{
    query, query_as, {Pool, Postgres},
};
use utoipa::ToSchema;

pub mod description;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Product {
    product_id: String,
    display_name: String,
    price_per_user: f32,
    short_description: String,
    main_image: String,
    available: bool,
}
impl Product {
    pub fn new(
        product_id: String,
        display_name: String,
        price_per_user: f32,
        short_description: String,
        main_image: String,
        available: bool,
    ) -> Self {
        Self {
            product_id,
            display_name,
            price_per_user,
            short_description,
            main_image,
            available,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartialProduct {
    display_name: String,
    price_per_user: f32,
    short_description: String,
    main_image: String,
    available: bool,
}
impl PartialProduct {
    pub fn new(
        display_name: String,
        price_per_user: f32,
        short_description: String,
        main_image: String,
        available: bool,
    ) -> Self {
        Self {
            display_name,
            price_per_user,
            short_description,
            main_image,
            available,
        }
    }
    fn generate_id(&self) -> String {
        self.display_name
            .to_lowercase()
            .replace('.', "")
            .replace(' ', "_")
    }
    fn into_product(self) -> Product {
        Product {
            product_id: self.generate_id(),
            display_name: self.display_name,
            price_per_user: self.price_per_user,
            short_description: self.short_description,
            main_image: self.main_image,
            available: self.available,
        }
    }
}

pub fn generate_id(display_name: &str) -> String {
    display_name
        .to_lowercase()
        .replace('.', "")
        .replace(' ', "_")
}

/// Returns all available products.
pub async fn get_products(pool: &Pool<Postgres>) -> Result<Vec<Product>, sqlx::Error> {
    let products = query_as!(
        Product,
        r#"SELECT *
        FROM product WHERE available = true"#
    )
    .fetch_all(pool)
    .await?;
    Ok(products)
}

/// Returns a product.
pub async fn get_product_by_id(
    pool: &Pool<Postgres>,
    product_id: &str,
) -> Result<Product, sqlx::Error> {
    let product = query_as!(
        Product,
        r#"SELECT product_id, display_name, price_per_user, short_description, main_image, available
        FROM product WHERE product_id = $1"#,
        product_id
    )
    .fetch_one(pool)
    .await?;
    Ok(product)
}

/// Create a new product.
pub async fn create_product(
    pool: &Pool<Postgres>,
    product: Product,
) -> Result<Product, sqlx::Error> {
    query!(
        r#"INSERT INTO product
        (product_id, display_name, price_per_user, short_description, main_image, available)
        VALUES ($1, $2, $3, $4, $5, $6)"#,
        product.product_id,
        product.display_name,
        product.price_per_user,
        product.short_description,
        product.main_image,
        product.available
    )
    .execute(pool)
    .await?;
    Ok(product)
}

/// Delete a product.
/// Returns path to the main image of the product, so it can be deleted.
pub async fn delete_product(
    pool: &Pool<Postgres>,
    product_id: &str,
) -> Result<String, sqlx::Error> {
    let row = query!(
        r#"DELETE FROM product
        WHERE product_id = $1
        RETURNING product.main_image;"#,
        product_id
    )
    .fetch_one(pool)
    .await?;

    Ok(row.main_image)
}

/// Update a product.
pub async fn update_product(
    pool: &Pool<Postgres>,
    new_product: &Product,
) -> Result<(), sqlx::Error> {
    query!(
        r#"UPDATE product
        SET display_name = $1, price_per_user = $2, short_description = $3, main_image = $4, available = $5
        WHERE product_id = $6"#,
        new_product.display_name,
        new_product.price_per_user,
        new_product.short_description,
        new_product.main_image,
        new_product.available,
        new_product.product_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

/// Returns true if the product exists, false otherwise.
pub async fn product_exists(pool: &Pool<Postgres>, product_id: &str) -> Result<bool, sqlx::Error> {
    let product = query!(
        r#"SELECT product_id
        FROM product WHERE product_id = $1"#,
        product_id
    )
    .fetch_optional(pool)
    .await?;
    Ok(product.is_some())
}
