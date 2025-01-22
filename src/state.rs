use jsonwebtoken::{DecodingKey, EncodingKey};
use ollama::Ollama;
use std::sync::OnceLock;
use tracing::{info, instrument};

pub struct AppState {
  /// Ollama connection
  pub ollama: Ollama,

  /// JWT encoding key
  pub jwt_encode: EncodingKey,

  /// JWT decoding key
  pub jwt_decode: DecodingKey,
}

static APP_STATE: OnceLock<AppState> = OnceLock::new();

/// Initialize the app state
///
/// - Creates **Ollama** connection
/// - Creates **JWT** encoding & decoding keys
#[instrument(name = "AppState::init", skip_all)]
pub fn init(ollama_url: String, jwt_secret: String) {
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

    jwt_encode: EncodingKey::from_secret(jwt_secret.as_bytes()),
    jwt_decode: DecodingKey::from_secret(jwt_secret.as_bytes()),
  };

  info!("Ollama, Redis, Postgres connections established");

  if APP_STATE.set(state).is_err() {
    panic!("Failed to initialize the app state!");
  }
}

fn get() -> &'static AppState {
  APP_STATE
    .get()
    .expect("For some magical reason, the app state is not initialized!")
}

pub fn ollama() -> &'static Ollama {
  &get().ollama
}

pub fn jwt_encode() -> &'static EncodingKey {
  &get().jwt_encode
}

pub fn jwt_decode() -> &'static DecodingKey {
  &get().jwt_decode
}
