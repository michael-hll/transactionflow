use std::time::Duration;

use actix_cors::Cors;
use actix_extensible_rate_limit::RateLimiter;
use actix_extensible_rate_limit::backend::SimpleInputFunctionBuilder;
use actix_extensible_rate_limit::backend::memory::InMemoryBackend;
use actix_web::App;
use actix_web::HttpServer;
use actix_web::middleware::from_fn;
use actix_web::web;
use tokio::sync::Mutex;

mod controllers;
mod db;
mod middleware;
mod utils;

#[derive(Debug)]
struct AppState {
  db: Mutex<sqlx::PgPool>,
  jwt_secret: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenvy::dotenv().ok();

  let state = web::Data::new(AppState {
    db: Mutex::new(sqlx::PgPool::connect(&std::env::var("DATABASE_URL").unwrap()).await.unwrap()),
    jwt_secret: std::env::var("JWT_SECRET").unwrap(),
  });

  let rate_limiter_backend = InMemoryBackend::builder().build();

  HttpServer::new(move || {
    App::new()
      .wrap(Cors::permissive())
      .wrap(
        RateLimiter::builder(
          rate_limiter_backend.clone(),
          SimpleInputFunctionBuilder::new(Duration::from_secs(60), 60).real_ip_key().build(),
        )
        .add_headers()
        .build(),
      )
      .app_data(state.clone())
      .service(controllers::auth::sign_up)
      .service(controllers::auth::sign_in)
      .service(
        web::scope("/api")
          .wrap(from_fn(middleware::auth::verify_jwt))
          .service(controllers::me::get_profile)
          .service(controllers::me::update_profile)
          .service(controllers::categories::index)
          .service(controllers::categories::create)
          .service(controllers::categories::delete)
          .service(controllers::categories::show)
          .service(controllers::categories::update)
          .service(controllers::transactions::index)
          .service(controllers::transactions::create)
          .service(controllers::transactions::show)
          .service(controllers::transactions::update)
          .service(controllers::transactions::destroy),
      )
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}
