mod utils;
mod sidebar;
mod chat;

use eframe::egui::{CentralPanel, Context};
use crate::state::{AppState, Chat};

pub fn draw_ui(state: &mut AppState, ctx: &Context) {
  sidebar::sidebar(state, ctx);

  CentralPanel::default().show(ctx, |ui| {
    if state.active_chat.is_none() {
      let mut chats = state.chats.lock().unwrap();

      if chats.is_empty() {
        chats.push(Chat {
          title: "New Chat".to_string(),
          ..Default::default()
        });
      }

      state.active_chat = Some(0);
    }

    chat::chat(state, ctx, ui);
  });
}
