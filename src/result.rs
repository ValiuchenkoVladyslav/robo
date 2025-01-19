use actix_web::http::StatusCode;

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

  #[error("Not Found")]
  NotFound,

  #[error("Invalid message role!")]
  InvalidRole,

  #[error("Unauthorized")]
  Unauthorized,

  #[error("Email already taken!")]
  EmailTaken,
}

impl actix_web::ResponseError for Error {
  fn status_code(&self) -> StatusCode {
    match self {
      Error::NotFound => StatusCode::NOT_FOUND,
      Error::Unauthorized => StatusCode::UNAUTHORIZED,
      _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
  }
}

pub type Result<T = ()> = core::result::Result<T, Error>;
