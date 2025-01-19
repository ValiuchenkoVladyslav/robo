use crate::{jwt::validate_jwt, result::Error as AppError};
use actix_web::{
  body::MessageBody,
  dev::{ServiceRequest, ServiceResponse},
  middleware::Next,
  Error, FromRequest, HttpMessage, HttpRequest,
};
// use jsonwebtoken::errors::ErrorKind as JwtErr;
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};
use tracing::error;

pub async fn auth_middleware(
  req: ServiceRequest,
  next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
  let Some(auth_header) = req.headers().get("Authorization") else {
    return Err(AppError::Unauthorized)?;
  };

  let Ok(auth_header) = auth_header.to_str() else {
    return Err(AppError::Unauthorized)?;
  };

  // Bearer [token]
  let Some(token) = auth_header.split(" ").last() else {
    return Err(AppError::Unauthorized)?;
  };

  let id = match validate_jwt(token) {
    Ok(id) => id,
    Err(err) => {
      return match err.kind() {
        // todo error messages
        // JwtErr::ExpiredSignature => Err(AppError::JwtExpired)?,
        _ => Err(AppError::Unauthorized)?,
      };
    }
  };

  req.extensions_mut().insert(id);

  next.call(req).await
}

/// Extracts the user id from request extensions. Must only be used inside auth middleware
#[derive(Debug, Serialize, Deserialize)]
pub struct Auth(pub i32);

impl FromRequest for Auth {
  type Error = AppError;
  type Future = Ready<Result<Self, Self::Error>>;

  fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
    let extensions = req.extensions();
    let Some(user_id) = extensions.get::<i32>() else {
      error!("Attempted to extract user id from request extensions, but none was found. Most likely the auth middleware was not used on this route");

      return ready(Err(AppError::Unauthorized));
    };

    ready(Ok(Auth(*user_id)))
  }
}
