//! User DB schemas

use crate::result::Result;
use sea_query::{enum_def, ColumnDef, PostgresQueryBuilder, Table};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[enum_def]
#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct User {
  // May be absent when creating a new user
  #[serde(default)]
  pub id: i32,
  pub name: String,
  pub email: String,
  pub password: String,
}

pub async fn create_tables(pool: &PgPool) -> Result {
  let user_table = Table::create()
    .table(UserIden::Table)
    .if_not_exists()
    .col(
      ColumnDef::new(UserIden::Id)
        .integer()
        .not_null()
        .auto_increment()
        .primary_key(),
    )
    .col(ColumnDef::new(UserIden::Name).string().not_null())
    .col(ColumnDef::new(UserIden::Email).string().not_null())
    .col(ColumnDef::new(UserIden::Password).string().not_null())
    .to_string(PostgresQueryBuilder);

  sqlx::query(&user_table).execute(pool).await?;

  Ok(())
}
