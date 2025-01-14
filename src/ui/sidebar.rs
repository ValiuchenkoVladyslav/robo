use eframe::egui::*;
use crate::{state::{AppState, Chat}, ui::utils::DARKER};

const SIDEBAR_OPTION_MIN_SIZE: Vec2 = vec2(0., 28.);
const SIDEBAR_OPTION_ROUNDING: Rounding = Rounding::same(6.);

pub fn new_chat_btn(ui: &mut Ui, state: &mut AppState) {
  let btn = Button::new(RichText::new("➕ New Chat").size(20.).strong())
    .min_size(SIDEBAR_OPTION_MIN_SIZE)
    .rounding(SIDEBAR_OPTION_ROUNDING)
    .ui(ui);

  if btn.clicked() {
    let mut chats = state.chats.lock().unwrap();
    chats.push(Chat::new(state.models.clone()));

    state.active_chat = chats.len() - 1;

    dbg!(&chats.last().unwrap().model);
  }
}

fn chats_list(ui: &mut Ui, state: &mut AppState) {
  for (i, chat) in state.chats.lock().unwrap().iter().enumerate() {
    Frame::default()
      .inner_margin(Margin::symmetric(0., 2.))
      .show(ui, |ui| {
        // TODO: show last msg on hover; use Text Layout
        let widgets = &mut ui.visuals_mut().widgets;

        widgets.inactive.weak_bg_fill =
          if i == state.active_chat {
            DARKER
          } else {
            Color32::TRANSPARENT
          };
        widgets.active.weak_bg_fill = DARKER;
        widgets.hovered.weak_bg_fill = DARKER;

        let chat_btn = Button::new(RichText::new(&chat.title).size(16.))
          .min_size(SIDEBAR_OPTION_MIN_SIZE)
          .rounding(SIDEBAR_OPTION_ROUNDING)
          .stroke(Stroke::NONE)
          .ui(ui);

        if chat_btn.clicked() {
          state.active_chat = i;
        }
      });
  }    
}

pub fn sidebar(state: &mut AppState, _ctx: &Context, ui: &mut Ui) {
  ui.vertical_centered_justified(|ui| {
    // create new chat button
    Frame::default()
      .inner_margin(Margin::symmetric(0., 6.))
      .show(ui, |ui| new_chat_btn(ui, state));

    // list of chats
    ScrollArea::vertical()
      .scroll_bar_visibility(scroll_area::ScrollBarVisibility::VisibleWhenNeeded)
      .show(ui, |ui| chats_list(ui, state));
  });
}
