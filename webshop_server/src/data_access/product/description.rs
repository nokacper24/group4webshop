use serde::{Deserialize, Serialize};
use sqlx::{
    query, query_as, {Pool, Postgres},
};
use std::error::Error;

/// Description component. Contains either text or image.
/// Database has constraints to ensure that only one of them is Some.
#[derive(Debug, Serialize, Deserialize)]
pub struct DescriptionComponent {
    component_id: i32,
    priority: i32,
    product_id: String,
    text: Option<TextComponent>,
    image: Option<ImageComponent>,
}

/// Description component without id.
/// Used for creating new components, id is generated by db.
pub struct PartialDescriptionComponent {
    priority: i32,
    text: Option<TextComponent>,
    image: Option<ImageComponent>,
}
impl PartialDescriptionComponent {
    fn get_type(&self) -> ComponentType {
        if self.text.is_some() || self.image.is_none() {
            return ComponentType::Text;
        } else if self.image.is_some() || self.text.is_none() {
            return ComponentType::Image;
        } else {
            return ComponentType::Invalid;
        }
    }
}
enum ComponentType {
    Text,
    Image,
    Invalid,
}

/// Description component error.  
/// Invalid component means that the component has
/// - both Text and image are None  
/// - both Text and image are Some  
///
/// SqlxError is a wrapper for sqlx::Error
enum DescriptionCompError {
    InvalidComponent(String),
    SqlxError(sqlx::Error),
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
) -> Result<Vec<DescriptionComponent>, sqlx::Error> {
    let rows = query!(
        r#"SELECT component_id, priority, product_id,
        description_component.text_id AS "text_id?",
        text_title AS "text_title?", paragraph AS "paragraph?",
        description_component.image_id, image_path AS "image_path?",
        alt_text AS "alt_text?"
        FROM description_component
        FULL JOIN product_text ON  description_component.text_id = product_text.text_id
        FULL JOIN product_image ON description_component.image_id = product_image.image_id
        WHERE description_component.product_id = $1
        ORDER BY priority ASC"#,
        product_id
    )
    .fetch_all(pool)
    .await?;

    let mut description_components: Vec<DescriptionComponent> = Vec::new();

    for row in rows {
        // text component
        if row.text_id.is_some() {
            match (row.text_title, row.paragraph) {
                (Some(text_title), Some(paragraph)) => {
                    description_components.push(DescriptionComponent {
                        component_id: row.component_id,
                        priority: row.priority,
                        product_id: row.product_id,
                        text: Some(TextComponent {
                            text_title,
                            paragraph,
                        }),
                        image: None,
                    });
                }
                _ => {}
            }
        // image component
        } else if row.image_id.is_some() {
            match (row.image_path, row.alt_text) {
                (Some(image_path), Some(alt_text)) => {
                    description_components.push(DescriptionComponent {
                        component_id: row.component_id,
                        priority: row.priority,
                        product_id: row.product_id,
                        text: None,
                        image: Some(ImageComponent {
                            image_path,
                            alt_text,
                        }),
                    });
                }
                _ => {}
            }
        } else {
            // Should never happen, db has constraints
            return Err(sqlx::Error::Decode(
                "Could not decode description component, corrupt data".into(),
            ));
        }
    }
    return Result::Ok(description_components);
}

/// Returns a description component by id.
pub async fn get_description_component(
    pool: &Pool<Postgres>,
    component_id: i32,
) -> Result<DescriptionComponent, sqlx::Error> {
    let row = query!(
        r#"SELECT component_id, priority, product_id,
        description_component.text_id AS "text_id?",
        text_title AS "text_title?", paragraph AS "paragraph?",
        description_component.image_id, image_path AS "image_path?",
        alt_text AS "alt_text?"
        FROM description_component
        FULL JOIN product_text ON  description_component.text_id = product_text.text_id
        FULL JOIN product_image ON description_component.image_id = product_image.image_id
        WHERE description_component.component_id = $1"#,
        component_id
    )
    .fetch_one(pool)
    .await?;

    // text component
    if row.text_id.is_some() {
        match (row.text_title, row.paragraph) {
            (Some(text_title), Some(paragraph)) => {
                return Result::Ok(DescriptionComponent {
                    component_id: row.component_id,
                    priority: row.priority,
                    product_id: row.product_id,
                    text: Some(TextComponent {
                        text_title,
                        paragraph,
                    }),
                    image: None,
                });
            }
            _ => {}
        }
    // image component
    } else if row.image_id.is_some() {
        match (row.image_path, row.alt_text) {
            (Some(image_path), Some(alt_text)) => {
                return Result::Ok(DescriptionComponent {
                    component_id: row.component_id,
                    priority: row.priority,
                    product_id: row.product_id,
                    text: None,
                    image: Some(ImageComponent {
                        image_path,
                        alt_text,
                    }),
                });
            }
            _ => {}
        }
    } else {
        // Should never happen, db has constraints
        return Err(sqlx::Error::Decode(
            "Could not decode description component, corrupt data".into(),
        ));
    }
    return Err(sqlx::Error::Decode(
        "Could not decode description component, corrupt data".into(),
    ));
}

/// Creates a new description component, and returns newly created component.
pub async fn create_component(
    pool: &Pool<Postgres>,
    product_id: &str,
    component: PartialDescriptionComponent,
) -> Result<DescriptionComponent, DescriptionCompError> {
    match component.get_type() {
        ComponentType::Text => {
            let text_id = create_text_component(pool, &component).await?;
            let component_id = query!(
                r#"INSERT INTO description_component (priority, product_id, text_id)
            VALUES ($1, $2, $3)
            RETURNING component_id"#,
                component.priority,
                product_id,
                text_id
            )
            .fetch_one(pool)
            .await;

            let component_id = match component_id {
                Ok(component_id) => component_id,
                Err(err) => return Err(DescriptionCompError::SqlxError(err)),
            };

            Ok(DescriptionComponent {
                component_id: component_id.component_id,
                priority: component.priority,
                product_id: product_id.to_string(),
                text: component.text,
                image: None,
            })
        }
        ComponentType::Image => {
            let image_id = create_image_component(pool, &component).await?;
            let component_id = query!(
                r#"INSERT INTO description_component (priority, product_id, image_id)
            VALUES ($1, $2, $3)
            RETURNING component_id"#,
                component.priority,
                product_id,
                image_id
            )
            .fetch_one(pool)
            .await;
            let component_id = match component_id {
                Ok(component_id) => component_id,
                Err(err) => return Err(DescriptionCompError::SqlxError(err)),
            };

            Ok(DescriptionComponent {
                component_id: component_id.component_id,
                priority: component.priority,
                product_id: product_id.to_string(),
                text: None,
                image: component.image,
            })
        }
        ComponentType::Invalid => {
            return Err(DescriptionCompError::InvalidComponent(
                "Invalid component".into(),
            ))
        }
    }
}

/// Creates a new text component and returns the id.
async fn create_text_component(
    pool: &Pool<Postgres>,
    component: &PartialDescriptionComponent,
) -> Result<i32, DescriptionCompError> {
    if let Some(text_component) = &component.text {
        let row = query!(
            r#"INSERT INTO product_text (text_title, paragraph)
        VALUES ($1, $2)
        RETURNING text_id"#,
            text_component.text_title,
            text_component.paragraph
        )
        .fetch_one(pool)
        .await;

        let row = match row {
            Ok(row) => row,
            Err(err) => return Err(DescriptionCompError::SqlxError(err)),
        };

        return Result::Ok(row.text_id);
    } else {
        return Err(DescriptionCompError::InvalidComponent(
            "Invalid text component.".into(),
        ));
    }
}

/// Creates a new image component and returns the id.
async fn create_image_component(
    pool: &Pool<Postgres>,
    component: &PartialDescriptionComponent,
) -> Result<i32, DescriptionCompError> {
    if let Some(image_component) = &component.image {
        let row = query!(
            r#"INSERT INTO product_image (image_path, alt_text)
        VALUES ($1, $2)
        RETURNING image_id"#,
            image_component.image_path,
            image_component.alt_text
        )
        .fetch_one(pool)
        .await;

        let row = match row {
            Ok(row) => row,
            Err(err) => return Err(DescriptionCompError::SqlxError(err)),
        };

        return Result::Ok(row.image_id);
    } else {
        return Err(DescriptionCompError::InvalidComponent(
            "Invalid image component.".into(),
        ));
    }
}
