//! User DB schemas

use crate::result::Result;
use sea_query::{enum_def, ColumnDef, PostgresQueryBuilder, Table};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[enum_def]
#[derive(ts_rs::TS, Debug, Deserialize, Serialize, FromRow)]
#[ts(export, export_to = "./index.ts")]
pub struct User {
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
      // not using auto increment. instead we rely on internal counter
      ColumnDef::new(UserIden::Id)
        .integer()
        .not_null()
        .primary_key(),
    )
    .col(ColumnDef::new(UserIden::Name).string().not_null())
    .col(ColumnDef::new(UserIden::Email).string().not_null())
    .col(ColumnDef::new(UserIden::Password).string().not_null())
    .to_string(PostgresQueryBuilder);

  sqlx::query(&user_table).execute(pool).await?;

  Ok(())
}
