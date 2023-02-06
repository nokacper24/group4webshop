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

#[derive(Debug, Serialize, Deserialize)]
pub struct FullProduct {
    product_id: String,
    display_name: String,
    price_per_user: f32,
    short_description: String,
    main_image: String,
    description_components: Vec<DescriptionComponent>,
}
/// Returns a full product, including description components.
pub async fn get_full_product_by_id(
    pool: &Pool<Postgres>,
    product_id: &str,
) -> Result<FullProduct, Box<dyn Error>> {
    let product = get_product_by_id(pool, product_id).await?;
    let description_components = get_product_description_components(pool, product_id).await?;

    Ok(FullProduct {
        product_id: product.product_id,
        display_name: product.display_name,
        price_per_user: product.price_per_user,
        short_description: product.short_description,
        main_image: product.main_image,
        description_components,
    })
}


/// Returns all description components for a product.
async fn get_product_description_components(
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
