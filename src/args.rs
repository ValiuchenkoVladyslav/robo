/// Robo: ChatGPT client written purely in Rust
#[derive(clap::Parser, Debug)]
#[command(about, long_about = None)]
pub struct Args {
  /// Custom Ollama API URL. Default is http://localhost:11434
  #[arg(short, long)]
  pub ollama_url: Option<String>,
}
