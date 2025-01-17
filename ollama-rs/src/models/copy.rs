use serde::Serialize;

use crate::{error::OllamaError, Ollama};

impl Ollama {
  /// Copy a model. Creates a model with another name from an existing model.
  pub async fn copy_model(&self, source: String, destination: String) -> crate::error::Result<()> {
    let request = CopyModelRequest {
      source,
      destination,
    };

    let url = format!("{}api/copy", self.url_str());
    let serialized = serde_json::to_string(&request)?;
    let builder = self.reqwest_client.post(url);

    let res = builder.body(serialized).send().await?;

    if res.status().is_success() {
      Ok(())
    } else {
      Err(OllamaError::Other(res.text().await?))
    }
  }
}

/// A copy model request to Ollama.
#[derive(Serialize)]
struct CopyModelRequest {
  source: String,
  destination: String,
}
