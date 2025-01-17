use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// message schema
pub mod message {
  use super::*;

  #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
  #[sea_orm(table_name = "messages")]
  pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub chat_id: i32,
    pub role: String,
    #[sea_orm(column_type = "Text")]
    pub content: String,
  }

  #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
  pub enum Relation {
    #[sea_orm(
      belongs_to = "super::chat::Entity",
      from = "Column::ChatId",
      to = "super::chat::Column::Id"
    )]
    Chat,
  }

  impl Related<super::chat::Entity> for Entity {
    fn to() -> RelationDef {
      Relation::Chat.def()
    }
  }

  impl ActiveModelBehavior for ActiveModel {}
}

/// chat schema
pub mod chat {
  use super::*;

  #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
  #[sea_orm(table_name = "chats")]
  pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub title: String,
    pub model: String,
    #[sea_orm(column_type = "Text")]
    pub text: String,
  }

  #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
  pub enum Relation {
    #[sea_orm(has_many = "super::message::Entity")]
    Message,
  }

  impl Related<super::message::Entity> for Entity {
    fn to() -> RelationDef {
      Relation::Message.def()
    }
  }

  impl ActiveModelBehavior for ActiveModel {}  
}

pub async fn run_migrations() -> crate::result::Result {
  let db = crate::AppState::db();

  let backend = db.get_database_backend();
  let schema = sea_orm::Schema::new(backend);

  // Create tables
  let mut stm = schema.create_table_from_entity(chat::Entity);
  db.execute(backend.build(stm.if_not_exists())).await?;

  let mut stm = schema.create_table_from_entity(message::Entity);
  db.execute(backend.build(stm.if_not_exists())).await?;

  Ok(())
}
