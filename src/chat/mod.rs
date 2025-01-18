mod routes;
pub mod schemas;

/// Chats API service
pub fn service() -> actix_web::Scope {
  actix_web::web::scope("/chats")
    .service(routes::get_chats)
    .service(routes::create_chat)
    .service(routes::edit_chat)
    .service(routes::delete_chat)
    .service(routes::get_messages)
    .service(routes::send_message)
}
