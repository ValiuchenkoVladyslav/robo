use std::{path::PathBuf, sync::LazyLock, fs};
use async_openai::{Client, config::OpenAIConfig};
use crate::result::Result;

static APP_DATA_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
  dirs::data_dir()
    .expect("Unable to locate data dir")
    .join("robo")
});

static APP_STATE_FILE: LazyLock<PathBuf> = LazyLock::new(|| {
  APP_DATA_DIR.join("state")
});

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct AppState {
  pub chats: Vec<String>,
  #[serde(skip)]
  pub active_chat: Option<usize>,

  pub openai_token: String,
  #[serde(skip)]
  pub openai_client: Client<OpenAIConfig>,

  pub input: String,
}

impl AppState {
  /// 'save' overlaps with [eframe::App::save]
  pub fn _save(&self) -> Result {
    let bytes = bincode::serialize(&self)?;

    fs::create_dir_all(&*APP_DATA_DIR)?;
    fs::write(&*APP_STATE_FILE, bytes)?;

    Ok(())
  }

  pub fn load(token: Option<String>) -> Result<Self> {
    let bytes = fs::read(&*APP_STATE_FILE)?;

    let token = token.unwrap_or_default();

    let Ok(mut state) = bincode::deserialize::<AppState>(&bytes) else {
      let mut state = AppState::default();

      state.openai_token = token;

      return Ok(state);
    };

    if !token.is_empty() {
      state.openai_token = token;
    }

    Ok(state)
  }
}
