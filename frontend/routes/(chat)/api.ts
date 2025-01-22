import { chats, messages } from "$lib/state";
import { API_URL } from "$lib/api";
import type { CreateChatRequest, Message, SendMessageRequest } from "~/types/gen";

export function newChat() {
  const title = prompt("Enter chat title");
  const model = prompt("Enter chat model");

  if (!title || !model) return;

  fetch(API_URL + "/chats", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${localStorage.getItem("JWT_TOKEN")}`,
    },
    body: JSON.stringify({ title, model } satisfies CreateChatRequest),
  })
  .then((res) => res.json())
  .then((chat) => chats.update((prev) => [...prev, chat]));
}

export function getMessages(chatId: number) {
  fetch(API_URL + `/chats/${chatId}`, {
    headers: {
      Authorization: `Bearer ${localStorage.getItem("JWT_TOKEN")}`,
    },
  })
  .then((res) => res.json())
  .then(messages.set);
}

export function sendMessage(chatId: number, text: string) {
  messages.update((prev) => [
    ...prev,
    {
      id: prev.length + 1,
      role: "User",
      text,
      chat_id: chatId
    } satisfies Message
  ]);

  fetch(API_URL + `/chats/${chatId}`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${localStorage.getItem("JWT_TOKEN")}`,
    },
    body: JSON.stringify({ text } satisfies SendMessageRequest),
  })
  .then((res) => res.json())
  .then((message) => messages.update((prev) => [...prev, message]));
}
