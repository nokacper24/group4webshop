use serde::{Deserialize, Serialize};
use sqlx::{
    query_as, {Pool, Postgres},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    category_id: i32,
    name: String,
    description: String,
}

/// Returns all categories.
pub async fn get_categories(pool: &Pool<Postgres>) -> Result<Vec<Category>, sqlx::Error> {
    let categories = query_as!(Category, "SELECT * FROM category")
        .fetch_all(pool)
        .await?;
    Ok(categories)
}

/// Returns a category.
pub async fn get_category_by_id(pool: &Pool<Postgres>, id: &i32) -> Result<Category, sqlx::Error> {
    let category = query_as!(
        Category,
        "SELECT * FROM category WHERE category_id = $1",
        id
    )
    .fetch_one(pool)
    .await?;
    Ok(category)
}
