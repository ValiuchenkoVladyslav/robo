//! Ollama API

use crate::{result::Result, state::AppState};
use axum::{routing::get, Json, Router};
use ollama::models::LocalModel;

async fn get_models() -> Result<Json<Vec<LocalModel>>> {
  let ollama = AppState::ollama();

  let models = ollama.list_local_models().await?;

  Ok(Json(models))
}

pub fn ollama_router() -> Router {
  Router::new().route("/models", get(get_models))
}
