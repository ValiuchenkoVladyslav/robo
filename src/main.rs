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
  // create runtime for async tasks
  let rt = tokio::runtime::Runtime::new()?;
  let _enter = rt.enter();

  // parse args and load app state
  let args = args::Args::parse();
  let app_state = AppState::load(args.ollama_url)?;

  eframe::run_native(
    "Robo AI Chat",
    eframe::NativeOptions::default(),
    Box::new(|_| Ok(Box::new(app_state))),
  )?;

  Ok(())
}
