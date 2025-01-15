use ollama_rs::{error::OllamaError, generation::chat::{request::ChatMessageRequest, ChatMessage}, Ollama};
use eframe::egui::*;
use std::sync::Arc;
use crate::{state::{AppState, Chat}, ui::utils::{set_input_rounding, DARKER}};
use parking_lot::RwLock;

async fn ask_ai(
  ollama: Arc<Ollama>,
  chats: Arc<RwLock<Vec<Chat>>>,
  chat_i: usize,
) -> core::result::Result<(), OllamaError> {
  // we clone the chat to avoid holding the lock while sending the message
  let mut curr_chat = chats.read()[chat_i].clone();

  ollama
    .send_chat_messages_with_history(
      &mut curr_chat.messages,
      ChatMessageRequest::new(
        curr_chat.model,
        vec![ChatMessage::user(curr_chat.saved_input)],
      ),
    )
    .await?;

  chats.write()[chat_i].messages = curr_chat.messages;

  Ok(())
}

pub fn chat(state: &mut AppState, ctx: &Context, ui: &mut Ui) {
  let curr_chat = &mut state.chats.write()[state.active_chat];

  ScrollArea::vertical()
    .scroll_bar_visibility(scroll_area::ScrollBarVisibility::VisibleWhenNeeded)
    .show(ui, |ui| {
      for message in curr_chat.messages[1..].iter() {
        ui.label(message.content.clone());
        ui.add_space(24.);
      }

      ui.add_space(126.);
    });

  TopBottomPanel::bottom("chat-input-panel")
    .show_separator_line(false)
    .frame(Frame::default().inner_margin(Margin { bottom: 16., left: 16., right: 16., top: 0. }))
    .show(ctx, |ui| {
      ui.vertical_centered_justified(|ui| {
        ui.set_width(ui.max_rect().width().min(520.));

        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
          ui.set_height(24.);

          // send message button
          let button = ui.button("Send ⬆");

          if button.hovered() {
            ctx.set_cursor_icon(CursorIcon::PointingHand);
          }

          if button.clicked() {
            curr_chat.messages.push(ChatMessage::user(curr_chat.saved_input.clone()));
            curr_chat.saved_input = "".to_string();

            let ollama = state.ollama.clone();
            let chats = state.chats.clone();
            let active_chat = state.active_chat;
            let ctx = ctx.clone();

            tokio::spawn(async move {
              if let Err(err) = ask_ai(ollama, chats, active_chat).await {
                dbg!(err);
              }

              ctx.request_repaint();
            });
          }

          // select model
          ComboBox::new("Model", "")
            .width(64.)
            .selected_text(&curr_chat.model)
            .show_ui(ui, |ui| {
              for model in state.models.read().iter() {
                ui.selectable_value(
                  &mut curr_chat.model,
                  model.name.clone(),
                  model.name.clone(),
                );
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
