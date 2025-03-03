use actix_web::{HttpRequest, HttpResponse, Responder, delete, get, post, put, web};
use serde::Deserialize;
use serde_json::json;

use crate::{AppState, db, utils};

#[derive(Deserialize, Debug)]
pub struct CreateCategoryRequest {
  pub name: String,
  pub description: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct UpdateCategoryRequest {
  pub name: String,
  pub description: Option<String>,
}

#[get("/categories")]
pub async fn index(
  state: web::Data<AppState>,
  req: HttpRequest,
) -> impl Responder {
  let db = state.db.lock().await;
  let user_id = utils::get_user_id(&req);

  let categories = db::categories::get_all_of_user(&db, user_id).await;

  HttpResponse::Ok().json(categories)
}

#[post("/categories")]
pub async fn create(
  state: web::Data<AppState>,
  req: HttpRequest,
  data: web::Json<CreateCategoryRequest>,
) -> impl Responder {
  let db = state.db.lock().await;
  let user_id = utils::get_user_id(&req);

  let category = db::categories::create(&db, user_id, &data).await.unwrap();

  HttpResponse::Ok().json(category)
}

#[get("/categories/{id}")]
pub async fn show(
  state: web::Data<AppState>,
  req: HttpRequest,
  id: web::Path<i64>,
) -> impl Responder {
  let db = state.db.lock().await;
  let user_id = utils::get_user_id(&req);

  let Some(category) = db::categories::get(&db, id.into_inner()).await else {
    return HttpResponse::NotFound().json(json!({"status": "error", "message": "Not found"}));
  };

  if category.user_id != user_id {
    return HttpResponse::Unauthorized()
      .json(json!({"status": "error", "message": "Unauthorized"}));
  }

  HttpResponse::Ok().json(category)
}

#[put("/categories/{id}")]
pub async fn update(
  state: web::Data<AppState>,
  req: HttpRequest,
  id: web::Path<i64>,
  data: web::Json<UpdateCategoryRequest>,
) -> impl Responder {
  let db = state.db.lock().await;
  let user_id = utils::get_user_id(&req);

  let Some(category) = db::categories::get(&db, id.into_inner()).await else {
    return HttpResponse::NotFound().json(json!({"status": "error", "message": "Not found"}));
  };

  if category.user_id != user_id {
    return HttpResponse::Unauthorized()
      .json(json!({"status": "error", "message": "Unauthorized"}));
  }

  db::categories::update(&db, category.id, &data).await;

  let category = db::categories::get(&db, category.id).await;

  HttpResponse::Ok().json(category)
}

#[delete("/categories/{id}")]
pub async fn delete(
  state: web::Data<AppState>,
  req: HttpRequest,
  id: web::Path<i64>,
) -> impl Responder {
  let db = state.db.lock().await;
  let user_id = utils::get_user_id(&req);

  let Some(category) = db::categories::get(&db, id.into_inner()).await else {
    return HttpResponse::NotFound().json(json!({"status": "error", "message": "Not found"}));
  };

  if category.user_id != user_id {
    return HttpResponse::Unauthorized()
      .json(json!({"status": "error", "message": "Unauthorized"}));
  }

  db::categories::destroy(&db, category.id).await;

  HttpResponse::Ok().json(json!({"status": "success"}))
}
