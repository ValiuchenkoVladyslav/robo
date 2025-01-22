//! User API routes

use crate::{
  db::{fetch_add_max_uid, postgres},
  jwt::create_jwt,
  result::{Error, Result},
  user::schemas::{User, UserIden},
};
use argon2::{
  password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
  Argon2, PasswordHash, PasswordVerifier,
};
use axum::Json;
use sea_query::{Expr, PostgresQueryBuilder, Query};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as};
use tracing::instrument;
use ts_rs::TS;
use validator::Validate;

#[derive(TS, Debug, Serialize)]
#[ts(export, export_to = "./index.ts")]
pub struct PublicUser {
  id: i32,
  name: String,
  email: String,
}

#[derive(TS, Debug, Serialize)]
#[ts(export, export_to = "./index.ts")]
pub struct AuthUser {
  token: String,
  public_user: PublicUser,
}

#[derive(TS, Debug, Deserialize, Validate)]
#[ts(export, export_to = "./index.ts")]
pub struct RegisterRequest {
  #[validate(length(min = 3, max = 255))]
  name: String,
  #[validate(email)]
  email: String,
  #[validate(length(min = 6, max = 255))]
  password: String,
}

/// create new user
#[instrument(name = "users::create_user")]
pub async fn create_user(Json(new_user): Json<RegisterRequest>) -> Result<Json<AuthUser>> {
  new_user.validate()?;

  // check if email is taken
  let find_by_email_query = Query::select()
    .from(UserIden::Table)
    .columns([UserIden::Email])
    .and_where(Expr::col(UserIden::Email).eq(new_user.email.clone()))
    .to_string(PostgresQueryBuilder);

  // query all shards for email
  let (cols1, cols2) = tokio::join!(
    query(&find_by_email_query).fetch_optional(postgres(1)),
    query(&find_by_email_query).fetch_optional(postgres(2)),
  );

  // if at least one shard failed, we can't be sure if the email is taken
  if cols1?.is_some() || cols2?.is_some() {
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

  let next_uid = fetch_add_max_uid();

  // insert new user
  let insert_user_query = Query::insert()
    .into_table(UserIden::Table)
    .columns([
      UserIden::Id,
      UserIden::Name,
      UserIden::Email,
      UserIden::Password,
    ])
    .values_panic([
      next_uid.into(),
      new_user.name.clone().into(),
      new_user.email.clone().into(),
      password_hash.into(),
    ])
    .to_string(PostgresQueryBuilder);

  query(&insert_user_query)
    .execute(postgres(next_uid))
    .await?;

  // return user with token
  let token = create_jwt(next_uid);

  let public_user = PublicUser {
    id: next_uid,
    name: new_user.name,
    email: new_user.email,
  };

  Ok(Json(AuthUser {
    token,
    public_user,
  }))
}

#[derive(TS, Debug, Deserialize)]
#[ts(export, export_to = "./index.ts")]
pub struct LoginRequest {
  email: String,
  password: String,
}

/// login user
#[instrument(name = "users::login_user")]
pub async fn login_user(Json(login_request): Json<LoginRequest>) -> Result<Json<AuthUser>> {
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

  // query all shards for email
  let (user1, user2) = tokio::join!(
    query_as::<_, User>(&find_by_email_query).fetch_optional(postgres(1)),
    query_as::<_, User>(&find_by_email_query).fetch_optional(postgres(2)),
  );

  let Some(user) = user1?.or(user2?) else {
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
    name: user.name,
    email: user.email,
  };

  Ok(Json(AuthUser {
    token,
    public_user,
  }))
}
