use eframe::egui::{self, Context, RichText};
use crate::state::AppState;

pub fn sidebar(state: &mut AppState, ctx: &Context) {
  egui::SidePanel::left("left-panel").resizable(false).show(ctx, |ui| {
    ui.vertical_centered_justified(|ui| {
      if ui.button(RichText::new("➕ New Chat").size(20.0)).clicked() {
        state.chats.push("New Chat".to_string());
      }
      // let egui_icon = egui::include_image!("../../data/icon.png");
      // ui.add(egui::Image::new(egui_icon.clone()));
      // ui.end_row();

      // ui.add(doc_link_label(
      //     "Button with image",
      //     "Button::image_and_text",
      // ));
      // if ui
      //     .add(egui::Button::image_and_text(egui_icon, "Click me!"))
      //     .clicked()
      // {
      //     *boolean = !*boolean;
      // }
      // ui.end_row();
      egui::ScrollArea::vertical()
        .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::VisibleWhenNeeded)
        .show(ui, |ui| {
          for chat in state.chats.iter() {
            // TODO: show last msg on hover; use Text Layout
            let _ = ui.button(RichText::new(chat).size(16.0));
          }
        });
    });
  });
}
