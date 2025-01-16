use crate::{
  ollama::{ask_ai, list_models},
  result::Error,
  state::{AppState, Chat},
};
use actix_web::{
  delete, get, patch, post, put,
  web::{Data, Json, Path},
  HttpResponse, Responder, Result,
};
use ollama_rs::{generation::chat::ChatMessage, models::LocalModel};

/// get all chats
#[get("/chats")]
pub async fn get_chats(data: Data<AppState>) -> impl Responder {
  let chats = data.chats.read();

  HttpResponse::Ok().json(chats.clone())
}

/// create a new chat
#[post("/chats")]
pub async fn create_chat(data: Data<AppState>) -> Json<Chat> {
  let chats = &data.chats;

  chats.write().push(Chat::new(&data.models));

  Json(chats.read().last().unwrap().clone())
}

/// edit chat
#[patch("/chats/{chat}")]
pub async fn edit_chat(
  data: Data<AppState>,
  chat: Path<usize>,
  chat_data: Json<Chat>,
) -> Result<Json<Chat>> {
  let mut chats = data.chats.write();
  let chat = chat.into_inner();

  if chat > chats.len() {
    return Err(Error::NotFound)?;
  }

  let chat_data = chat_data.into_inner();

  chats[chat] = chat_data.clone();

  Ok(Json(chat_data))
}

/// send a message to a chat
#[put("/chats/{chat}")]
pub async fn send_message(
  data: Data<AppState>,
  chat: Path<usize>,
  message: Json<ChatMessage>,
) -> Result<Json<ChatMessage>> {
  let chats = &data.chats;
  let chat = chat.into_inner();

  if chat > chats.read().len() {
    return Err(Error::NotFound)?;
  }

  chats.write()[chat].messages.push(message.into_inner());

  ask_ai(data.ollama, chats, chat).await?;

  let msg = chats.read()[chat].messages.last().unwrap().clone();

  Ok(Json(msg))
}

/// delete chat
#[delete("/chats/{chat}")]
pub async fn delete_chat(data: Data<AppState>, chat: Path<usize>) -> impl Responder {
  let mut chats = data.chats.write();
  let chat = chat.into_inner();

  if chat < chats.len() {
    chats.remove(chat);

    return HttpResponse::Ok();
  }

  HttpResponse::NotFound()
}

/// list ollama models
#[get("/models")]
pub async fn get_models(data: Data<AppState>) -> Result<Json<Vec<LocalModel>>> {
  list_models(data.ollama, &data.models).await?;

  Ok(Json(data.models.read().clone()))
}
