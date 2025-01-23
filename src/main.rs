mod chat;
mod db;
mod jwt;
mod ollama;
mod result;
mod state;
mod user;

use axum::Router;
use std::{
  env::{current_exe, var},
  path::Path,
};
use tower_http::{
  cors::CorsLayer,
  services::{ServeDir, ServeFile},
};
use tracing::info;

// runs inside musl container
#[cfg(not(all(target_arch = "x86_64", target_os = "linux", target_env = "musl")))]
const HOST: &str = "127.0.0.1";

#[cfg(all(target_arch = "x86_64", target_os = "linux", target_env = "musl"))]
const HOST: &str = "0.0.0.0";

#[tokio::main]
async fn main() -> result::Result {
  #[cfg(debug_assertions)]
  {
    tracing_subscriber::fmt()
      .with_max_level(tracing::Level::DEBUG)
      .init();

    dotenv::from_filename(".env.dev").ok();
  }

  #[cfg(not(debug_assertions))]
  {
    tracing_subscriber::fmt().init();

    dotenv::dotenv().ok();
  }

  db::init().await;
  state::init(
    var("OLLAMA_URL").expect("OLLAMA_URL env var"),
    var("JWT_SECRET").expect("JWT_SECRET env"),
  );

  db::run_migrations().await?;

  let app = Router::new()
    .nest(
      "/api",
      Router::new()
        .merge(ollama::ollama_router())
        .merge(user::user_router())
        .merge(chat::chat_router())
        .layer(CorsLayer::permissive()),
    )
    .fallback_service({
      let static_dir = current_exe()?
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("build");

      ServeDir::new(&static_dir).fallback(ServeFile::new(static_dir.join("index.html")))
    });

  let listener = tokio::net::TcpListener::bind((HOST, 3000)).await?;
  info!("listening on: {}", listener.local_addr()?);

  axum::serve(listener, app).await?;

  Ok(())
}
