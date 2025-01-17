// mod api;
mod args;
// mod ollama;
mod result;
mod state;

use clap::Parser;
use state::init_state;
use actix_web::{App, HttpServer};

#[tokio::main]
async fn main() -> result::Result {
  let args = args::Args::parse();

  init_state(
    {
      let port_pos = args.ollama_url.rfind(":").expect("No port found in Ollama url!");

      ollama_rs::Ollama::new(
        &args.ollama_url[..port_pos],
        args.ollama_url[port_pos + 1..].parse()?,
      )
    },
    redis::Client::open(args.redis_url)?,
  );

  HttpServer::new(move || {
    App::new()
      // .service(api::routes::get_chats)
      // .service(api::routes::create_chat)
      // .service(api::routes::edit_chat)
      // .service(api::routes::send_message)
      // .service(api::routes::delete_chat)
  })
    .bind(("127.0.0.1", args.port))?
    .run()
    .await?;

  Ok(())
}
