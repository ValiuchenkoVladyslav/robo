use crate::result::Result;
use ollama_rs::{generation::chat::ChatMessage, models::LocalModel, Ollama};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::{
  fs,
  path::PathBuf,
  sync::{Arc, LazyLock},
};

static APP_DATA_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
  dirs::data_dir()
    .expect("Unable to locate data dir")
    .join("robo")
});

static APP_STATE_FILE: LazyLock<PathBuf> = LazyLock::new(|| APP_DATA_DIR.join("state"));

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Chat {
  pub title: String,
  pub saved_input: String,
  pub model: String,
  pub messages: Vec<ChatMessage>,
}

impl Chat {
  pub fn new(models: &Arc<RwLock<Vec<LocalModel>>>) -> Self {
    Self {
      title: "New Chat".to_string(),
      saved_input: String::new(),
      model: models
        .read()
        .first()
        .map_or("phi3.5", |model| &model.name)
        .to_string(),
      messages: vec![ChatMessage::system(
        "Reply shortly, no more than asked".to_string(),
      )],
    }
  }
}

// at least we can avoid Arc with Ollama
fn default_ollama() -> &'static Ollama {
  Box::leak(Box::new(Ollama::default()))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppState {
  #[serde(skip, default = "default_ollama")]
  pub ollama: &'static Ollama,
  pub models: Arc<RwLock<Vec<LocalModel>>>,

  pub chats: Arc<RwLock<Vec<Chat>>>,
  pub active_chat: usize,
}

impl Default for AppState {
  fn default() -> Self {
    Self {
      ollama: default_ollama(),
      models: Default::default(),
      chats: Default::default(),
      active_chat: 0,
    }
  }
}

impl AppState {
  pub fn save(&self) -> Result {
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
        let ollama = Ollama::new(&ollama_url[..pos], ollama_url[pos + 1..].parse()?);

        state.ollama = Box::leak(Box::new(ollama));
      }
    }

    Ok(state)
  }
}
