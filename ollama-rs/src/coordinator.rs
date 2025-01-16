use crate::{
  generation::{
    chat::{request::ChatMessageRequest, ChatMessage, ChatMessageResponse},
    options::GenerationOptions,
    tools::ToolGroup,
  },
  history::ChatHistory,
  Ollama,
};

pub struct Coordinator<C: ChatHistory, T: ToolGroup> {
  model: String,
  ollama: Ollama,
  options: GenerationOptions,
  history: C,
  tools: T,
  debug: bool,
}

impl<C: ChatHistory> Coordinator<C, ()> {
  pub fn new(ollama: Ollama, model: String, history: C) -> Self {
    Self {
      model,
      ollama,
      options: GenerationOptions::default(),
      history,
      tools: (),
      debug: false,
    }
  }
}

impl<C: ChatHistory, T: ToolGroup> Coordinator<C, T> {
  pub fn new_with_tools(ollama: Ollama, model: String, history: C, tools: T) -> Self {
    Self {
      model,
      ollama,
      options: GenerationOptions::default(),
      history,
      tools,
      debug: false,
    }
  }

  pub fn options(mut self, options: GenerationOptions) -> Self {
    self.options = options;
    self
  }

  pub fn debug(mut self, debug: bool) -> Self {
    self.debug = debug;
    self
  }

  pub async fn chat(
    &mut self,
    messages: Vec<ChatMessage>,
  ) -> crate::error::Result<ChatMessageResponse> {
    if self.debug {
      for m in &messages {
        eprintln!("Hit {} with:", self.model);
        eprintln!("\t{:?}: '{}'", m.role, m.content);
      }
    }

    let resp = self
      .ollama
      .send_chat_messages_with_history(
        &mut self.history,
        ChatMessageRequest::new(self.model.clone(), messages)
          .options(self.options.clone())
          .tools::<T>(),
      )
      .await?;

    if !resp.message.tool_calls.is_empty() {
      for call in resp.message.tool_calls {
        if self.debug {
          eprintln!("Tool call: {:?}", call.function);
        }

        let resp = self.tools.call(&call.function).await?;

        if self.debug {
          eprintln!("Tool response: {}", &resp);
        }

        self.history.push(ChatMessage::tool(resp))
      }

      // recurse
      Box::pin(self.chat(vec![])).await
    } else {
      if self.debug {
        eprintln!(
          "Response from {} of type {:?}: '{}'",
          resp.model, resp.message.role, resp.message.content
        );
      }

      Ok(resp)
    }
  }
}
