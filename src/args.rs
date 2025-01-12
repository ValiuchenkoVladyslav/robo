/// Robo: ChatGPT client written purely in Rust
#[derive(clap::Parser, Debug)]
#[command(about, long_about = None)]
pub struct Args {
  /// OpenAi API token.
  /// Required on first run. Will be saved to internal storage.
  #[arg(short, long)]
  pub token: Option<String>,
}
