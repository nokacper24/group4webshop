use serde::{Serialize, Deserialize};
use sqlx::{query_as,{Postgres, Pool}};

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    product_name: String,
    price_per_user: f32,
    short_description: String,
    main_image: String,
}
impl Product {
    pub fn new(
        product_name: String,
        price_per_user: f32,
        short_description: String,
        main_image: String,
    ) -> Self {
        Product {
            product_name,
            price_per_user,
            short_description,
            main_image,
        }
    }
    pub fn product_name(&self) -> &String {
        &self.product_name
    }
    pub fn price_per_user(&self) -> &f32 {
        &self.price_per_user
    }
    pub fn short_description(&self) -> &String {
        &self.short_description
    }
    pub fn main_image(&self) -> &String {
        &self.main_image
    }
}

pub async fn get_products(pool: &Pool<Postgres>) -> Result<Vec<Product>, sqlx::Error> {
    let products = query_as!(Product, "SELECT * FROM product")
        .fetch_all(pool)
        .await?;
    Ok(products)
}

pub async fn get_product_by_name(pool: &Pool<Postgres>, product_name: &str) -> Result<Product, sqlx::Error> {
    let product = query_as!(Product, "SELECT * FROM product WHERE product_name = $1", product_name)
        .fetch_one(pool)
        .await?;
    Ok(product)
}