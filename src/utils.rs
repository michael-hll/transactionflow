use actix_web::HttpMessage;
use actix_web::HttpRequest;

use crate::db::user::User;
use crate::db::{self};
use crate::middleware::auth::UserId;

pub fn get_user_id(req: &HttpRequest) -> i64 {
  let ext = req.extensions();
  let user_id = ext.get::<UserId>().map(|x| x.0).unwrap_or_else(|| 0);
  user_id
}

pub async fn get_auth_user(
  req: &HttpRequest,
  db: &sqlx::PgPool,
) -> User {
  let user_id = get_user_id(&req);
  db::user::get_by_id(db, user_id).await.unwrap()
}
