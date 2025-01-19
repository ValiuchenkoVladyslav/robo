//! User API routes

use crate::{
  jwt::create_jwt,
  result::{Error, Result},
  state::AppState,
  user::schemas::{User, UserIden},
};
use actix_web::{
  delete, get, patch, post, put,
  web::{Json, Path},
};
use sea_query::{Expr, PostgresQueryBuilder, Query};
use serde::Serialize;
use sqlx::{query, Row};
use tracing::{debug, instrument};

#[derive(Debug, Serialize)]
struct CreateUserResponse {
  token: String,
  full_user: User,
}

/// create new user
#[instrument(name = "users::create_user")]
#[post("/")]
pub async fn create_user(new_user: Json<User>) -> Result<Json<CreateUserResponse>> {
  let db = AppState::db();

  // check if email is taken
  let find_by_email_query = Query::select()
    .from(UserIden::Table)
    .columns(vec![UserIden::Email])
    .and_where(Expr::col(UserIden::Email).eq(new_user.email.clone()))
    .to_string(PostgresQueryBuilder);

  let cols = query(&find_by_email_query).fetch_optional(db).await?;

  if cols.is_some() {
    debug!("Email already taken");
    return Err(Error::NotFound); // todo
  }

  debug!("TODO HASH PASSWORD");

  // insert new user
  let insert_user_query = Query::insert()
    .into_table(UserIden::Table)
    .columns([UserIden::Name, UserIden::Email, UserIden::Password])
    .values_panic([
      new_user.name.clone().into(),
      new_user.email.clone().into(),
      new_user.password.clone().into(),
    ])
    .returning_col(UserIden::Id)
    .to_string(PostgresQueryBuilder);

  let user_id: i32 = query(&insert_user_query).fetch_one(db).await?.get(0);

  // create jwt token
  let token = create_jwt(user_id);

  // return user with token
  let mut full_user = new_user.into_inner();
  full_user.id = user_id;

  Ok(Json(CreateUserResponse {
    token,
    full_user,
  }))
}
