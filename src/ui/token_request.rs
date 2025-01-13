//! OpenAi API token request if not provided on first run.

use eframe::egui::*;
use crate::{state::AppState, ui::utils::{DARKER, set_input_rounding}};

pub fn token_request(state: &mut AppState, _ctx: &Context, ui: &mut Ui) {
  ui.vertical_centered_justified(|ui| {
    ui.set_width(ui.max_rect().width().min(520.0));

    set_input_rounding(ui);

    let input_data_id = ui.make_persistent_id("token-input");
    let mut token = ui.data_mut(|d| d.get_persisted(input_data_id).unwrap_or_default());

    let input = TextEdit::singleline(&mut token)
      .hint_text("OpenAi API Token")
      .font(FontId::new(20.0, FontFamily::Proportional))
      .background_color(DARKER)
      .margin(Margin::same(8.0))
      .ui(ui);

    if input.lost_focus() {
      state.openai_token = token;
    } else if input.changed() {
      ui.data_mut(|d| d.insert_persisted(input_data_id, token));
    }

    ui.heading("OpenAi API Token is required on first run");
  });
}
