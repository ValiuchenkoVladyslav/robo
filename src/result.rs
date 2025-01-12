#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("Io error: {0}")]
  Io(#[from] std::io::Error),

  #[error("Bincode error: {0}")]
  BincodeError(#[from] bincode::Error),
}

pub type Result<T=()> = core::result::Result<T, Error>;
