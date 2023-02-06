use std::error::Error;
use serde::{Deserialize, Serialize};
use sqlx::{
    query, query_as, {Pool, Postgres},
};


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