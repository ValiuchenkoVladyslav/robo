use crate::state::AppState;
use actix_web::{
  body::MessageBody,
  dev::{ServiceRequest, ServiceResponse},
  middleware::Next,
  web::Data,
  Error,
};

/// saves the app state to disk after each request
pub async fn data_persistance(
  mut req: ServiceRequest,
  next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
  let data = req.extract::<Data<AppState>>();

  let res = next.call(req).await;

  if let Ok(data) = data.await {
    if let Err(err) = data.save() {
      eprintln!("Failed to save app state: {}", err);
    }
  }

  res
}
