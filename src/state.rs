use redis::Client as Redis;
use ollama_rs::Ollama;
use sea_orm::{Database, DatabaseConnection};
use std::sync::OnceLock;
use crate::result::Result;

static APP_STATE: OnceLock<AppState> = OnceLock::new();

#[derive(Debug)]
pub struct AppState {
  /// Ollama connection
  pub ollama: Ollama,

  /// Redis connection
  pub redis: Redis,

  /// Postgres connection pool
  pub postgres: DatabaseConnection,
}

impl AppState {
  /// Initialize the app state
  /// 
  /// - Creates a new **Ollama** connection, **Redis** connection, and **Postgres** connection pool
  /// 
  /// - Puts everything into the `APP_STATE` static variable
  /// 
  /// Connections can be accessed via [AppState::get] function
  pub async fn new(ollama_url: String, redis_url: String, postgres_url: String) -> Result {
    let state = AppState {
      ollama: {
        let port_pos = ollama_url.rfind(':').expect("OLLAMA_URL must contain a port!");
  
        Ollama::new(
          &ollama_url[..port_pos],
          ollama_url[port_pos + 1..].parse()?,
        )
      },
      redis: Redis::open(redis_url)?,
      postgres: Database::connect(postgres_url).await?,
    };

    APP_STATE.set(state).unwrap();

    Ok(())
  }

  fn get() -> &'static AppState {
    APP_STATE.get().expect("For some magical reason, the app state is not initialized!")
  }

  pub fn db() -> &'static DatabaseConnection {
    &AppState::get().postgres
  }

  pub fn ollama() -> &'static Ollama {
    &AppState::get().ollama
  }

  pub fn redis() -> &'static Redis {
    &AppState::get().redis
  }
}
