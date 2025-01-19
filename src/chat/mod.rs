use crate::user::auth::auth_middleware;
use actix_web::{
  body::MessageBody,
  dev::{ServiceFactory, ServiceRequest, ServiceResponse},
  middleware::from_fn,
};

mod routes;
pub mod schemas;

/// Chats API service
pub fn service() -> actix_web::Scope<
  impl ServiceFactory<
    ServiceRequest,
    Config = (),
    Response = ServiceResponse<impl MessageBody>,
    Error = actix_web::Error,
    InitError = (),
  >,
> {
  actix_web::web::scope("/chats")
    .wrap(from_fn(auth_middleware))
    .service(routes::get_chats)
    .service(routes::create_chat)
    .service(routes::edit_chat)
    .service(routes::delete_chat)
    .service(routes::get_messages)
    .service(routes::send_message)
}
