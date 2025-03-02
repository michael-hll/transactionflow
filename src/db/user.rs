use crate::controllers::auth::SignupRequest;
use bcrypt::{DEFAULT_COST, hash};
use rust_decimal::Decimal;
use sqlx::types::chrono;

pub async fn has_email_exists(db: &sqlx::PgPool, email: &str) -> bool {
  sqlx::query!("SELECT email FROM users WHERE email = $1", email)
    .fetch_optional(db)
    .await
    .unwrap()
    .is_some()
}

pub async fn create(db: &sqlx::PgPool, user: &SignupRequest) -> bool {
  let hashed_password = hash(&user.password, DEFAULT_COST).unwrap();

  sqlx::query!(
    r#"
      INSERT INTO users (email, password, first_name, last_name)
      VALUES ($1, $2, $3, $4)
    "#,
    &user.email,
    hashed_password,
    &user.first_name,
    &user.last_name
  )
  .execute(db)
  .await
  .is_ok()
}

pub struct User {
  pub id: i64,
  pub email: String,
  pub password: String,
  pub first_name: String,
  pub last_name: String,
  pub balance: Decimal,
  pub created_at: chrono::NaiveDateTime,
  pub updated_at: chrono::NaiveDateTime,
}

pub async fn get_by_email(db: &sqlx::PgPool, email: &str) -> Option<User> {
  sqlx::query_as!(
    User,
    r#"
      SELECT id, email, password, first_name, last_name, balance, created_at, updated_at
      FROM users
      WHERE email = $1
    "#,
    email
  )
  .fetch_optional(db)
  .await
  .unwrap()
}
