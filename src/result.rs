use actix_web::http::StatusCode;

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("Io error: {0}")]
  Io(#[from] std::io::Error),

  #[error("Ollama error: {0}")]
  Ollama(#[from] ollama_rs::error::OllamaError),

  // only used for port parsing; see state.rs
  #[error("Failed to parse port: {0}")]
  ParsePort(#[from] std::num::ParseIntError),

  #[error("Redis error: {0}")]
  Redis(#[from] redis::RedisError),

  #[error("Not Found")]
  NotFound,
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
