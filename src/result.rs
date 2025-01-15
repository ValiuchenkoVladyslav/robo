#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("Io error: {0}")]
  Io(#[from] std::io::Error),

  #[error("Bincode error: {0}")]
  Bincode(#[from] bincode::Error),

  #[error("UI error: {0}")]
  Ui(#[from] eframe::Error),

  #[error("Ollama error: {0}")]
  Ollama(#[from] ollama_rs::error::OllamaError),

  #[error("Failed to parse port: {0}")]
  ParsePort(#[from] std::num::ParseIntError),
}

pub type Result<T=()> = core::result::Result<T, Error>;
