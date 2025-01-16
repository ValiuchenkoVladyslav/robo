use crate::{
  gui::utils::{set_input_rounding, DARKER},
  ollama::ask_ai_sync,
  state::AppState,
};
use eframe::egui::*;
use ollama_rs::generation::chat::ChatMessage;

pub fn chat(state: &mut AppState, ctx: &Context, ui: &mut Ui) {
  let curr_chat = &mut state.chats.write()[state.active_chat];

  ScrollArea::vertical()
    .scroll_bar_visibility(scroll_area::ScrollBarVisibility::VisibleWhenNeeded)
    .show(ui, |ui| {
      ui.set_width(ui.max_rect().width());

      ui.vertical_centered(|ui| {
        ui.set_max_width(ui.max_rect().width().min(640.));

        ui.vertical(|ui| {
          for message in curr_chat.messages[1..].iter() {
            Frame::none()
              .rounding(16.)
              .fill(DARKER)
              .inner_margin(vec2(16., 8.))
              .show(ui, |ui| {
                ui.label(RichText::new(&message.content).size(16.));
              });

            ui.add_space(24.);
          }
        });
      });

      ui.add_space(126.);
    });

  TopBottomPanel::bottom("chat-input-panel")
    .show_separator_line(false)
    .frame(Frame::default().inner_margin(Margin {
      bottom: 16.,
      left: 16.,
      right: 16.,
      top: 0.,
    }))
    .show(ctx, |ui| {
      ui.vertical_centered_justified(|ui| {
        ui.set_width(ui.max_rect().width().min(640.));

        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
          ui.set_height(24.);

          // send message button
          let button = ui.button("Send ⬆");

          if button.hovered() {
            ctx.set_cursor_icon(CursorIcon::PointingHand);
          }

          if button.clicked() {
            curr_chat
              .messages
              .push(ChatMessage::user(curr_chat.saved_input.clone()));
            curr_chat.saved_input = "".to_string();

            ask_ai_sync(state.ollama, &state.chats, state.active_chat);
          }

          // select model
          ComboBox::new("Model", "")
            .width(64.)
            .selected_text(&curr_chat.model)
            .show_ui(ui, |ui| {
              for model in state.models.read().iter() {
                ui.selectable_value(&mut curr_chat.model, model.name.clone(), model.name.clone());
              }
            });
        });

        // message input
        ScrollArea::vertical()
          .scroll_bar_visibility(scroll_area::ScrollBarVisibility::VisibleWhenNeeded)
          .show(ui, |ui| {
            set_input_rounding(ui);

            TextEdit::multiline(&mut curr_chat.saved_input)
              .desired_rows(3)
              .min_size(vec2(0., ui.max_rect().height()))
              .font(FontId::new(18., FontFamily::Proportional))
              .background_color(DARKER)
              .hint_text("Ask the question here...")
              .ui(ui);
          });
      });
    });
}
