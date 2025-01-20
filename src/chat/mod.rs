//! Chat API

mod routes;
pub mod schemas;

use crate::user::auth;
use axum::{
  middleware::from_fn,
  routing::{delete, get, patch, post},
  Router,
};

pub fn chat_router() -> Router {
  Router::new()
    .route("/chats", get(routes::get_chats))
    .route("/chats", post(routes::create_chat))
    .route("/chats", patch(routes::edit_chat))
    .route("/chats/{chat_id}", delete(routes::delete_chat))
    .route("/chats/{chat_id}", get(routes::get_messages))
    .route("/chats/{chat_id}", post(routes::send_message))
    .layer(from_fn(auth::auth_middleware))
}
