//! User API routes

use crate::{
  jwt::create_jwt,
  result::{Error, Result},
  state::AppState,
  user::schemas::{User, UserIden},
};
use argon2::{
  password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
  Argon2, PasswordHash, PasswordVerifier,
};
use axum::Json;
use sea_query::{Expr, PostgresQueryBuilder, Query};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, Row};
use tracing::instrument;

#[derive(Debug, Serialize)]
pub struct PublicUser {
  id: i32,
  name: String,
  email: String,
}

#[derive(Debug, Serialize)]
pub struct AuthUser {
  token: String,
  public_user: PublicUser,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
  name: String,
  email: String,
  password: String,
}

/// create new user
#[instrument(name = "users::create_user")]
pub async fn create_user(new_user: Json<RegisterRequest>) -> Result<Json<AuthUser>> {
  let db = AppState::db();

  // check if email is taken
  let find_by_email_query = Query::select()
    .from(UserIden::Table)
    .columns([UserIden::Email])
    .and_where(Expr::col(UserIden::Email).eq(new_user.email.clone()))
    .to_string(PostgresQueryBuilder);

  let cols = query(&find_by_email_query).fetch_optional(db).await?;

  if cols.is_some() {
    return Err(Error::EmailTaken); // TODO better errors
  }

  // hash password
  let password_hash = Argon2::default()
    .hash_password(
      new_user.password.as_bytes(),
      &SaltString::generate(&mut OsRng),
    )
    .unwrap()
    .to_string();

  // insert new user
  let insert_user_query = Query::insert()
    .into_table(UserIden::Table)
    .columns([UserIden::Name, UserIden::Email, UserIden::Password])
    .values_panic([
      new_user.name.clone().into(),
      new_user.email.clone().into(),
      password_hash.into(),
    ])
    .returning_col(UserIden::Id)
    .to_string(PostgresQueryBuilder);

  let user_id: i32 = query(&insert_user_query).fetch_one(db).await?.get(0);

  // return user with token
  let token = create_jwt(user_id);

  let public_user = PublicUser {
    id: user_id,
    name: new_user.name.clone(),
    email: new_user.email.clone(),
  };

  Ok(Json(AuthUser {
    token,
    public_user,
  }))
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
  email: String,
  password: String,
}

/// login user
#[instrument(name = "users::login_user")]
pub async fn login_user(login_request: Json<LoginRequest>) -> Result<Json<AuthUser>> {
  let db = AppState::db();

  // get user by email
  let find_by_email_query = Query::select()
    .from(UserIden::Table)
    .columns([
      UserIden::Id,
      UserIden::Name,
      UserIden::Email,
      UserIden::Password,
    ])
    .and_where(Expr::col(UserIden::Email).eq(login_request.email.clone()))
    .to_string(PostgresQueryBuilder);

  let user: Option<User> = query_as(&find_by_email_query).fetch_optional(db).await?;

  let Some(user) = user else {
    return Err(Error::Unauthorized);
  };

  // verify password
  let Ok(parsed_hash) = PasswordHash::new(&user.password) else {
    return Err(Error::Unauthorized);
  };

  let is_valid = Argon2::default()
    .verify_password(login_request.password.as_bytes(), &parsed_hash)
    .is_ok();

  if !is_valid {
    return Err(Error::Unauthorized);
  }

  // return user with token
  let token = create_jwt(user.id);

  let public_user = PublicUser {
    id: user.id,
    name: user.name.clone(),
    email: user.email.clone(),
  };

  Ok(Json(AuthUser {
    token,
    public_user,
  }))
}
