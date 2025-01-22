//! DB utils

use crate::{
  chat::schemas::create_tables as create_chat_tables,
  result::Result,
  state::{postgres, redis},
  user::schemas::create_tables as create_user_tables,
};
use redis::Commands;
use serde::{de::DeserializeOwned, Serialize};
use serde_json as json;
use tracing::{debug, error, info, instrument};

#[instrument]
pub async fn run_migrations() -> Result {
  let db = postgres();

  info!("Running user migrations");
  create_user_tables(db).await?;

  info!("Running chat migrations");
  create_chat_tables(db).await?;

  Ok(())
}

#[instrument]
pub fn get_cached<T: DeserializeOwned>(key: &str) -> Option<T> {
  let cached = redis().lock().get::<_, String>(&key);

  let Ok(cached) = cached else {
    debug!("cache miss");
    return None;
  };

  debug!("cache hit");

  match json::from_str(&cached) {
    Ok(v) => Some(v),
    Err(e) => {
      error!("{e}");

      None
    }
  }
}

#[instrument(skip(value))]
pub fn set_cache(key: &str, value: impl Serialize, ex: u64) {
  let value = match json::to_string(&value) {
    Ok(v) => v,
    Err(e) => {
      error!("{e}");

      return;
    }
  };

  let res = redis().lock().set_ex::<_, _, ()>(key, value, ex);

  // set chache errors should not impact the main flow
  if let Err(e) = res {
    error!("{e}");
  }
}

/// invalidate a cache key without blocking the main flow
#[instrument]
pub fn invalidate_cache(key: &str) {
  let key = key.to_string();

  tokio::spawn(async move {
    let res = redis().lock().del::<_, ()>(key);

    if let Err(e) = res {
      error!("{e}");
    }
  });
}
