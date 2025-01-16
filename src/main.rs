#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(feature = "api")]
mod api;
mod args;
#[cfg(feature = "gui")]
mod gui;
mod ollama;
mod result;
mod state;

use clap::Parser;
use state::AppState;

#[tokio::main]
async fn main() -> result::Result {
  let args = args::Args::parse();

  let app_state = AppState::load(args.ollama_url)?;

  // refetch models without blocking main thread
  tokio::spawn({
    let ollama = app_state.ollama;
    let models = app_state.models.clone();

    async move {
      if let Err(err) = ollama::list_models(ollama, &models).await {
        eprintln!(
          "Failed to list models: {}\nMake sure Ollama is running!",
          err
        );
      }
    }
  });

  // run the GUI by default or API if specified in args
  #[cfg(all(feature = "api", feature = "gui"))]
  if args.api_mode {
    api::run_server(app_state, args.port).await?;
  } else {
    gui::run_gui(app_state)?;
  }

  #[cfg(all(feature = "api", not(feature = "gui")))]
  api::run_server(app_state, args.port).await?;

  #[cfg(all(not(feature = "api"), feature = "gui"))]
  gui::run_gui(app_state)?;

  Ok(())
}
