use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::post;
use actix_web::web;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;

use crate::AppState;
use crate::db;

#[derive(Deserialize, Debug)]
pub struct SignupRequest {
  pub email: String,
  pub password: String,
  pub first_name: String,
  pub last_name: String,
}

#[post("/auth/sign-up")]
pub async fn sign_up(
  state: web::Data<AppState>,
  data: web::Json<SignupRequest>,
) -> impl Responder {
  let db = state.db.lock().await;
  if db::user::has_email_exists(&db, &data.email).await {
    return HttpResponse::UnprocessableEntity().json(json!({
      "status": "error",
      "message": "email already exists"
    }));
  }

  db::user::create(&db, &data).await;

  HttpResponse::Created().json(json!({
    "status": "success",
    "message": "user created"
  }))
}

#[derive(Deserialize, Debug)]
pub struct SigninRequest {
  pub email: String,
  pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
  pub sub: String,
  pub role: String,
  pub exp: u64,
}

#[post("/auth/sign-in")]
pub async fn sign_in(
  state: web::Data<AppState>,
  data: web::Json<SigninRequest>,
) -> impl Responder {
  let db = state.db.lock().await;
  let user = db::user::get_by_email(&db, &data.email).await;

  if user.is_none() {
    return HttpResponse::Unauthorized().json(json!({
      "status": "error",
      "message": "invalid email or password"
    }));
  }

  let user = user.unwrap();
  if !bcrypt::verify(&data.password, &user.password).unwrap() {
    return HttpResponse::Unauthorized().json(json!({
      "status": "error",
      "message": "invalid email or password"
    }));
  }

  let claims = Claims {
    sub: user.id.to_string(),
    role: "user".to_string(),
    exp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + 24 * 60 * 60, // 24 hours
  };

  let token = jsonwebtoken::encode(
    &jsonwebtoken::Header::default(),
    &claims,
    &jsonwebtoken::EncodingKey::from_secret(state.jwt_secret.as_bytes()),
  )
  .unwrap();

  HttpResponse::Ok().json(json!({
    "status": "success",
    "token": token
  }))
}
