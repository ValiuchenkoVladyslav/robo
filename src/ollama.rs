//! Ollama API

use crate::{result::Result, state::AppState};
use actix_web::{get, web::Json};
use ollama::models::LocalModel;

/// list available ollama models
#[get("/models")]
pub async fn get_models() -> Result<Json<Vec<LocalModel>>> {
  let ollama = AppState::ollama();

  let models = ollama.list_local_models().await?;

  Ok(Json(models))
}
