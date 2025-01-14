mod utils;
mod sidebar;
mod chat;

use eframe::egui::{CentralPanel, Context, SidePanel};
use crate::state::{AppState, Chat};

pub fn draw_ui(state: &mut AppState, ctx: &Context) {
  SidePanel::left("sidebar")
    .resizable(false)
    .show_separator_line(false)
    .show(ctx, |ui| {
      sidebar::sidebar(state, ctx, ui);
    });

  CentralPanel::default().show(ctx, |ui| {
    // create a new chat if there are no chats
    if let Ok(mut chats) = state.chats.lock() {
      if chats.is_empty() {
        chats.push(Chat::new(state.models.clone()));

        state.active_chat = 0;
      }
    }

    chat::chat(state, ctx, ui);
  });
}
