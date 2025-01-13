use eframe::egui::*;
use crate::{state::AppState, ui::utils::DARKER};

const SIDEBAR_OPTION_MIN_SIZE: Vec2 = vec2(0.0, 28.0);
const SIDEBAR_OPTION_ROUNDING: Rounding = Rounding::same(6.0);

pub fn new_chat_btn(ui: &mut Ui, state: &mut AppState) {
  let btn = Button::new(RichText::new("➕ New Chat").size(20.0).strong())
    .min_size(SIDEBAR_OPTION_MIN_SIZE)
    .rounding(SIDEBAR_OPTION_ROUNDING)
    .ui(ui);

  if btn.clicked() {
    state.active_chat = None;
  }
}

fn chats_list(ui: &mut Ui, state: &mut AppState) {
  for (i, chat) in state.chats.iter().enumerate() {
    Frame::default()
      .inner_margin(Margin::symmetric(0.0, 2.0))
      .show(ui, |ui| {
        // TODO: show last msg on hover; use Text Layout
        let widgets = &mut ui.visuals_mut().widgets;

        widgets.inactive.weak_bg_fill =
          if Some(i) == state.active_chat {
            DARKER
          } else {
            Color32::TRANSPARENT
          };
        widgets.active.weak_bg_fill = DARKER;
        widgets.hovered.weak_bg_fill = DARKER;

        let chat_btn = Button::new(RichText::new(chat).size(16.0))
          .min_size(SIDEBAR_OPTION_MIN_SIZE)
          .rounding(SIDEBAR_OPTION_ROUNDING)
          .stroke(Stroke::NONE)
          .ui(ui);

        if chat_btn.clicked() {
          state.active_chat = Some(i);
        }
      });
  }    
}

pub fn sidebar(state: &mut AppState, ctx: &Context) {
  SidePanel::left("sidebar")
    .resizable(false)
    .show_separator_line(false)
    .show(ctx, |ui| {
      ui.vertical_centered_justified(|ui| {
        // create new chat button
        Frame::default()
          .inner_margin(Margin::symmetric(0.0, 6.0))
          .show(ui, |ui| new_chat_btn(ui, state));

        // list of chats
        ScrollArea::vertical()
          .scroll_bar_visibility(scroll_area::ScrollBarVisibility::VisibleWhenNeeded)
          .show(ui, |ui| chats_list(ui, state));
      });
    });
}
