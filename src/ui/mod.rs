mod sidebar;

use eframe::egui::{self, Context};
use crate::state::AppState;

pub fn draw_ui(state: &mut AppState, ctx: &Context) {
  sidebar::sidebar(state, ctx);

  egui::CentralPanel::default().show(ctx, |ui| {
    ui.heading("Central Panel");

    ui.label(&state.openai_token);
    // ui.horizontal(|ui| {
    //   let name_label = ui.label("Your name: ");

    //   ui.text_edit_singleline(&mut self.name)
    //     .labelled_by(name_label.id);
    // });

    // ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
  });
}
