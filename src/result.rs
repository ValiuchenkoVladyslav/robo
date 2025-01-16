#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("Io error: {0}")]
  Io(#[from] std::io::Error),

  #[error("Bincode error: {0}")]
  Bincode(#[from] bincode::Error),

  #[cfg(feature = "gui")]
  #[error("GUI error: {0}")]
  Gui(#[from] eframe::Error),

  #[error("Ollama error: {0}")]
  Ollama(#[from] ollama_rs::error::OllamaError),

  // only used for port parsing; see state.rs
  #[error("Failed to parse port: {0}")]
  ParsePort(#[from] std::num::ParseIntError),

  #[cfg(feature = "api")]
  #[error("Not Found")]
  NotFound,
}

#[cfg(feature = "api")]
use actix_web::http::StatusCode;

#[cfg(feature = "api")]
impl actix_web::ResponseError for Error {
  fn status_code(&self) -> StatusCode {
    match self {
      Error::NotFound => StatusCode::NOT_FOUND,
      _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
  }
}

pub type Result<T = ()> = core::result::Result<T, Error>;
