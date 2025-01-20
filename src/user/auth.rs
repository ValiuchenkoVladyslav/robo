use crate::{
  jwt::validate_jwt,
  result::{Error, Result},
};
use axum::{
  extract::{FromRequestParts, Request},
  http::request::Parts,
  middleware::Next,
  response::Response,
};
use tracing::error;

pub async fn auth_middleware(mut req: Request, next: Next) -> Result<Response> {
  let Some(auth_header) = req.headers().get("Authorization") else {
    return Err(Error::Unauthorized)?;
  };

  let Ok(auth_header) = auth_header.to_str() else {
    return Err(Error::Unauthorized)?;
  };

  // Bearer [token]
  let Some(token) = auth_header.split(" ").last() else {
    return Err(Error::Unauthorized)?;
  };

  let id = match validate_jwt(token) {
    Ok(id) => id,
    _ => return Err(Error::Unauthorized)?,
  };

  req.extensions_mut().insert(id);

  let response = next.run(req).await;

  Ok(response)
}

/// Extracts the user id from JWT token. Must only be used inside auth middleware
#[derive(Debug)]
pub struct Auth(pub i32);

impl<S: Send + Sync> FromRequestParts<S> for Auth {
  type Rejection = Error;

  async fn from_request_parts(req: &mut Parts, _: &S) -> Result<Self> {
    let Some(user_id) = req.extensions.get::<i32>() else {
      error!("Attempted to extract user id from request extensions, but none was found. Most likely the auth middleware was not used on this route");

      return Err(Error::Unauthorized);
    };

    Ok(Self(*user_id))
  }
}
