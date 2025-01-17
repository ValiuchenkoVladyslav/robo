#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api;
mod args;
mod ollama;
mod result;
mod state;

use clap::Parser;
use state::AppState;

#[tokio::main]
async fn main() -> result::Result {
  let args = args::Args::parse();

  let app_state = AppState::load(args.ollama_url)?;

  ollama::list_models(app_state.ollama, &app_state.models).await?;

  api::run_server(app_state, args.port).await?;

  Ok(())
}
