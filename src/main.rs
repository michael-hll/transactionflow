use actix_web::{App, HttpServer, web};
use tokio::sync::Mutex;

mod controllers;
mod db;

struct AppState {
  db: Mutex<sqlx::PgPool>,
  jwt_secret: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenvy::dotenv().ok();

  let state = web::Data::new(AppState {
    db: Mutex::new(
      sqlx::PgPool::connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap(),
    ),
    jwt_secret: std::env::var("JWT_SECRET").unwrap(),
  });

  HttpServer::new(move || {
    App::new()
      .app_data(state.clone())
      .service(controllers::auth::sign_up)
      .service(controllers::auth::sign_in)
      .service(controllers::me::get_profile)
      .service(controllers::me::update_profile)
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}
