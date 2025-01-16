/// Robo: Ollama client written purely in Rust
#[derive(clap::Parser, Debug)]
#[command(about, long_about = None)]
pub struct Args {
  /// Custom Ollama API URL. Default is http://localhost:11434
  #[arg(short, long)]
  pub ollama_url: Option<String>,

  /// API only mode
  #[cfg(all(feature = "api", feature = "gui"))]
  #[arg(short, long, default_value = "false")]
  pub api_mode: bool,

  /// API port. Only applicable in API mode
  #[cfg(feature = "api")]
  #[arg(short, long, default_value = "3000")]
  pub port: u16,
}
