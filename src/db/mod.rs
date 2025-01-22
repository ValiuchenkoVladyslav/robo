//! DB module
//!
//! - App DB is split into two shards based on user id parity.
//! - Also we use Redis for caching.

pub mod cache;

use crate::{
  chat::schemas::create_tables as create_chat_tables, result::Result,
  user::schemas::create_tables as create_user_tables,
};
use parking_lot::Mutex;
use redis::Client as Redis;
use sqlx::{query, PgPool, Row};
use std::{
  env::var,
  sync::{
    atomic::{AtomicI32, Ordering},
    OnceLock,
  },
};
use tracing::{info, instrument};

pub struct DBState {
  /// Redis connection
  redis: Mutex<Redis>,

  /// Postgres shard for odd user ids
  shard1: PgPool,
  /// Postgres shard for even user ids
  shard2: PgPool,

  /// Max user id among all shards. Used to decide where to put new users
  max_uid: AtomicI32,
}

static DB_STATE: OnceLock<DBState> = OnceLock::new();

/// Init db connections. Requires the following env vars:
/// - `REDIS_URL`
/// - `PG_SHARD1`
/// - `PG_SHARD2`
///
/// Also queries the max user id from both shards and stores it in internal state.
///
/// - Redis can be accessed via [crate::db::redis()]
/// - Postgres can be accessed via [crate::db::postgres]
pub async fn init() {
  // init connections
  let redis_url = var("REDIS_URL").expect("REDIS_URL env var");
  let pg_shard1 = var("PG_SHARD1").expect("PG_SHARD1 env var");
  let pg_shard2 = var("PG_SHARD2").expect("PG_SHARD2 env var");

  let mut state = DBState {
    redis: Mutex::new(Redis::open(redis_url).expect("Failed to connect to Redis")),
    shard1: PgPool::connect(&pg_shard1)
      .await
      .expect("Failed to connect to Postgres shard 1"),
    shard2: PgPool::connect(&pg_shard2)
      .await
      .expect("Failed to connect to Postgres shard 2"),
    max_uid: AtomicI32::new(0),
  };

  // get max uid among all shards
  let shard_1_max_uid: i32 = query("SELECT COALESCE(MAX(\"id\"), 0) FROM \"user\"")
    .fetch_one(&state.shard1)
    .await
    .map(|row| row.get(0))
    .expect("Failed to get max uid from shard 1");

  let shard_2_max_uid: i32 = query("SELECT COALESCE(MAX(\"id\"), 0) FROM \"user\"")
    .fetch_one(&state.shard2)
    .await
    .map(|row| row.get(0))
    .expect("Failed to get max uid from shard 2");

  state.max_uid = AtomicI32::new(shard_1_max_uid.max(shard_2_max_uid));

  if DB_STATE.set(state).is_err() {
    panic!("Failed to initialize the app state!");
  }
}

fn get() -> &'static DBState {
  DB_STATE.get().unwrap()
}

/// Get Redis connection
pub fn redis() -> &'static Mutex<Redis> {
  &get().redis
}

/// Increment max user id among all shards returning old value.
///
/// This functions is intentionally the only way to aquire max user id.
///
/// We must increment it as soon as we aquire in order to avoid collisions.
/// Those may happen if users aquired same value but increment logic executes later
/// (e.g. after insert).
///
/// In worst case scenario (if id is aquired but not used) we will have a gap in ids which
/// is fine (better than having a collision).
pub fn fetch_add_max_uid() -> i32 {
  get().max_uid.fetch_add(1, Ordering::SeqCst)
}

/// Get Postgres shard connection based on user id parity
pub fn postgres(user_id: i32) -> &'static PgPool {
  if user_id % 2 == 0 {
    &get().shard2
  } else {
    &get().shard1
  }
}

#[instrument]
pub async fn run_migrations() -> Result {
  info!("Running migrations for shard 1");
  let db = &get().shard1;

  create_user_tables(db).await?;
  create_chat_tables(db).await?;

  info!("Running migrations for shard 2");
  let db = &get().shard2;

  create_user_tables(db).await?;
  create_chat_tables(db).await?;

  Ok(())
}
