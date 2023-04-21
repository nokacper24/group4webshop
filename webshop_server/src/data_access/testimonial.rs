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

impl Testimonial {
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
    pub fn testimonial_id(&self) -> i32 {
        self.testimonial_id
    }
    pub fn author(&self) -> &String {
        &self.author
    }
    pub fn text(&self) -> &String {
        &self.text
    }
    pub fn author_pic(&self) -> &String {
        &self.author_pic
    }
    pub fn product_id(&self) -> &String {
        &self.product_id
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PartialTestimonial {
    author: String,
    text: String,
    author_pic: String,
    product_id: String,
}
impl PartialTestimonial {
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

/// Update an existing testimonial
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
