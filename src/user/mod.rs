pub mod auth;
mod routes;
pub mod schemas;

/// Users API service
pub fn service() -> actix_web::Scope {
  actix_web::web::scope("/users")
    .service(routes::create_user)
    .service(routes::login_user)
}
