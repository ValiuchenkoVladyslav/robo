use crate::{result::Result, state::Chat};
use ollama_rs::{
  generation::chat::{request::ChatMessageRequest, ChatMessage},
  models::LocalModel,
  Ollama,
};
use parking_lot::RwLock;
use std::sync::Arc;

pub async fn ask_ai(ollama: &Ollama, chats: &Arc<RwLock<Vec<Chat>>>, chat: usize) -> Result {
  // we clone the chat to avoid holding the lock while sending the message
  let mut curr_chat = chats.read()[chat].clone();

  ollama
    .send_chat_messages_with_history(
      &mut curr_chat.messages,
      ChatMessageRequest::new(
        curr_chat.model,
        vec![ChatMessage::user(curr_chat.saved_input)],
      ),
    )
    .await?;

  chats.write()[chat]
    .messages
    .push(curr_chat.messages.last().unwrap().clone());

  Ok(())
}

pub fn ask_ai_sync(ollama: &'static Ollama, chats: &Arc<RwLock<Vec<Chat>>>, chat: usize) {
  tokio::spawn({
    let chats = chats.clone();

    async move {
      if let Err(err) = ask_ai(ollama, &chats, chat).await {
        dbg!(err);
      }
    }
  });
}

pub async fn list_models(ollama: &Ollama, models: Arc<RwLock<Vec<LocalModel>>>) -> Result {
  *models.write() = ollama.list_local_models().await?;

  Ok(())
}
