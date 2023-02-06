use std::error::Error;

use serde::{Deserialize, Serialize};
use sqlx::{
    query, query_as, {Pool, Postgres},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    product_id: String,
    display_name: String,
    price_per_user: f32,
    short_description: String,
    main_image: String,
    available: bool,
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
    fn generate_id(&self) -> String {
        self.display_name
            .to_lowercase()
            .replace(".", "")
            .replace(" ", "_")
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

#[derive(Debug, Serialize, Deserialize)]
pub struct DescriptionComponent {
    priority: i32,
    text: Option<TextComponent>,
    image: Option<ImageComponent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextComponent {
    text_title: String,
    paragraph: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ImageComponent {
    image_path: String,
    alt_text: String,
}

/// Returns all available products.
pub async fn get_products(pool: &Pool<Postgres>) -> Result<Vec<Product>, sqlx::Error> {
    let products = query_as!(Product,
        r#"SELECT *
        FROM product WHERE available = true"#)
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
    product: &PartialProduct,
) -> Result<(), sqlx::Error> {
    query!(
        r#"INSERT INTO product
        (product_id, display_name, price_per_user, short_description, main_image, available)
        VALUES ($1, $2, $3, $4, $5, $6)"#,
        product.generate_id(),
        product.display_name,
        product.price_per_user,
        product.short_description,
        product.main_image,
        product.available
    )
    .execute(pool)
    .await?;
    Ok(())
}

/// Update a product.
pub async fn update_product(
    pool: &Pool<Postgres>,
    product: &Product,
) -> Result<(), sqlx::Error> {
    query!(
        r#"UPDATE product
        SET display_name = $1, price_per_user = $2, short_description = $3, main_image = $4, available = $5
        WHERE product_id = $6"#,
        product.display_name,
        product.price_per_user,
        product.short_description,
        product.main_image,
        product.available,
        product.product_id
    )
    .execute(pool)
    .await?;
    Ok(())
}


/// Returns all description components of a product.
pub async fn get_product_description_components(
    pool: &Pool<Postgres>,
    product_id: &str,
) -> Result<Vec<DescriptionComponent>, Box<dyn Error>> {
    let rows = query!(
        r#"SELECT priority, description_component.text_id AS "text_id?", text_title AS "text_title?", paragraph AS "paragraph?", description_component.image_id, image_path AS "image_path?", alt_text AS "alt_text?"
        FROM description_component
        FULL JOIN product_text ON  description_component.text_id = product_text.text_id
        FULL JOIN product_image ON description_component.image_id = product_image.image_id
        WHERE description_component.product_id = $1"#, product_id)
        .fetch_all(pool)
        .await?;

    let mut description_components: Vec<DescriptionComponent> = Vec::new();

    for row in rows {
        if row.text_id.is_some() {
            match (row.text_title, row.paragraph) {
                (Some(text_title), Some(paragraph)) => {
                    description_components.push(DescriptionComponent {
                        priority: row.priority,
                        text: Some(TextComponent {
                            text_title,
                            paragraph,
                        }),
                        image: None,
                    });
                }
                _ => {}
            }
        } else if row.image_id.is_some() {
            match (row.image_path, row.alt_text) {
                (Some(image_path), Some(alt_text)) => {
                    description_components.push(DescriptionComponent {
                        priority: row.priority,
                        text: None,
                        image: Some(ImageComponent {
                            image_path,
                            alt_text,
                        }),
                    });
                }
                _ => {}
            }
        } else { // Should never happen, db has constraints
            return Err("Could not decode description component, corrupt data.".into());
        }
    }
    return Result::Ok(description_components);
}