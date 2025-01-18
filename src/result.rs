use actix_web::http::StatusCode;

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("Io error: {0}")]
  Io(#[from] std::io::Error),

  // only used for port parsing
  #[error("Failed to parse port: {0}")]
  ParsePort(#[from] std::num::ParseIntError),

  #[error("Ollama error: {0}")]
  Ollama(#[from] ollama_rs::error::OllamaError),

  #[error("Redis error: {0}")]
  Redis(#[from] redis::RedisError),

  #[error("DB error: {0}")]
  Db(#[from] sqlx::Error),

  #[error("Not Found")]
  NotFound,

  #[error("Invalid message role!")]
  InvalidRole,
}

impl actix_web::ResponseError for Error {
  fn status_code(&self) -> StatusCode {
    match self {
      Error::NotFound => StatusCode::NOT_FOUND,
      _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
  }
}

pub type Result<T = ()> = core::result::Result<T, Error>;
