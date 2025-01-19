use crate::result::Result;
use ollama::Ollama;
use parking_lot::Mutex;
use redis::Client as Redis;
use sqlx::PgPool;
use std::sync::OnceLock;
use tracing::{info, instrument};

static APP_STATE: OnceLock<AppState> = OnceLock::new();

#[derive(Debug)]
pub struct AppState {
  /// Ollama connection
  pub ollama: Ollama,

  /// Redis connection
  pub redis: Mutex<Redis>,

  /// Postgres connection pool
  pub postgres: PgPool,
}

impl AppState {
  /// Initialize the app state
  ///
  /// - Creates a new **Ollama** connection, **Redis** connection, and **Postgres** connection pool
  ///
  /// - Puts everything into `APP_STATE` static variable
  ///
  /// Connections can be accessed via `AppState::get`
  #[instrument(name = "AppState::init", skip_all)]
  pub async fn init(ollama_url: String, redis_url: String, postgres_url: String) -> Result {
    let state = AppState {
      ollama: {
        let port_pos = ollama_url
          .rfind(':')
          .expect("OLLAMA_URL must contain a port!");

        Ollama::new(&ollama_url[..port_pos], ollama_url[port_pos + 1..].parse()?)
      },
      redis: Mutex::new(Redis::open(redis_url)?),
      postgres: PgPool::connect(&postgres_url).await?,
    };

    info!("Ollama, Redis, Postgres connections established");

    APP_STATE.set(state).unwrap();

    Ok(())
  }

  fn get() -> &'static AppState {
    APP_STATE
      .get()
      .expect("For some magical reason, the app state is not initialized!")
  }

  pub fn db() -> &'static PgPool {
    &AppState::get().postgres
  }

  pub fn ollama() -> &'static Ollama {
    &AppState::get().ollama
  }

  pub fn redis() -> &'static Mutex<Redis> {
    &AppState::get().redis
  }
}
