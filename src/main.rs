#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod result;
mod state;
mod args;
mod ui;

use state::AppState;
use clap::Parser;
use eframe::egui;

impl eframe::App for AppState {
  fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
    ui::draw_ui(self, ctx);
  }

  fn on_exit(&mut self, _: Option<&eframe::glow::Context>) {
    self._save().expect("Unable to save app state");      
  }
}

fn main() -> result::Result {
  let args = args::Args::parse();
  let mut app_state = AppState::load().unwrap_or_default();

  if let Some(token) = args.token {
    app_state.openai_token = token;
  } else if app_state.openai_token.is_empty() {
    #[cfg(not(target_os = "windows"))]
    eprintln!("OpenAI API token is required on first run!");

    #[cfg(target_os = "windows")]
    std::process::Command::new("cmd")
      .args(["/C", "echo OpenAI API token is required on first run!", "&&", "pause"])
      .spawn()?;

    return Ok(());
  }

  eframe::run_native(
    "Robo AI Chat",
    eframe::NativeOptions::default(),
    Box::new(|_| Ok(Box::new(app_state))),
  )?;

  Ok(())
}
