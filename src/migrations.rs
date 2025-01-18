use crate::{chat::schemas::create_tables as create_chat_tables, result, state};
use tracing::{info, instrument};

#[instrument]
pub async fn run_migrations() -> result::Result {
  let db = state::AppState::db();

  info!("Running chat migrations");
  create_chat_tables(db).await?;

  Ok(())
}
