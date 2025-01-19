mod chat;
mod db;
mod jwt;
mod ollama;
mod result;
mod state;
mod user;

use actix_web::{
  middleware::{NormalizePath, TrailingSlash},
  App, HttpServer,
};
use std::env::var;

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

  state::AppState::init(
    var("OLLAMA_URL").expect("OLLAMA_URL env var"),
    var("REDIS_URL").expect("REDIS_URL env var"),
    var("POSTGRES_URL").expect("POSTGRES_URL env"),
    var("JWT_SECRET").expect("JWT_SECRET env"),
  )
  .await?;

  db::run_migrations().await?;

  HttpServer::new(|| {
    App::new()
      .wrap(NormalizePath::new(TrailingSlash::Always))
      .service(ollama::get_models)
      .service(user::service())
      .service(chat::service())
  })
  .bind((HOST, 3000))?
  .run()
  .await?;

  Ok(())
}
