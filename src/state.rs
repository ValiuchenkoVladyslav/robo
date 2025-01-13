use std::{path::PathBuf, sync::LazyLock, fs, env};
use serde::{Serialize, Deserialize};
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

#[derive(Default, Serialize, Deserialize, Debug)]
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

    let mut app_state: AppState = bincode::deserialize(&bytes).unwrap_or_else(|_| {
      env::set_var("OPENAI_API_KEY", &token);

      Default::default()
    });

    if !token.is_empty() {
      app_state.openai_token = token;
    }

    Ok(app_state)
  }
}
