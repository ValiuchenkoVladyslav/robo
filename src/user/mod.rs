//! User API

pub mod auth;
mod routes;
pub mod schemas;

use axum::{routing::post, Router};
use routes::{create_user, login_user};

pub fn user_router() -> Router {
  Router::new()
    .route("/register", post(create_user))
    .route("/login", post(login_user))
}
