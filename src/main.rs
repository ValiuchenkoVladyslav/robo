mod chat;
mod migrations;
mod ollama;
mod result;
mod state;

use actix_web::{
  middleware::{NormalizePath, TrailingSlash},
  App, HttpServer,
};
use std::env::var;
use tracing::Level;

#[tokio::main]
async fn main() -> result::Result {
  #[cfg(debug_assertions)]
  {
    tracing_subscriber::fmt()
      .with_max_level(Level::DEBUG)
      .init();

    dotenv::from_filename(".env.dev").ok();
  }

  #[cfg(not(debug_assertions))]
  {
    tracing_subscriber::fmt().init();

    dotenv::dotenv().ok();
  }

  let ollama_url = var("OLLAMA_URL").expect("OLLAMA_URL env var");
  let redis_url = var("REDIS_URL").expect("REDIS_URL env var");
  let postgres_url = var("POSTGRES_URL").expect("POSTGRES_URL env");

  state::AppState::init(ollama_url, redis_url, postgres_url).await?;

  migrations::run_migrations().await?;

  HttpServer::new(|| {
    App::new()
      .wrap(NormalizePath::new(TrailingSlash::Always))
      .service(ollama::get_models)
      .service(chat::service())
  })
  .bind(("127.0.0.1", 3000))?
  .run()
  .await?;

  Ok(())
}
