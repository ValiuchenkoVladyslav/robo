use crate::result::Result;
use jsonwebtoken::{DecodingKey, EncodingKey};
use ollama::Ollama;
use parking_lot::Mutex;
use redis::Client as Redis;
use sqlx::PgPool;
use std::sync::OnceLock;
use tracing::{info, instrument};

static APP_STATE: OnceLock<AppState> = OnceLock::new();

pub struct AppState {
  /// Ollama connection
  pub ollama: Ollama,

  /// Redis connection
  pub redis: Mutex<Redis>,

  /// Postgres connection pool
  pub postgres: PgPool,

  /// JWT encoding key
  pub jwt_encode: EncodingKey,

  /// JWT decoding key
  pub jwt_decode: DecodingKey,
}

impl AppState {
  /// Initialize the app state
  ///
  /// - Creates **Ollama** **Redis** and **Postgres** connections
  /// - Creates **JWT** encoding & decoding keys
  ///
  /// Puts everything into `APP_STATE` static variable. Values can be accessed via `AppState` methods
  #[instrument(name = "AppState::init", skip_all)]
  pub async fn init(
    ollama_url: String,
    redis_url: String,
    postgres_url: String,
    jwt_secret: String,
  ) -> Result {
    let state = AppState {
      ollama: {
        let port_pos = ollama_url
          .rfind(':')
          .expect("OLLAMA_URL must contain a port!");

        let port = ollama_url[port_pos + 1..]
          .parse()
          .expect("OLLAMA_URL port must be a number!");

        Ollama::new(&ollama_url[..port_pos], port)
      },
      redis: Mutex::new(Redis::open(redis_url)?),
      postgres: PgPool::connect(&postgres_url).await?,

      jwt_encode: EncodingKey::from_secret(jwt_secret.as_bytes()),
      jwt_decode: DecodingKey::from_secret(jwt_secret.as_bytes()),
    };

    info!("Ollama, Redis, Postgres connections established");

    if APP_STATE.set(state).is_err() {
      panic!("Failed to initialize the app state!");
    }

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

  pub fn jwt_encode() -> &'static EncodingKey {
    &AppState::get().jwt_encode
  }

  pub fn jwt_decode() -> &'static DecodingKey {
    &AppState::get().jwt_decode
  }
}
