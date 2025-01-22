import { browser } from "$app/environment";
import { writable } from "svelte/store";
import type { Chat, Message, PublicUser } from "~/types/gen";

export const user = writable<PublicUser | null>(null);

if (browser) {
  const cachedUser = localStorage.getItem("user");

  if (cachedUser) {
    user.set(JSON.parse(cachedUser));
  }
}

export function setUser(newUser: PublicUser) {
  user.set(newUser);

  localStorage.setItem("user", JSON.stringify(newUser));
}

export const chats = writable<Chat[]>([]);

export const activeChat = writable<Chat | null>(null);
export const messages = writable<Message[]>([]);
