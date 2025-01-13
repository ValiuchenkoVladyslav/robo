mod utils;
mod sidebar;
mod chat;
mod token_request;

use eframe::egui::{CentralPanel, Context};
use crate::state::AppState;

pub fn draw_ui(state: &mut AppState, ctx: &Context) {
  sidebar::sidebar(state, ctx);

  CentralPanel::default().show(ctx, |ui| {
    if state.openai_token.is_empty() {
      token_request::token_request(state, ctx, ui);
    } else {
      chat::chat(state, ctx, ui);
    }
  });
}
