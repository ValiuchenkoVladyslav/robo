import { create } from 'zustand'
import { Chat } from '~/types/gen';

type ActiveChatStore = {
  chat?: Chat;
  set: (chat: Chat) => void;
}

export const useActiveChat = create<ActiveChatStore>()((set) => ({
  chat: undefined,
  set(chat) {
    set({ chat });
  },
}));
