use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::get;
use actix_web::post;
use actix_web::web;
use serde::Deserialize;

use crate::AppState;
use crate::db;
use crate::utils;

#[get("/me")]
pub async fn get_profile(
  state: web::Data<AppState>,
  req: HttpRequest,
) -> impl Responder {
  let db = state.db.lock().await;
  let user = utils::get_auth_user(&req, &db).await;

  HttpResponse::Ok().json(user)
}

#[derive(Deserialize, Debug)]
pub struct UpdateProfileRequest {
  pub first_name: String,
  pub last_name: String,
}

#[post("/me")]
pub async fn update_profile(
  state: web::Data<AppState>,
  req: HttpRequest,
  data: web::Json<UpdateProfileRequest>,
) -> impl Responder {
  let db = state.db.lock().await;
  let user_id = utils::get_user_id(&req);
  db::user::update(&db, user_id, &data).await;
  let user = utils::get_auth_user(&req, &db).await;
  HttpResponse::Ok().json(user)
}
