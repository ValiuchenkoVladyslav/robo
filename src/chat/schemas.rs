//! Chat DB schemas

use crate::{
  result::{Error, Result},
  user::schemas::UserIden,
};
use sea_query::{enum_def, ColumnDef, ForeignKey, PostgresQueryBuilder, Table, Value};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(Debug, Deserialize, Serialize)]
pub enum Role {
  User,
  Ai,
  System,
}

impl Role {
  /// Convert an i16 to a Role. Required for deserializing from DB.
  pub fn from_i16(role: i16) -> Result<Self> {
    match role {
      0 => Ok(Role::User),
      1 => Ok(Role::Ai),
      2 => Ok(Role::System),
      _ => Err(Error::InvalidRole),
    }
  }
}

impl From<Role> for Value {
  fn from(role: Role) -> Self {
    Value::SmallUnsigned(Some(role as u16))
  }
}

#[enum_def]
#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Chat {
  // May be absent when creating a new chat
  #[serde(default)]
  pub id: i32,
  pub title: String,
  pub model: String,
  pub user_id: i32,
}

#[enum_def]
#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Message {
  // May be absent when creating a new message
  #[serde(default)]
  pub id: i32,
  pub role: Role,
  pub text: String,
  pub chat_id: i32,
}

pub async fn create_tables(pool: &PgPool) -> Result {
  let chat_table = Table::create()
    .table(ChatIden::Table)
    .if_not_exists()
    .col(
      ColumnDef::new(ChatIden::Id)
        .integer()
        .not_null()
        .auto_increment()
        .primary_key(),
    )
    .col(ColumnDef::new(ChatIden::Model).string().not_null())
    .col(ColumnDef::new(ChatIden::Title).string().not_null())
    .col(ColumnDef::new(ChatIden::UserId).integer().not_null())
    .foreign_key(
      ForeignKey::create()
        .from(ChatIden::Table, ChatIden::UserId)
        .to(UserIden::Table, UserIden::Id)
        .on_delete(sea_query::ForeignKeyAction::Cascade),
    )
    .to_string(PostgresQueryBuilder);

  let message_table = Table::create()
    .table(MessageIden::Table)
    .if_not_exists()
    .col(
      ColumnDef::new(MessageIden::Id)
        .integer()
        .not_null()
        .auto_increment()
        .primary_key(),
    )
    .col(
      ColumnDef::new(MessageIden::Role)
        .small_unsigned()
        .not_null(),
    )
    .col(ColumnDef::new(MessageIden::Text).string().not_null())
    .col(ColumnDef::new(MessageIden::ChatId).integer().not_null())
    .foreign_key(
      ForeignKey::create()
        .from(MessageIden::Table, MessageIden::ChatId)
        .to(ChatIden::Table, ChatIden::Id)
        .on_delete(sea_query::ForeignKeyAction::Cascade),
    )
    .to_string(PostgresQueryBuilder);

  sqlx::query(&chat_table).execute(pool).await?;
  sqlx::query(&message_table).execute(pool).await?;

  Ok(())
}
