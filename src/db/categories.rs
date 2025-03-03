use rust_decimal::Decimal;
use serde::Serialize;

use crate::controllers::categories::{CreateCategoryRequest, UpdateCategoryRequest};

#[derive(Serialize)]
pub struct Category {
  pub id: i64,
  pub user_id: i64,
  pub name: String,
  pub description: Option<String>,
  pub balance: Decimal,
  pub created_at: chrono::NaiveDateTime,
  pub updated_at: chrono::NaiveDateTime,
}

pub async fn get_all_of_user(
  db: &sqlx::PgPool,
  user_id: i64,
) -> Vec<Category> {
  sqlx::query_as!(Category, "SELECT * FROM categories WHERE user_id = $1", user_id)
    .fetch_all(db)
    .await
    .unwrap()
}

pub async fn get(
  db: &sqlx::PgPool,
  id: i64,
) -> Option<Category> {
  sqlx::query_as!(Category, "SELECT * FROM categories WHERE id = $1", id).fetch_one(db).await.ok()
}

pub async fn create(
  db: &sqlx::PgPool,
  user_id: i64,
  category: &CreateCategoryRequest,
) -> Option<Category> {
  let new_category = sqlx::query_as!(
    Category,
    "INSERT INTO categories (user_id, name, description) VALUES ($1, $2, $3)
     RETURNING *",
    user_id,
    &category.name,
    category.description.as_deref()
  )
  .fetch_one(db)
  .await
  .ok()?;

  Some(new_category)
}

pub async fn update(
  db: &sqlx::PgPool,
  id: i64,
  category: &UpdateCategoryRequest,
) {
  sqlx::query!(
    "UPDATE categories SET name = $1, description = $2 WHERE id = $3",
    &category.name,
    category.description.as_deref(),
    id
  )
  .execute(db)
  .await
  .unwrap();
}

pub async fn destroy(
  db: &sqlx::PgPool,
  id: i64,
) {
  sqlx::query!("DELETE FROM categories WHERE id = $1", id).execute(db).await.unwrap();
}

pub async fn update_balance(
  db: &sqlx::PgPool,
  id: i64,
  balance: Decimal,
) {
  sqlx::query!("UPDATE categories SET balance = $1 WHERE id = $2", balance, id)
    .execute(db)
    .await
    .unwrap();
}
