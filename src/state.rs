use std::{fs, path::PathBuf, sync::{Arc, LazyLock, Mutex}};
use serde::{Deserialize, Serialize};
use crate::result::Result;
use ollama_rs::{generation::chat::ChatMessage, models::LocalModel, Ollama};

pub static APP_DATA_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
  dirs::data_dir()
    .expect("Unable to locate data dir")
    .join("robo")
});

static APP_STATE_FILE: LazyLock<PathBuf> = LazyLock::new(|| {
  APP_DATA_DIR.join("state")
});

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Chat {
  pub title: String,
  pub saved_input: String,
  pub messages: Vec<ChatMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AppState {
  #[serde(skip)]
  pub ollama: Ollama,
  pub models: Arc<Mutex<Vec<LocalModel>>>,

  pub chats: Arc<Mutex<Vec<Chat>>>,
  pub active_chat: Option<usize>,
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
    let bytes = fs::read(&*APP_STATE_FILE)?;

    let mut state: AppState = bincode::deserialize(&bytes).unwrap_or_default();

    if let Some(ollama_url) = ollama_url {
      if let Some(pos) = ollama_url.rfind(':') {
        state.ollama = Ollama::new(
          &ollama_url[..pos],
          ollama_url[pos + 1..].parse()?,
        );
      }
    }

    Ok(state)
  }
}
