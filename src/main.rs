mod result;
mod state;
mod args;
mod ui;

use state::AppState;
use clap::Parser;
use eframe::egui;
use ui::draw_ui;
use std::thread;

impl eframe::App for AppState {
  fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
    draw_ui(self, ctx);
  }

  fn on_exit(&mut self, _: Option<&eframe::glow::Context>) {
    self._save().expect("Unable to save app state");      
  }
}

fn main() -> eframe::Result {
  let args = args::Args::parse();
  let mut app_state = AppState::load().unwrap_or_default();

  if let Some(token) = args.token {
    app_state.openai_token = token;
  } else if app_state.openai_token.is_empty() {
    eprintln!("OpenAI API token is required on first run!");

    thread::park();
  }

  // hide console window
  #[cfg(target_os = "windows")]
  unsafe { winapi::um::wincon::FreeConsole() };

  eframe::run_native(
    "Robo AI Chat",
    eframe::NativeOptions::default(),
    Box::new(|_| Ok(Box::new(app_state))),
  )
}
