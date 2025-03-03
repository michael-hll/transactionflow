use actix_web::{HttpRequest, HttpResponse, Responder, delete, get, post, put, web};
use rust_decimal::Decimal;
use serde::Deserialize;
use serde_json::json;

use crate::{AppState, db, utils};

#[derive(Deserialize)]
pub struct CreateTransactionRequest {
  pub category_id: i64,
  pub r#type: String,
  pub amount: Decimal,
  pub memo: String,
  pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateTransactionRequest {
  pub memo: String,
  pub description: Option<String>,
}

#[get("/transactions")]
pub async fn index(
  state: web::Data<AppState>,
  req: HttpRequest,
) -> impl Responder {
  let db = state.db.lock().await;
  let user_id = utils::get_user_id(&req);

  let transactions = db::transactions::get_all_of_user(&db, user_id).await;

  HttpResponse::Ok().json(transactions)
}

#[post("/transactions")]
pub async fn create(
  state: web::Data<AppState>,
  req: HttpRequest,
  data: web::Json<CreateTransactionRequest>,
) -> impl Responder {
  let db = state.db.lock().await;
  let user = utils::get_auth_user(&req, &db).await;

  let Some(category) = db::categories::get(&db, data.category_id).await else {
    return HttpResponse::NotFound().json(json!({
        "status": "error",
        "message": "Category not found"
    }));
  };

  if category.user_id != user.id {
    return HttpResponse::Forbidden().json(json!({
        "status": "error",
        "message": "Unauthorized"
    }));
  }

  if data.r#type == "DEBIT" && (user.balance < data.amount || category.balance < data.amount) {
    return HttpResponse::BadRequest().json(json!({
        "status": "error",
        "message": "Insufficient balance"
    }));
  }

  let transaction = db::transactions::create(&db, user.id, &data).await;

  let user_balance = if utils::is_debit(&data.r#type) {
    user.balance - data.amount
  } else {
    user.balance + data.amount
  };

  db::user::update_balance(&db, user.id, user_balance).await;

  let category_balance = if utils::is_debit(&data.r#type) {
    category.balance - data.amount
  } else {
    category.balance + data.amount
  };

  db::categories::update_balance(&db, category.id, category_balance).await;

  HttpResponse::Created().json(transaction)
}

#[get("/transactions/{id}")]
pub async fn show(
  state: web::Data<AppState>,
  req: HttpRequest,
  id: web::Path<i64>,
) -> impl Responder {
  let db = state.db.lock().await;
  let user_id = utils::get_user_id(&req);

  let Some(transaction) = db::transactions::get(&db, id.into_inner()).await else {
    return HttpResponse::NotFound().json(json!({
        "status": "error",
        "message": "Transaction not found"
    }));
  };

  if transaction.user_id != user_id {
    return HttpResponse::Forbidden().json(json!({
        "status": "error",
        "message": "Unauthorized"
    }));
  }

  HttpResponse::Ok().json(transaction)
}

#[put("/transactions/{id}")]
pub async fn update(
  state: web::Data<AppState>,
  req: HttpRequest,
  id: web::Path<i64>,
  data: web::Json<UpdateTransactionRequest>,
) -> impl Responder {
  let db = state.db.lock().await;
  let user_id = utils::get_user_id(&req);

  let Some(transaction) = db::transactions::get(&db, id.into_inner()).await else {
    return HttpResponse::NotFound().json(json!({
        "status": "error",
        "message": "Transaction not found"
    }));
  };

  if transaction.user_id != user_id {
    return HttpResponse::Forbidden().json(json!({
        "status": "error",
        "message": "Unauthorized"
    }));
  };

  db::transactions::update(&db, transaction.id, &data).await;

  let transaction = db::transactions::get(&db, transaction.id).await.unwrap();

  HttpResponse::Ok().json(transaction)
}

#[delete("/transactions/{id}")]
pub async fn destroy(
  state: web::Data<AppState>,
  req: HttpRequest,
  id: web::Path<i64>,
) -> impl Responder {
  let db = state.db.lock().await;
  let user = utils::get_auth_user(&req, &db).await;

  let Some(transaction) = db::transactions::get(&db, id.into_inner()).await else {
    return HttpResponse::NotFound().json(json!({
        "status": "error",
        "message": "Transaction not found"
    }));
  };

  if transaction.user_id != user.id {
    return HttpResponse::Forbidden().json(json!({
        "status": "error",
        "message": "Unauthorized"
    }));
  };

  let category = db::categories::get(&db, transaction.category_id).await.unwrap();

  if utils::is_credit(&transaction.r#type)
    && (transaction.amount > user.balance || transaction.amount > category.balance)
  {
    return HttpResponse::BadRequest().json(json!({
        "status": "error",
        "message": "Insufficient balance"
    }));
  }

  db::transactions::destroy(&db, transaction.id).await;

  let user_balance = if utils::is_credit(&transaction.r#type) {
    user.balance - transaction.amount
  } else {
    user.balance + transaction.amount
  };

  db::user::update_balance(&db, user.id, user_balance).await;

  let category_balance = if utils::is_credit(&transaction.r#type) {
    category.balance - transaction.amount
  } else {
    category.balance + transaction.amount
  };

  db::categories::update_balance(&db, category.id, category_balance).await;

  HttpResponse::Ok().json(json!({"status": "success"}))
}
