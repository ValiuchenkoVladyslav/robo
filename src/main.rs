// mod api;
mod result;
mod db;
mod state;
mod schemas;

use state::AppState;
use actix_web::{App, HttpServer};
use std::env::var;

#[tokio::main]
async fn main() -> result::Result {
  let ollama_url = var("OLLAMA_URL").expect("OLLAMA_URL env var");
  let redis_url = var("REDIS_URL").expect("REDIS_URL env var");
  let postgres_url = var("POSTGRES_URL").expect("POSTGRES_URL env");
  let app_port: u16 = var("APP_PORT").expect("APP_PORT env var").parse()?;

  println!("Initializing app state...");
  AppState::new(ollama_url, redis_url, postgres_url).await?;

  println!("Running migrations...");
  schemas::run_migrations().await?;

  println!("Starting web server...");
  HttpServer::new(|| {
    App::new()
      // .service(api::routes::get_chats)
      // .service(api::routes::create_chat)
      // .service(api::routes::edit_chat)
      // .service(api::routes::send_message)
      // .service(api::routes::delete_chat)
  })
    .bind(("127.0.0.1", app_port))?
    .run()
    .await?;

  Ok(())
}
