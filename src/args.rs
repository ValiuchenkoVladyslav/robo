/// Robo: Ollama client written purely in Rust
#[derive(clap::Parser, Debug)]
#[command(about, long_about = None)]
pub struct Args {
  /// Custom Ollama API URL. Default is http://localhost:11434
  #[arg(short, long, default_value = "http://localhost:11434")]
  pub ollama_url: String,

  /// Custom Redis URL. Default is redis://127.0.0.1/
  #[arg(short, long, default_value = "redis://127.0.0.1/")]
  pub redis_url: String,

  /// API port to listen on
  #[arg(short, long, default_value = "3000")]
  pub port: u16,
}
