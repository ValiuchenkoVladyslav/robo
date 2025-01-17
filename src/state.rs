use redis::Client as Redis;
use ollama_rs::{generation::chat::ChatMessage, models::LocalModel, Ollama};
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Chat {
  pub title: String,
  pub model: String,
  pub messages: Vec<ChatMessage>,
}

impl Chat {
  pub fn new(models: &Vec<LocalModel>) -> Self {
    Self {
      title: "New Chat".to_string(),
      model: models
        .first()
        .map_or("phi3.5", |model| &model.name)
        .to_string(),
      messages: vec![ChatMessage::system(
        "Reply shortly, no more than asked".to_string(),
      )],
    }
  }
}

struct AppState {
  pub ollama: Ollama,
  pub redis: Redis,
}

static APP_STATE: OnceLock<AppState> = OnceLock::new();

pub fn init_state(ollama: Ollama, redis: Redis) {
  if APP_STATE.set(AppState { ollama, redis }).is_err() {
    panic!("Failed to init app state!");
  }
}

pub fn state() -> &'static AppState {
  APP_STATE.get().expect("For some magical reason, the app state is not initialized!")
}
