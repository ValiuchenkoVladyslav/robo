//! Chat API routes

use super::schemas::{Chat, ChatIden, Message, MessageIden};
use crate::{
  chat::schemas::Role,
  result::{Error, Result},
  state::AppState,
};
use actix_web::{
  delete, get, patch, post, put,
  web::{Json, Path},
  HttpResponse, HttpResponseBuilder,
};
use ollama_rs::generation::chat::{request::ChatMessageRequest, ChatMessage};
use sea_query::{Expr, Order, PostgresQueryBuilder, Query};
use sqlx::{query, query_as, Row};

/// get all chats
#[get("/")]
pub async fn get_chats() -> Result<Json<Vec<Chat>>> {
  let db = AppState::db();

  let chats_query = Query::select()
    .columns(vec![ChatIden::Id, ChatIden::Model, ChatIden::Title])
    .from(ChatIden::Table)
    .to_string(PostgresQueryBuilder);

  let chats: Vec<Chat> = query_as(&chats_query).fetch_all(db).await?;

  Ok(Json(chats))
}

/// create new chat
#[post("/")]
pub async fn create_chat(new_chat: Json<Chat>) -> Result<Json<Chat>> {
  let db = AppState::db();

  let mut new_chat = new_chat.into_inner();

  let create_chat_query = Query::insert()
    .into_table(ChatIden::Table)
    .columns([ChatIden::Title, ChatIden::Model])
    .values_panic([new_chat.title.clone().into(), new_chat.model.clone().into()])
    .returning_col(ChatIden::Id)
    .to_string(PostgresQueryBuilder);

  let chat_id = query(&create_chat_query).fetch_one(db).await?.get(0);

  new_chat.id = chat_id;

  Ok(Json(new_chat))
}

/// edit chat
#[patch("/")]
pub async fn edit_chat(chat: Json<Chat>) -> Result<HttpResponseBuilder> {
  let db = AppState::db();

  let chat_update_query = Query::update()
    .table(ChatIden::Table)
    .values([
      (ChatIden::Title, chat.title.clone().into()),
      (ChatIden::Model, chat.model.clone().into()),
    ])
    .and_where(Expr::col(ChatIden::Id).eq(chat.id))
    .to_string(PostgresQueryBuilder);

  query(&chat_update_query).execute(db).await?;

  Ok(HttpResponse::Ok())
}

/// delete chat
#[delete("/{chat_id}")]
pub async fn delete_chat(chat_id: Path<i32>) -> Result<HttpResponseBuilder> {
  let db = AppState::db();

  let delete_chat_query = Query::delete()
    .from_table(ChatIden::Table)
    .and_where(Expr::col(ChatIden::Id).eq(chat_id.into_inner()))
    .to_string(PostgresQueryBuilder);

  query(&delete_chat_query).execute(db).await?;

  Ok(HttpResponse::Ok())
}

/// get chat messages
#[get("/{chat_id}")]
pub async fn get_messages(chat_id: Path<i32>) -> Result<Json<Vec<Message>>> {
  let db = AppState::db();

  let chat_id = chat_id.into_inner();

  let get_msgs_query = Query::select()
    .from(MessageIden::Table)
    .columns([MessageIden::Id, MessageIden::Text, MessageIden::Role])
    .and_where(Expr::col(MessageIden::ChatId).eq(chat_id))
    .to_string(PostgresQueryBuilder);

  let rows = query(&get_msgs_query).fetch_all(db).await?;

  let messages = rows
    .into_iter()
    .filter_map(|row| {
      let msg = Message {
        id: row.get("id"),
        text: row.get("text"),
        role: Role::from_i16(row.get("role")).ok()?,
        chat_id,
      };

      Some(msg)
    })
    .collect();

  Ok(Json(messages))
}

/// send a message to a chat. returns ai response
#[put("/")]
pub async fn send_message(user_msg: Json<Message>) -> Result<Json<Message>> {
  let db = AppState::db();
  let ollama = AppState::ollama();

  let chat_id = user_msg.chat_id;

  // get chat AI model
  let chat_model_query = Query::select()
    .from(ChatIden::Table)
    .column(ChatIden::Model)
    .and_where(Expr::col(ChatIden::Id).eq(chat_id))
    .to_string(PostgresQueryBuilder);

  let chat_model = query(&chat_model_query).fetch_one(db).await?.try_get(0);

  let Ok(chat_model) = chat_model else {
    return Err(Error::NotFound);
  };

  // get all chat messages
  let get_messages_query = Query::select()
    .from(MessageIden::Table)
    .columns([MessageIden::Id, MessageIden::Text, MessageIden::Role])
    .and_where(Expr::col(MessageIden::ChatId).eq(chat_id))
    .order_by(MessageIden::Id, Order::Asc)
    .to_string(PostgresQueryBuilder);

  let rows = query(&get_messages_query).fetch_all(db).await?;

  // convert rows to ollama messages
  let mut messages = rows
    .into_iter()
    .filter_map(|row| {
      let role = Role::from_i16(row.get("role")).ok()?;
      let text = row.get("text");

      let msg = match role {
        Role::User => ChatMessage::user(text),
        Role::Ai => ChatMessage::assistant(text),
        Role::System => ChatMessage::system(text),
      };

      Some(msg)
    })
    .collect::<Vec<ChatMessage>>();

  // send chat messages to ollama
  ollama
    .send_chat_messages_with_history(
      &mut messages,
      ChatMessageRequest::new(chat_model, vec![ChatMessage::user(user_msg.text.clone())]),
    )
    .await?;

  let ai_res = messages.last().unwrap();

  // insert new messages
  let insert_msgs_query = Query::insert()
    .into_table(MessageIden::Table)
    .columns([MessageIden::Text, MessageIden::Role, MessageIden::ChatId])
    .values_panic([
      user_msg.text.clone().into(),
      Role::User.into(),
      chat_id.into(),
    ])
    .values_panic([
      ai_res.content.clone().into(),
      Role::Ai.into(),
      chat_id.into(),
    ])
    .to_string(PostgresQueryBuilder);

  query(&insert_msgs_query).execute(db).await?;

  let ai_res = Message {
    id: messages.len() as i32,
    text: ai_res.content.clone(),
    role: Role::Ai,
    chat_id,
  };

  Ok(Json(ai_res))
}
