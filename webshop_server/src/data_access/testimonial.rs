//! Data access layer implementation for testimonials.
//! This module contains all the functions that are used to access and manipulate the testimonial table in the database.
use serde::{Deserialize, Serialize};
use sqlx::{
    query, query_as, {Pool, Postgres},
};
use utoipa::ToSchema;

/// Testimonial struct
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Testimonial {
    testimonial_id: i32,
    author: String,
    text: String,
    author_pic: String,
    product_id: String,
}

impl Testimonial {
    /// Creates a new testimonial
    pub fn new(
        testimonial_id: i32,
        author: String,
        text: String,
        author_pic: String,
        product_id: String,
    ) -> Self {
        Self {
            testimonial_id,
            author,
            text,
            author_pic,
            product_id,
        }
    }
    /// Returns the path to the author's picture
    pub fn author_pic(&self) -> &String {
        &self.author_pic
    }
}

/// Partial testimonial struct, same as testimonial but without the testimonial_id.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PartialTestimonial {
    author: String,
    text: String,
    author_pic: String,
    product_id: String,
}
impl PartialTestimonial {
    /// Creates a new partial testimonial
    pub fn new(author: String, text: String, author_pic: String, product_id: String) -> Self {
        Self {
            author,
            text,
            author_pic,
            product_id,
        }
    }
}

/// Returns all testimonials for a specific product
///
/// # Arguments
///
/// * `pool` - The database connection pool
/// * `product_id` - The id of the product
///
/// # Returns
///
/// * `Result<Vec<Testimonial>, sqlx::Error>` - A vector of testimonials
pub async fn get_testimonials_by_product(
    pool: &Pool<Postgres>,
    product_id: &String,
) -> Result<Vec<Testimonial>, sqlx::Error> {
    let testimonial = query_as!(
        Testimonial,
        r#"SELECT * 
        FROM testimonial 
        WHERE product_id = $1"#,
        product_id
    )
    .fetch_all(pool)
    .await?;
    Ok(testimonial)
}

/// Returns a specific testimonial for a specific product
///
/// # Arguments
///
/// * `pool` - The database connection pool
/// * `product_id` - The id of the product
/// * `testimonial_id` - The id of the testimonial
pub async fn get_testimonial_by_prod_and_id(
    pool: &Pool<Postgres>,
    product_id: &String,
    testimonial_id: &i32,
) -> Result<Testimonial, sqlx::Error> {
    let testimonial = query_as!(
        Testimonial,
        r#"SELECT * 
        FROM testimonial 
        WHERE product_id = $1 AND testimonial_id = $2"#,
        product_id,
        testimonial_id
    )
    .fetch_one(pool)
    .await?;
    Ok(testimonial)
}

/// Creates a new testimonial for a specific product
///
/// # Arguments
///
/// * `pool` - The database connection pool
/// * `testimonial` - The testimonial to be created
pub async fn create_testimonial(
    pool: &Pool<Postgres>,
    testimonial: PartialTestimonial,
) -> Result<Testimonial, sqlx::Error> {
    let testimonial = query_as!(
        Testimonial,
        r#"INSERT INTO testimonial (author, text, author_pic, product_id)
        VALUES ($1, $2, $3, $4)
        RETURNING *"#,
        testimonial.author,
        testimonial.text,
        testimonial.author_pic,
        testimonial.product_id
    )
    .fetch_one(pool)
    .await?;
    Ok(testimonial)
}

/// Update an existing testimonial.
/// Id is taken from the testimonial struct provided.
///
/// # Arguments
///
/// * `pool` - The database connection pool
/// * `testimonial` - The testimonial to be updated
pub async fn update_testimonial(
    pool: &Pool<Postgres>,
    testimonial: Testimonial,
) -> Result<Testimonial, sqlx::Error> {
    let testimonial = query_as!(
        Testimonial,
        r#"UPDATE testimonial
        SET author = $1, text = $2, author_pic = $3, product_id = $4
        WHERE testimonial_id = $5
        RETURNING *"#,
        testimonial.author,
        testimonial.text,
        testimonial.author_pic,
        testimonial.product_id,
        testimonial.testimonial_id
    )
    .fetch_one(pool)
    .await?;
    Ok(testimonial)
}

/// Delete an existing testimonial
///
/// # Arguments
///
/// * `pool` - The database connection pool
/// * `product_id` - The id of the product the testimonial belongs to
/// * `testimonial_id` - The id of the testimonial
pub async fn delete_testimonial(
    pool: &Pool<Postgres>,
    product_id: &str,
    testimonial_id: &i32,
) -> Result<Testimonial, sqlx::Error> {
    let testimonial = query_as!(
        Testimonial,
        r#"DELETE FROM testimonial
        WHERE product_id = $1 AND testimonial_id = $2
        RETURNING *"#,
        product_id,
        testimonial_id
    )
    .fetch_one(pool)
    .await?;
    Ok(testimonial)
}

/// Returns all testimonial author image paths for a specific product.
/// Useful for deleting the images from the file system when deleting a product.
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
    let rows = query!(
        r#"SELECT author_pic
        FROM testimonial
        WHERE product_id = $1"#,
        product_id
    )
    .fetch_all(pool)
    .await?;

    let paths = rows.iter().map(|row| row.author_pic.clone()).collect();
    Ok(paths)
}
