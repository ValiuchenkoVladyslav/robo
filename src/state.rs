use std::{fs, path::PathBuf, sync::{Arc, LazyLock}};
use serde::{Deserialize, Serialize};
use ollama_rs::{generation::chat::ChatMessage, models::LocalModel, Ollama};
use crate::result::Result;
use parking_lot::RwLock;

static APP_DATA_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
  dirs::data_dir()
    .expect("Unable to locate data dir")
    .join("robo")
});

static APP_STATE_FILE: LazyLock<PathBuf> = LazyLock::new(|| {
  APP_DATA_DIR.join("state")
});

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Chat {
  pub title: String,
  pub saved_input: String,
  pub model: String,
  pub messages: Vec<ChatMessage>,
}

impl Chat {
  pub fn new(models: Arc<RwLock<Vec<LocalModel>>>) -> Self {
    Self {
      title: "New Chat".to_string(),
      saved_input: String::new(),
      model: models.read().first()
        .map_or("phi3.5", |model| &model.name)
        .to_string(),
      messages: vec![ChatMessage::system("Reply shortly, no more than asked".to_string())],
    }
  }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AppState {
  #[serde(skip)]
  pub ollama: Arc<Ollama>,
  pub models: Arc<RwLock<Vec<LocalModel>>>,

  pub chats: Arc<RwLock<Vec<Chat>>>,
  pub active_chat: usize,
}

impl AppState {
  /// 'save' overlaps with [eframe::App::save]
  pub fn _save(&self) -> Result {
    let bytes = bincode::serialize(&self)?;

    fs::create_dir_all(&*APP_DATA_DIR)?;
    fs::write(&*APP_STATE_FILE, bytes)?;

    Ok(())
  }

  pub fn load(ollama_url: Option<String>) -> Result<Self> {
    let mut state: AppState = fs::read(&*APP_STATE_FILE)
      .map(|bytes| bincode::deserialize(&bytes).unwrap_or_default())
      .unwrap_or_default();

    if let Some(ollama_url) = ollama_url {
      if let Some(pos) = ollama_url.rfind(':') {
        state.ollama = Arc::new(
          Ollama::new(
            &ollama_url[..pos],
            ollama_url[pos + 1..].parse()?,
          )
        );
      }
    }

    tokio::spawn({
      let ollama = state.ollama.clone();
      let models = state.models.clone();

      async move {
        *models.write() = ollama.list_local_models().await.expect("Failed to list models");
      }
    });

    Ok(state)
  }
}
