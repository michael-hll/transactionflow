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

  HttpServer::new(move || {
    App::new()
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
          .service(controllers::categories::update),
      )
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}
