mod middleware;
mod routes;

use crate::state::AppState;
use actix_web::{middleware::from_fn, web::Data, App, HttpServer};

pub async fn run_server(app_state: AppState, port: u16) -> std::io::Result<()> {
  let app_state = Data::new(app_state);

  HttpServer::new(move || {
    App::new()
      .app_data(app_state.clone())
      .wrap(from_fn(middleware::data_persistance))
      .service(routes::get_chats)
      .service(routes::create_chat)
      .service(routes::edit_chat)
      .service(routes::send_message)
      .service(routes::delete_chat)
  })
  .bind(("127.0.0.1", port))?
  .run()
  .await
}
