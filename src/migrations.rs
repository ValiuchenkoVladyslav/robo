use crate::{chat::schemas::create_tables as create_chat_tables, result, state};

pub async fn run_migrations() -> result::Result {
  let db = state::AppState::db();

  create_chat_tables(db).await?;

  Ok(())
}
