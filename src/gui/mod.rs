mod chat;
mod sidebar;
mod utils;

use crate::state::{AppState, Chat};
use eframe::{
  egui::{CentralPanel, Context, SidePanel},
  glow, Frame,
};

impl eframe::App for AppState {
  fn update(&mut self, ctx: &Context, _: &mut Frame) {
    SidePanel::left("sidebar")
      .resizable(false)
      .show_separator_line(false)
      .show(ctx, |ui| {
        sidebar::sidebar(self, ctx, ui);
      });

    CentralPanel::default().show(ctx, |ui| {
      // create a new chat if there are no chats
      {
        let mut chats = self.chats.write();

        if chats.is_empty() {
          chats.push(Chat::new(&self.models));

          self.active_chat = 0;
        }
      }

      chat::chat(self, ctx, ui);
    });
  }

  fn on_exit(&mut self, _: Option<&glow::Context>) {
    if let Err(err) = AppState::save(self) {
      eprintln!("Failed to save app state: {}", err);
    }
  }
}

pub fn run_gui(state: AppState) -> crate::result::Result {
  eframe::run_native(
    "Robo AI Chat",
    eframe::NativeOptions::default(),
    Box::new(|_| Ok(Box::new(state))),
  )?;

  Ok(())
}
