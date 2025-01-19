//! DB utils

use crate::{
  chat::schemas::create_tables as create_chat_tables,
  result::{Error, Result},
  state,
  user::schemas::create_tables as create_user_tables,
};
use redis::Commands;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_str as from_json_str, to_string as to_json_string};
use tracing::{debug, error, info, instrument};

#[instrument]
pub async fn run_migrations() -> Result {
  let db = state::AppState::db();

  info!("Running user migrations");
  create_user_tables(db).await?;

  info!("Running chat migrations");
  create_chat_tables(db).await?;

  Ok(())
}

#[instrument]
pub fn get_cached<T: DeserializeOwned>(key: &str) -> Result<T> {
  let cached = state::AppState::redis().lock().get::<_, String>(&key);

  let Ok(cached) = cached else {
    debug!("cache miss");
    return Err(Error::NotFound);
  };

  debug!("cache hit");

  Ok(from_json_str(&cached)?)
}

#[instrument(skip(value))]
pub fn set_cache(key: &str, value: impl Serialize, ex: u64) {
  let res =
    state::AppState::redis()
      .lock()
      .set_ex::<_, _, ()>(key, to_json_string(&value).unwrap(), ex);

  // set chache errors should not impact the main flow
  if let Err(e) = res {
    error!("{e}");
  }
}

#[instrument]
pub fn invalidate_cache(key: &str) {
  let res = state::AppState::redis().lock().del::<_, ()>(key);

  // del chache errors should not impact the main flow
  if let Err(e) = res {
    error!("{e}");
  }
}
