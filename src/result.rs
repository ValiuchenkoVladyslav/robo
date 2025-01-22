use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("Io error: {0}")]
  Io(#[from] std::io::Error),

  #[error("JSON error: {0}")]
  Serde(#[from] serde_json::Error),

  #[error("Ollama error: {0}")]
  Ollama(#[from] ollama::error::OllamaError),

  #[error("Redis error: {0}")]
  Redis(#[from] redis::RedisError),

  #[error("DB error: {0}")]
  Db(#[from] sqlx::Error),

  #[error("Validation error: {0}")]
  Validation(#[from] validator::ValidationErrors),

  #[error("Not Found")]
  NotFound,

  #[error("Invalid message role!")]
  InvalidRole,

  #[error("Unauthorized")]
  Unauthorized,

  #[error("Email already taken!")]
  EmailTaken,
}

impl IntoResponse for Error {
  fn into_response(self) -> Response {
    let code = match self {
      Error::NotFound => StatusCode::NOT_FOUND,
      Error::EmailTaken => StatusCode::CONFLICT,
      Error::Unauthorized => StatusCode::UNAUTHORIZED,
      Error::Validation(_) => StatusCode::BAD_REQUEST,
      _ => StatusCode::INTERNAL_SERVER_ERROR,
    };

    (code, self.to_string()).into_response()
  }
}

pub type Result<T = ()> = core::result::Result<T, Error>;
