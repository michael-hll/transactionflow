use rust_decimal::Decimal;
use serde::Serialize;

use crate::controllers::transactions::{CreateTransactionRequest, UpdateTransactionRequest};

#[derive(Serialize)]
pub struct Transaction {
  pub id: i64,
  pub user_id: i64,
  pub category_id: i64,
  pub r#type: String,
  pub amount: Decimal,
  pub memo: String,
  pub description: Option<String>,
  pub created_at: chrono::NaiveDateTime,
  pub updated_at: chrono::NaiveDateTime,
}

pub async fn get_all_of_user(
  db: &sqlx::PgPool,
  user_id: i64,
) -> Vec<Transaction> {
  sqlx::query_as!(Transaction, "SELECT * FROM transactions WHERE user_id = $1", user_id)
    .fetch_all(db)
    .await
    .unwrap()
}

pub async fn get_all_of_category(
  db: &sqlx::PgPool,
  category_id: i64,
) -> Vec<Transaction> {
  sqlx::query_as!(Transaction, "SELECT * FROM transactions WHERE category_id = $1", category_id)
    .fetch_all(db)
    .await
    .unwrap()
}

pub async fn get(
  db: &sqlx::PgPool,
  id: i64,
) -> Option<Transaction> {
  sqlx::query_as!(Transaction, "SELECT * FROM transactions WHERE id = $1", id)
    .fetch_optional(db)
    .await
    .unwrap()
}

pub async fn create(
  db: &sqlx::PgPool,
  user_id: i64,
  transaction: &CreateTransactionRequest,
) -> Transaction {
  sqlx::query_as!(Transaction, "INSERT INTO transactions (user_id, category_id, type, amount, memo, description) VALUES ($1, $2, $3, $4, $5, $6)
  RETURNING *", user_id, transaction.category_id, transaction.r#type, transaction.amount, transaction.memo, transaction.description)
        .fetch_one(db)
        .await
        .unwrap()
}

pub async fn update(
  db: &sqlx::PgPool,
  id: i64,
  transaction: &UpdateTransactionRequest,
) {
  sqlx::query!(
    "UPDATE transactions SET memo = $1, description = $2 WHERE id = $3",
    &transaction.memo,
    transaction.description.as_deref(),
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
  sqlx::query!("DELETE FROM transactions WHERE id = $1", id).execute(db).await.unwrap();
}
