use serde::{Deserialize, Serialize};
use sqlx::{
    query, Executor, {Pool, Postgres},
};

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
#[derive(Debug, Serialize, Deserialize)]
pub struct PartialDescriptionComponent {
    text: Option<TextComponent>,
    image: Option<ImageComponent>,
}
impl PartialDescriptionComponent {
    fn get_type(&self) -> ComponentType {
        if self.text.is_some() || self.image.is_none() {
            ComponentType::Text
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
pub enum DescriptionCompError {
    InvalidComponent(String),
    SqlxError(sqlx::Error),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextComponent {
    text_id: Option<i32>,
    text_title: String,
    paragraph: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ImageComponent {
    image_id: Option<i32>,
    image_path: String,
    alt_text: String,
}

impl ImageComponent {
    pub fn new(image_id: Option<i32>, image_path: String, alt_text: String) -> Self {
        Self {
            image_id,
            image_path,
            alt_text,
        }
    }
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
        ORDER BY priority ASC;"#,
        product_id
    )
    .fetch_all(pool)
    .await?;

    let mut description_components: Vec<DescriptionComponent> = Vec::new();

    for row in rows {
        // text component
        if row.text_id.is_some() {
            if let (Some(text_id), Some(text_title), Some(paragraph)) =
                (Some(row.text_id), row.text_title, row.paragraph)
            {
                description_components.push(DescriptionComponent {
                    component_id: row.component_id,
                    priority: row.priority,
                    product_id: row.product_id,
                    text: Some(TextComponent {
                        text_id,
                        text_title,
                        paragraph,
                    }),
                    image: None,
                });
            }
        // image component
        } else if row.image_id.is_some() {
            if let (Some(image_id), Some(image_path), Some(alt_text)) =
                (Some(row.image_id), row.image_path, row.alt_text)
            {
                description_components.push(DescriptionComponent {
                    component_id: row.component_id,
                    priority: row.priority,
                    product_id: row.product_id,
                    text: None,
                    image: Some(ImageComponent {
                        image_id,
                        image_path,
                        alt_text,
                    }),
                });
            }
        } else {
            // Should never happen, db has constraints
            return Err(sqlx::Error::Decode(
                "Could not decode description component, corrupt data".into(),
            ));
        }
    }
    Result::Ok(description_components)
}

pub async fn get_all_image_paths(
    pool: &Pool<Postgres>,
    product_id: &str,
) -> Result<Vec<String>, sqlx::Error> {
    let rows = query!(
        r#"SELECT image_path
        FROM description_component
        FULL JOIN product_image ON description_component.image_id = product_image.image_id
        WHERE description_component.product_id = $1
        AND image_path IS NOT null;
        "#,
        product_id
    )
    .fetch_all(pool)
    .await?;

    let mut paths: Vec<String> = Vec::new();
    for row in rows {
        paths.push(row.image_path);
    }
    Result::Ok(paths)
}

/// Returns a description component by id.
#[deprecated]
pub async fn get_description_component_by_id(
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
        WHERE description_component.component_id = $1;"#,
        component_id
    )
    .fetch_one(pool)
    .await?;

    // text component
    if row.text_id.is_some() {
        if let (Some(text_id), Some(text_title), Some(paragraph)) =
            (Some(row.text_id), row.text_title, row.paragraph)
        {
            return Result::Ok(DescriptionComponent {
                component_id: row.component_id,
                priority: row.priority,
                product_id: row.product_id,
                text: Some(TextComponent {
                    text_id,
                    text_title,
                    paragraph,
                }),
                image: None,
            });
        }
    // image component
    } else if row.image_id.is_some() {
        if let (Some(image_id), Some(image_path), Some(alt_text)) =
            (Some(row.image_id), row.image_path, row.alt_text)
        {
            return Result::Ok(DescriptionComponent {
                component_id: row.component_id,
                priority: row.priority,
                product_id: row.product_id,
                text: None,
                image: Some(ImageComponent {
                    image_id,
                    image_path,
                    alt_text,
                }),
            });
        }
    } else {
        // Should never happen, db has constraints
        return Err(sqlx::Error::Decode(
            "Could not decode description component, corrupt data".into(),
        ));
    }
    Err(sqlx::Error::Decode(
        "Could not decode description component, corrupt data".into(),
    ))
}

/// Returns a description component by id.
/// Ensures that the component belongs to the product.
pub async fn get_description_component_checked(
    pool: &Pool<Postgres>,
    product_id: &str,
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
    WHERE description_component.component_id = $1
    AND description_component.product_id = $2
    ;"#,
        component_id,
        product_id
    )
    .fetch_one(pool)
    .await?;

    // text component
    if row.text_id.is_some() {
        if let (Some(text_id), Some(text_title), Some(paragraph)) =
            (Some(row.text_id), row.text_title, row.paragraph)
        {
            return Result::Ok(DescriptionComponent {
                component_id: row.component_id,
                priority: row.priority,
                product_id: row.product_id,
                text: Some(TextComponent {
                    text_id,
                    text_title,
                    paragraph,
                }),
                image: None,
            });
        }
    // image component
    } else if row.image_id.is_some() {
        if let (Some(image_id), Some(image_path), Some(alt_text)) =
            (Some(row.image_id), row.image_path, row.alt_text)
        {
            return Result::Ok(DescriptionComponent {
                component_id: row.component_id,
                priority: row.priority,
                product_id: row.product_id,
                text: None,
                image: Some(ImageComponent {
                    image_id,
                    image_path,
                    alt_text,
                }),
            });
        }
    } else {
        // Should never happen, db has constraints
        return Err(sqlx::Error::Decode(
            "Could not decode description component, corrupt data".into(),
        ));
    }
    Err(sqlx::Error::Decode(
        "Could not decode description component, corrupt data".into(),
    ))
}

/// Creates a new description component, and returns newly created component.
///
/// # Arguments
/// * `pool` - Database connection pool
/// * `product_id` - Product id of the product the component belongs to
/// * `component` - Partial description component, any type.
#[deprecated]
pub async fn create_component(
    pool: &Pool<Postgres>,
    product_id: &str,
    component: PartialDescriptionComponent,
) -> Result<DescriptionComponent, DescriptionCompError> {
    if let ComponentType::Invalid = component.get_type() {
        return Err(DescriptionCompError::InvalidComponent(
            "Invalid component type".into(),
        ));
    }

    if let Some(text_comp) = component.text {
        let text_comp = insert_text_component(pool, &text_comp).await?;

        let result_row = query!(
            r#"INSERT INTO description_component (product_id, text_id)
            VALUES ($1, $2)
            RETURNING *;"#,
            product_id,
            text_comp.text_id
        )
        .fetch_one(pool)
        .await;

        let result_row = match result_row {
            Ok(result_row) => result_row,
            Err(err) => return Err(DescriptionCompError::SqlxError(err)),
        };

        Ok(DescriptionComponent {
            component_id: result_row.component_id,
            priority: result_row.priority,
            product_id: result_row.product_id,
            text: Some(text_comp),
            image: None,
        })
    } else if let Some(image_comp) = component.image {
        let image_comp = insert_image_component(pool, &image_comp).await?;
        let returned_row = query!(
            r#"INSERT INTO description_component (product_id, image_id)
            VALUES ($1, $2)
            RETURNING *;"#,
            product_id,
            image_comp.image_id
        )
        .fetch_one(pool)
        .await;
        let returned_row = match returned_row {
            Ok(returned_row) => returned_row,
            Err(err) => return Err(DescriptionCompError::SqlxError(err)),
        };

        Ok(DescriptionComponent {
            component_id: returned_row.component_id,
            priority: returned_row.priority,
            product_id: returned_row.product_id,
            text: None,
            image: Some(image_comp),
        })
    } else {
        return Err(DescriptionCompError::InvalidComponent(
            "Component is missing".into(),
        ));
    }
}

pub async fn create_text_component(
    pool: &Pool<Postgres>,
    product_id: &str,
    text_comp: TextComponent,
) -> Result<DescriptionComponent, DescriptionCompError> {
    let text_comp = insert_text_component(pool, &text_comp).await?;

    let result_row = query!(
        r#"INSERT INTO description_component (product_id, text_id)
            VALUES ($1, $2)
            RETURNING *;"#,
        product_id,
        text_comp.text_id
    )
    .fetch_one(pool)
    .await;

    let result_row = match result_row {
        Ok(result_row) => result_row,
        Err(err) => return Err(DescriptionCompError::SqlxError(err)),
    };

    Ok(DescriptionComponent {
        component_id: result_row.component_id,
        priority: result_row.priority,
        product_id: result_row.product_id,
        text: Some(text_comp),
        image: None,
    })
}

pub async fn create_image_component(
    pool: &Pool<Postgres>,
    product_id: &str,
    image_comp: ImageComponent,
) -> Result<DescriptionComponent, DescriptionCompError> {
    let image_comp = insert_image_component(pool, &image_comp).await?;
    let returned_row = query!(
        r#"INSERT INTO description_component (product_id, image_id)
            VALUES ($1, $2)
            RETURNING *;"#,
        product_id,
        image_comp.image_id
    )
    .fetch_one(pool)
    .await;
    let returned_row = match returned_row {
        Ok(returned_row) => returned_row,
        Err(err) => return Err(DescriptionCompError::SqlxError(err)),
    };

    Ok(DescriptionComponent {
        component_id: returned_row.component_id,
        priority: returned_row.priority,
        product_id: returned_row.product_id,
        text: None,
        image: Some(image_comp),
    })
}

/// Creates a new text component and returns the id.
async fn insert_text_component(
    pool: &Pool<Postgres>,
    text_component: &TextComponent,
) -> Result<TextComponent, DescriptionCompError> {
    let result_row = query!(
        r#"INSERT INTO product_text (text_title, paragraph)
        VALUES ($1, $2)
        RETURNING *;"#,
        text_component.text_title,
        text_component.paragraph
    )
    .fetch_one(pool)
    .await;

    let result_row = match result_row {
        Ok(result_row) => result_row,
        Err(err) => return Err(DescriptionCompError::SqlxError(err)),
    };

    let text_component = TextComponent {
        text_id: Some(result_row.text_id),
        text_title: result_row.text_title,
        paragraph: result_row.paragraph,
    };
    Result::Ok(text_component)
}

/// Creates a new image component and returns the id.
async fn insert_image_component(
    pool: &Pool<Postgres>,
    image_component: &ImageComponent,
) -> Result<ImageComponent, DescriptionCompError> {
    let result_row = query!(
        r#"INSERT INTO product_image (image_path, alt_text)
        VALUES ($1, $2)
        RETURNING *;"#,
        image_component.image_path,
        image_component.alt_text
    )
    .fetch_one(pool)
    .await;

    let result_row = match result_row {
        Ok(result_row) => result_row,
        Err(err) => return Err(DescriptionCompError::SqlxError(err)),
    };

    let image_comp = ImageComponent {
        image_id: Some(result_row.image_id),
        image_path: result_row.image_path,
        alt_text: result_row.alt_text,
    };

    Result::Ok(image_comp)
}

pub async fn update_priority(
    pool: &Pool<Postgres>,
    product_id: &str,
    component_id: i32,
    new_priority: i32,
) -> Result<(), sqlx::Error> {
    let query = query!(
        r#"UPDATE description_component
        SET priority = $1
        WHERE component_id = $2 AND product_id=$3;"#,
        new_priority,
        component_id,
        product_id,
    );

    pool.execute(query).await?;
    Ok(())
}

/// swaps the priority of two components
///
/// # Arguments
///
/// * `pool` - The database pool
/// * `product_id` - The product id
/// * `component_ids` - The component ids to swap priority
///
/// # Returns
///
/// * `Result<Vec<DescriptionComponent>, sqlx::Error>` - All description component after priority swap
pub async fn swap_priority(
    pool: &Pool<Postgres>,
    product_id: &str,
    component_ids: (i32, i32),
) -> Result<Vec<DescriptionComponent>, sqlx::Error> {
    let all_components = get_product_description_components(pool, product_id).await?;

    let component_1 = match all_components
        .iter()
        .find(|component| component.component_id == component_ids.0)
    {
        Some(component) => component,
        None => return Err(sqlx::Error::RowNotFound),
    };
    let component_2 = match all_components
        .iter()
        .find(|component| component.component_id == component_ids.1)
    {
        Some(component) => component,
        None => return Err(sqlx::Error::RowNotFound),
    };

    let query1 = query!(
        r#"
        UPDATE description_component
        SET priority = $1
        WHERE component_id = $2;
        "#,
        component_2.priority,
        component_1.component_id
    );

    let query2 = query!(
        r#"UPDATE description_component
        SET priority = $1
        WHERE component_id = $2;
        "#,
        component_1.priority,
        component_2.component_id
    );

    let mut transaction = pool.begin().await?;
    transaction.execute(query1).await?;
    transaction.execute(query2).await?;
    transaction.commit().await?;

    let updated_components = get_product_description_components(pool, product_id).await?;
    Ok(updated_components)
}

/// Deletes a component from the database.
/// If the component is an image_component, path to the image is returned.
///
/// # Returns
///
/// * `Result<Option<String>, sqlx::Error>`
///     * `Ok(Some(String))` - Path to the image referenced by the deleted component
///     * `Ok(None)` - No image in the deleted component
///     * `Err(sqlx::Error)` - Error while deleting component
pub async fn delete_component(
    pool: &Pool<Postgres>,
    product_id: &str,
    component_id: i32,
) -> Result<Option<String>, sqlx::Error> {
    let to_delete = get_description_component_checked(pool, product_id, component_id).await?;
    let img_path = match to_delete.image {
        Some(image) => Some(image.image_path),
        None => None,
    };

    let query = query!(
        r#"DELETE FROM description_component
        WHERE component_id = $1 AND product_id=$2;"#,
        component_id,
        product_id
    );
    let x = pool.execute(query).await;
    match x {
        Ok(_) => Ok(img_path),
        Err(err) => Err(err),
    }
}
