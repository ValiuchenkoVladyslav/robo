use std::{path::PathBuf, sync::LazyLock, fs};
use serde::{Serialize, Deserialize};
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
  /// ai chats
  pub chats: Vec<String>,

  /// OpenAI API token
  pub openai_token: String,
}

impl AppState {
  /// 'save' overlaps with [eframe::App::save]
  pub fn _save(&self) -> Result {
    let bytes = bincode::serialize(&self)?;

    fs::create_dir_all(&*APP_DATA_DIR)?;
    fs::write(&*APP_STATE_FILE, bytes)?;

    Ok(())
  }

  pub fn load() -> Result<Self> {
    let bytes = fs::read(&*APP_STATE_FILE)?;

    let app_state = bincode::deserialize(&bytes)?;

    Ok(app_state)
  }
}
