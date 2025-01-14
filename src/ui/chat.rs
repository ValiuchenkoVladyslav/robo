use ollama_rs::{error::OllamaError, generation::chat::{request::ChatMessageRequest, ChatMessage}, Ollama};
use eframe::egui::*;
use std::sync::{Arc, Mutex};
use crate::{state::{AppState, Chat}, ui::utils::{set_input_rounding, DARKER}};

async fn ask_ai(
  mut ollama: Ollama,
  chats: Arc<Mutex<Vec<Chat>>>,
  chat_idx: usize,
) -> core::result::Result<(), OllamaError> {
  // we clone the chat to avoid holding the lock while sending the message
  let mut active_chat = chats.lock().unwrap()[chat_idx].clone();

  ollama
    .send_chat_messages_with_history(
      &mut active_chat.messages,
      ChatMessageRequest::new(
        active_chat.model,
        vec![ChatMessage::user(active_chat.saved_input)],
      ),
    )
    .await?;

  chats.lock().unwrap()[chat_idx].messages = active_chat.messages;

  Ok(())
}

pub fn chat(state: &mut AppState, ctx: &Context, ui: &mut Ui) {
  let active_chat = &mut state.chats.lock().unwrap()[state.active_chat];

  ScrollArea::vertical()
    .scroll_bar_visibility(scroll_area::ScrollBarVisibility::VisibleWhenNeeded)
    .show(ui, |ui| {
      for message in active_chat.messages[1..].iter() {
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
          let button = Button::new("Send").ui(ui);

          if button.hovered() {
            ctx.set_cursor_icon(CursorIcon::PointingHand);
          }

          if button.clicked() {
            active_chat.messages.push(ChatMessage::user(active_chat.saved_input.clone()));
            active_chat.saved_input = "".to_string();

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
            .selected_text(&active_chat.model)
            .show_ui(ui, |ui| {
              for model in state.models.lock().unwrap().iter() {
                ui.selectable_value(
                  &mut active_chat.model,
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

            TextEdit::multiline(&mut active_chat.saved_input)
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
