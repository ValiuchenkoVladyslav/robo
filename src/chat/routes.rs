//! Chat API routes

use super::schemas::{Chat, ChatIden, Message, MessageIden};
use crate::{
  chat::schemas::Role,
  db::{cache, postgres},
  result::{Error, Result},
  state::ollama,
  user::auth::Auth,
};
use axum::{extract::Path, Json};
use ollama::generation::chat::{request::ChatMessageRequest, ChatMessage};
use sea_query::{Expr, Order, PostgresQueryBuilder, Query};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, Row};
use std::fmt::Display;
use tracing::instrument;
use ts_rs::TS;
use validator::Validate;

fn chats_cache_key(pref: impl Display) -> String {
  format!("{pref}:get_chats")
}

fn messages_cache_key(pref: impl Display) -> String {
  format!("{pref}:get_messages")
}

/// get all chats
#[instrument(name = "chats::get_chats")]
pub async fn get_chats(Auth(user_id): Auth) -> Result<Json<Vec<Chat>>> {
  let db = postgres(user_id);

  let redis_key = chats_cache_key(user_id);

  if let Some(cached) = cache::get(&redis_key) {
    return Ok(Json(cached));
  }

  let chats_query = Query::select()
    .from(ChatIden::Table)
    .columns([
      ChatIden::Id,
      ChatIden::Model,
      ChatIden::Title,
      ChatIden::UserId,
    ])
    .and_where(Expr::col(ChatIden::UserId).eq(user_id))
    .to_string(PostgresQueryBuilder);

  let chats: Vec<Chat> = query_as(&chats_query).fetch_all(db).await?;

  cache::set(&redis_key, &chats, 460);

  Ok(Json(chats))
}

#[derive(Debug, TS, Deserialize, Validate)]
#[ts(export, export_to = "./index.ts")]
pub struct CreateChatRequest {
  #[validate(length(min = 3, max = 255))]
  title: String,
  model: String,
}

/// create new chat
#[instrument(name = "chats::create_chat")]
pub async fn create_chat(
  Auth(user_id): Auth,
  Json(new_chat): Json<CreateChatRequest>,
) -> Result<Json<Chat>> {
  new_chat.validate()?;

  let db = postgres(user_id);

  let create_chat_query = Query::insert()
    .into_table(ChatIden::Table)
    .columns([ChatIden::Title, ChatIden::Model, ChatIden::UserId])
    .values_panic([
      new_chat.title.clone().into(),
      new_chat.model.clone().into(),
      user_id.into(),
    ])
    .returning_col(ChatIden::Id)
    .to_string(PostgresQueryBuilder);

  let chat_id = query(&create_chat_query).fetch_one(db).await?.get(0);

  let new_chat = Chat {
    id: chat_id,
    title: new_chat.title,
    model: new_chat.model,
    user_id,
  };

  cache::invalidate(&chats_cache_key(user_id));

  Ok(Json(new_chat))
}

/// edit chat
#[instrument(name = "chats::edit_chat")]
pub async fn edit_chat(Auth(user_id): Auth, Json(chat): Json<Chat>) -> Result<()> {
  let db = postgres(user_id);

  let chat_update_query = Query::update()
    .table(ChatIden::Table)
    .values([
      (ChatIden::Title, chat.title.into()),
      (ChatIden::Model, chat.model.into()),
    ])
    .and_where(Expr::col(ChatIden::Id).eq(chat.id))
    .and_where(Expr::col(ChatIden::UserId).eq(user_id))
    .to_string(PostgresQueryBuilder);

  let res = query(&chat_update_query).execute(db).await?;

  if res.rows_affected() == 0 {
    return Err(Error::NotFound);
  }

  cache::invalidate(&chats_cache_key(user_id));

  Ok(())
}

/// delete chat
#[instrument(name = "chats::delete_chat")]
pub async fn delete_chat(Auth(user_id): Auth, chat_id: Path<i32>) -> Result<()> {
  let db = postgres(user_id);

  let delete_chat_query = Query::delete()
    .from_table(ChatIden::Table)
    .and_where(Expr::col(ChatIden::Id).eq(chat_id.0))
    .and_where(Expr::col(ChatIden::UserId).eq(user_id))
    .to_string(PostgresQueryBuilder);

  query(&delete_chat_query).execute(db).await?;

  cache::invalidate(&chats_cache_key(user_id));

  Ok(())
}

/// get chat messages
#[instrument(name = "chats::get_messages")]
pub async fn get_messages(
  Auth(user_id): Auth,
  Path(chat_id): Path<i32>,
) -> Result<Json<Vec<Message>>> {
  let redis_key = messages_cache_key(format!("{chat_id}-{user_id}"));

  if let Some(cached) = cache::get(&redis_key) {
    return Ok(Json(cached));
  }

  let db = postgres(user_id);

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

  let messages = query(&get_msgs_query)
    .fetch_all(db)
    .await?
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

  cache::set(&redis_key, &messages, 460);

  Ok(Json(messages))
}

#[derive(Debug, TS, Deserialize, Serialize)]
#[ts(export, export_to = "./index.ts")]
pub struct SendMessageRequest {
  text: String,
}

/// send a message to a chat. returns ai response
#[instrument(name = "chats::send_message")]
pub async fn send_message(
  Auth(user_id): Auth,
  Path(chat_id): Path<i32>,
  Json(user_msg): Json<SendMessageRequest>,
) -> Result<Json<Message>> {
  let db = postgres(user_id);

  // get chat AI model
  let model_query = Query::select()
    .from(ChatIden::Table)
    .column(ChatIden::Model)
    .and_where(Expr::col(ChatIden::Id).eq(chat_id))
    .and_where(Expr::col(ChatIden::UserId).eq(user_id))
    .to_string(PostgresQueryBuilder);

  let Ok(chat_model) = query(&model_query).fetch_one(db).await?.try_get(0) else {
    return Err(Error::NotFound); // todo: better errors
  };

  // try get messages from cache
  let redis_key = messages_cache_key(format!("{chat_id}-{user_id}"));

  let mut messages = cache::get::<Vec<Message>>(&redis_key)
    .unwrap_or_default()
    .into_iter()
    .map(|msg| {
      let message = match msg.role {
        Role::User => ChatMessage::user,
        Role::Ai => ChatMessage::assistant,
        Role::System => ChatMessage::system,
      };

      message(msg.text)
    })
    .collect::<Vec<ChatMessage>>();

  // if cache is empty, fetch messages from db
  if messages.is_empty() {
    let get_messages_query = Query::select()
      .from(MessageIden::Table)
      .inner_join(
        ChatIden::Table,
        Expr::col((MessageIden::Table, MessageIden::ChatId))
          .equals((ChatIden::Table, ChatIden::Id)),
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

    messages = query(&get_messages_query)
      .fetch_all(db)
      .await?
      .into_iter()
      .filter_map(|row| {
        let msg = match Role::from_i16(row.get("role")).ok()? {
          Role::User => ChatMessage::user,
          Role::Ai => ChatMessage::assistant,
          Role::System => ChatMessage::system,
        };

        Some(msg(row.get("text")))
      })
      .collect();
  }

  // send chat messages to ollama
  let user_msg = user_msg.text;

  ollama()
    .send_chat_messages_with_history(
      &mut messages,
      ChatMessageRequest::new(chat_model, vec![ChatMessage::user(user_msg.clone())]),
    )
    .await?;

  let ai_res = messages.last().unwrap();

  // insert new messages
  let insert_msgs_query = Query::insert()
    .into_table(MessageIden::Table)
    .columns([MessageIden::Text, MessageIden::Role, MessageIden::ChatId])
    .values_panic([user_msg.into(), Role::User.into(), chat_id.into()])
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

  cache::invalidate(&redis_key);

  Ok(Json(ai_res))
}
