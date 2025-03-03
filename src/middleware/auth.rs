use actix_web::Error;
use actix_web::HttpMessage;
use actix_web::body::BoxBody;
use actix_web::dev::ServiceRequest;
use actix_web::dev::ServiceResponse;
use actix_web::error::ErrorUnauthorized;
use actix_web::middleware::Next;
use actix_web::web;
use jsonwebtoken::DecodingKey;
use jsonwebtoken::Validation;
use jsonwebtoken::decode;
use serde_json::json;

use crate::AppState;
use crate::controllers::auth::Claims;

pub struct UserId(pub i64);

pub async fn verify_jwt(
  req: ServiceRequest,
  next: Next<BoxBody>,
) -> Result<ServiceResponse, Error> {
  let auth_header = req.headers().get("Authorization").ok_or_else(|| {
    ErrorUnauthorized(json!({
      "status": "error",
      "message": "missing authorization header"
    }))
  })?;

  let auth_str = auth_header.to_str().map_err(|_| {
    ErrorUnauthorized(json!({
      "status": "error",
      "message": "missing authorization header"
    }))
  })?;

  if !auth_str.starts_with("Bearer ") {
    return Err(ErrorUnauthorized(json!({
      "status": "error",
      "message": "invalid authorization header"
    })));
  }

  let token = auth_str.trim_start_matches("Bearer ");
  let state = req.app_data::<web::Data<AppState>>().unwrap();
  let key = DecodingKey::from_secret(state.jwt_secret.as_bytes());
  match decode::<Claims>(token, &key, &Validation::default()) {
    Ok(token_data) => {
      req.extensions_mut().insert(UserId(token_data.claims.sub.parse::<i64>().unwrap_or(0)));
      next.call(req).await
    }
    Err(_) => {
      return Err(ErrorUnauthorized(json!({
        "status": "error",
        "message": "invalid token"
      })));
    }
  }
}
