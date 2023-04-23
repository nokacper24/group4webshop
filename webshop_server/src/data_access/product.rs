//! This module is a data access implementation for products.  
//! Allows for access and manipulation of products in the database.
//!
//! Its submodule `description` contains the data access implementation for product descriptions.

use serde::{Deserialize, Serialize};
use sqlx::{
    query, query_as, {Pool, Postgres},
};
use utoipa::ToSchema;

use crate::data_access::testimonial;

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
        product_id: &str,
        display_name: &str,
        price_per_user: f32,
        short_description: &str,
        main_image: &str,
        available: bool,
    ) -> Self {
        Self {
            product_id: product_id.to_string(),
            display_name: display_name.to_string(),
            price_per_user,
            short_description: short_description.to_string(),
            main_image: main_image.to_string(),
            available,
        }
    }
    pub fn product_id(&self) -> &str {
        &self.product_id
    }
    pub fn main_image(&self) -> &str {
        &self.main_image
    }
    pub fn available(&self) -> bool {
        self.available
    }
}

/// Generates a product id from a display name.
/// Lowercases the display name, removes dots (.) and replaces spaces with underscores (_).
///
/// # Arguments
///
/// * `display_name` - The product display name to generate the id from.
pub fn generate_id(display_name: &str) -> String {
    display_name
        .to_lowercase()
        .replace('.', "")
        .replace(' ', "_")
}

/// Returns all products, optionally only available products.
///
/// # Arguments
///
/// * `pool` - The database pool
/// * `only_available`
///     - true - only available products are returned, should be used for the public api
///     - false - all products are returned
///
/// # Returns
pub async fn get_products(
    pool: &Pool<Postgres>,
    only_available: bool,
) -> Result<Vec<Product>, sqlx::Error> {
    let products = if only_available {
        query_as!(
            Product,
            r#"SELECT *
            FROM product WHERE available = true"#
        )
        .fetch_all(pool)
        .await?
    } else {
        query_as!(
            Product,
            r#"SELECT *
            FROM product"#
        )
        .fetch_all(pool)
        .await?
    };
    Ok(products)
}

/// Returns a product by its id.
///
/// # Arguments
///
/// * `pool` - The database pool
/// * `product_id` - The id of the product to return
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

/// Create a new product in the database.
///
/// # Arguments
///
/// * `pool` - The database pool
/// * `product` - The product to insert into the database
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
///
/// # Arguments
///
/// * `pool` - The database pool
/// * `product_id` - The id of the product to delete
///
/// # Returns
/// The path to the main image of the product.
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
///
/// # Arguments
///
/// * `pool` - The database pool
/// * `new_product` - The updated product
///
/// # Returns
/// The updated product.
pub async fn update_product(
    pool: &Pool<Postgres>,
    new_product: &Product,
) -> Result<Product, sqlx::Error> {
    query_as!(Product,
        r#"UPDATE product
        SET display_name = $1, price_per_user = $2, short_description = $3, main_image = $4, available = $5
        WHERE product_id = $6
        RETURNING *"#,
        new_product.display_name,
        new_product.price_per_user,
        new_product.short_description,
        new_product.main_image,
        new_product.available,
        new_product.product_id
    )
    .fetch_one(pool)
    .await
}

/// Update the availability of a product.
/// Returns the updated product.
///
/// # Arguments
///
/// * `pool` - The database pool
/// * `product_id` - The id of the product to update
/// * `available` - The new availability
///
/// # Returns
/// The updated product.
pub async fn update_product_available(
    pool: &Pool<Postgres>,
    product_id: &str,
    available: bool,
) -> Result<Product, sqlx::Error> {
    query_as!(
        Product,
        r#"UPDATE product
        SET available = $1
        WHERE product_id = $2
        RETURNING *"#,
        available,
        product_id
    )
    .fetch_one(pool)
    .await
}

/// Returns true if the product exists, false otherwise.
///
/// # Arguments
///
/// * `pool` - The database pool
/// * `product_id` - The id of the product to check
///
/// # Returns
/// true if the product exists, false otherwise.
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

/// Returns all image aths related to the product.
/// Incluides the main image, testimonial author images and description component images.
///
/// # Arguments
///
/// * `pool` - The database connection pool
/// * `product_id` - The id of the product
///
/// # Returns
/// * `Result<Vec<String>, sqlx::Error>` - Vector of image paths.
pub async fn get_all_image_paths(
    pool: &Pool<Postgres>,
    product_id: &str,
) -> Result<Vec<String>, sqlx::Error> {
    // Get main image
    let main_image = query!(
        r#"SELECT main_image
        FROM product
        WHERE product_id = $1"#,
        product_id
    )
    .fetch_one(pool)
    .await?
    .main_image;
    let testimonil_images = testimonial::get_all_image_paths(pool, product_id).await?;
    let description_images = description::get_all_image_paths(pool, product_id).await?;

    let mut images = vec![main_image];
    images.extend(testimonil_images);
    images.extend(description_images);
    Ok(images)
}
