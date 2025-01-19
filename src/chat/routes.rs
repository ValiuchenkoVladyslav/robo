//! Chat API routes

use super::schemas::{Chat, ChatIden, Message, MessageIden};
use crate::{
  chat::schemas::Role,
  db::{get_cached, invalidate_cache, set_cache},
  result::{Error, Result},
  state::AppState,
  user::auth::Auth,
};
use actix_web::{
  delete, get, patch, post, put,
  web::{Json, Path},
  HttpResponse, HttpResponseBuilder,
};
use ollama::generation::chat::{request::ChatMessageRequest, ChatMessage};
use sea_query::{Expr, Order, PostgresQueryBuilder, Query};
use sqlx::{query, query_as, Row};
use std::fmt::Display;
use tracing::instrument;

fn chats_cache_key(pref: impl Display) -> String {
  format!("{pref}:get_chats")
}

fn messages_cache_key(pref: impl Display) -> String {
  format!("{pref}:get_messages")
}

/// get all chats
#[instrument(name = "chats::get_chats")]
#[get("/")]
pub async fn get_chats(Auth(user_id): Auth) -> Result<Json<Vec<Chat>>> {
  let db = AppState::db();

  let redis_key = chats_cache_key(user_id);

  if let Ok(cached) = get_cached(&redis_key) {
    return Ok(Json(cached));
  }

  let chats_query = Query::select()
    .from(ChatIden::Table)
    .columns(vec![
      ChatIden::Id,
      ChatIden::Model,
      ChatIden::Title,
      ChatIden::UserId,
    ])
    .and_where(Expr::col(ChatIden::UserId).eq(user_id))
    .to_string(PostgresQueryBuilder);

  let chats: Vec<Chat> = query_as(&chats_query).fetch_all(db).await?;

  set_cache(&redis_key, &chats, 460);

  Ok(Json(chats))
}

/// create new chat
#[instrument(name = "chats::create_chat")]
#[post("/")]
pub async fn create_chat(
  Auth(user_id): Auth,
  Json(mut new_chat): Json<Chat>,
) -> Result<Json<Chat>> {
  let db = AppState::db();

  new_chat.user_id = user_id;

  let create_chat_query = Query::insert()
    .into_table(ChatIden::Table)
    .columns([ChatIden::Title, ChatIden::Model, ChatIden::UserId])
    .values_panic([
      new_chat.title.clone().into(),
      new_chat.model.clone().into(),
      user_id.clone().into(),
    ])
    .returning_col(ChatIden::Id)
    .to_string(PostgresQueryBuilder);

  let chat_id = query(&create_chat_query).fetch_one(db).await?.get(0);

  new_chat.id = chat_id;

  invalidate_cache(&chats_cache_key(user_id));

  Ok(Json(new_chat))
}

/// edit chat
#[instrument(name = "chats::edit_chat")]
#[patch("/")]
pub async fn edit_chat(Auth(user_id): Auth, chat: Json<Chat>) -> Result<HttpResponseBuilder> {
  let db = AppState::db();

  let chat_update_query = Query::update()
    .table(ChatIden::Table)
    .values([
      (ChatIden::Title, chat.title.clone().into()),
      (ChatIden::Model, chat.model.clone().into()),
    ])
    .and_where(Expr::col(ChatIden::Id).eq(chat.id))
    .and_where(Expr::col(ChatIden::UserId).eq(user_id))
    .to_string(PostgresQueryBuilder);

  let res = query(&chat_update_query).execute(db).await?;

  if res.rows_affected() == 0 {
    return Err(Error::NotFound);
  }

  invalidate_cache(&chats_cache_key(user_id));

  Ok(HttpResponse::Ok())
}

/// delete chat
#[instrument(name = "chats::delete_chat")]
#[delete("/{chat_id}/")]
pub async fn delete_chat(Auth(user_id): Auth, chat_id: Path<i32>) -> Result<HttpResponseBuilder> {
  let db = AppState::db();

  let delete_chat_query = Query::delete()
    .from_table(ChatIden::Table)
    .and_where(Expr::col(ChatIden::Id).eq(chat_id.into_inner()))
    .and_where(Expr::col(ChatIden::UserId).eq(user_id))
    .to_string(PostgresQueryBuilder);

  query(&delete_chat_query).execute(db).await?;

  invalidate_cache(&chats_cache_key(user_id));

  Ok(HttpResponse::Ok())
}

/// get chat messages
#[instrument(name = "chats::get_messages")]
#[get("/{chat_id}/")]
pub async fn get_messages(Auth(user_id): Auth, chat_id: Path<i32>) -> Result<Json<Vec<Message>>> {
  let db = AppState::db();

  let chat_id = chat_id.into_inner();

  let redis_key = messages_cache_key(format!("{chat_id}-{user_id}"));

  if let Ok(cached) = get_cached(&redis_key) {
    return Ok(Json(cached));
  }

  let get_msgs_query = Query::select()
    .from(MessageIden::Table)
    .inner_join(
      ChatIden::Table,
      Expr::col((MessageIden::Table, MessageIden::ChatId)).equals((ChatIden::Table, ChatIden::Id)),
    )
    .columns([
      (MessageIden::Table, MessageIden::Id),
      (MessageIden::Table, MessageIden::Text),
      (MessageIden::Table, MessageIden::Role),
    ])
    .and_where(Expr::col(MessageIden::ChatId).eq(chat_id))
    .and_where(Expr::col((ChatIden::Table, ChatIden::UserId)).eq(user_id))
    .order_by(MessageIden::Id, Order::Asc)
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

  set_cache(&redis_key, &messages, 460);

  Ok(Json(messages))
}

/// send a message to a chat. returns ai response
#[instrument(name = "chats::send_message")]
#[put("/")]
pub async fn send_message(Auth(user_id): Auth, user_msg: Json<Message>) -> Result<Json<Message>> {
  let db = AppState::db();
  let ollama = AppState::ollama();

  let chat_id = user_msg.chat_id;

  // get chat AI model
  let chat_model_query = Query::select()
    .from(ChatIden::Table)
    .column(ChatIden::Model)
    .and_where(Expr::col(ChatIden::Id).eq(chat_id))
    .and_where(Expr::col(ChatIden::UserId).eq(user_id))
    .to_string(PostgresQueryBuilder);

  let chat_model = query(&chat_model_query).fetch_one(db).await?.try_get(0);

  let Ok(chat_model) = chat_model else {
    return Err(Error::NotFound);
  };

  // get all chat messages
  let get_messages_query = Query::select()
    .from(MessageIden::Table)
    .inner_join(
      ChatIden::Table,
      Expr::col((MessageIden::Table, MessageIden::ChatId)).equals((ChatIden::Table, ChatIden::Id)),
    )
    .columns([
      (MessageIden::Table, MessageIden::Id),
      (MessageIden::Table, MessageIden::Text),
      (MessageIden::Table, MessageIden::Role),
    ])
    .and_where(Expr::col(MessageIden::ChatId).eq(chat_id))
    .and_where(Expr::col((ChatIden::Table, ChatIden::UserId)).eq(user_id))
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

  invalidate_cache(&messages_cache_key(format!("{chat_id}-{user_id}")));

  Ok(Json(ai_res))
}
