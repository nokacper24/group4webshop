use serde::{Deserialize, Serialize};
use sqlx::{
    query_as, {Pool, Postgres},
};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Testimonial {
    testimonial_id: i32,
    author: String,
    text: String,
    author_pic: String,
    product_id: String,
}

/// Returns all testimonials for a specific product
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
