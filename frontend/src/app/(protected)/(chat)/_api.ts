import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { Chat, CreateChatRequest, Message, SendMessageRequest } from "~/types/gen";
import { API_URL, withAuth } from "~/utils";

export function useChatsQuery() {
  return useQuery<Chat[]>({
    queryKey: ["chats"],

    async queryFn() {
      const res = await fetch(API_URL + "/chats", {
        headers: withAuth(),
      });

      return res.json();
    },
  });
}

export function useNewChatMutation() {
  const queryClient = useQueryClient();

  return useMutation<Chat, unknown, CreateChatRequest>({
    async mutationFn({ title, model }) {
      const res = await fetch(API_URL + "/chats", {
        method: "POST",
        headers: withAuth({
          "Content-Type": "application/json",
        }),
        body: JSON.stringify({ title, model }),
      });

      return res.json();
    },

    onSuccess(chat) {
      queryClient.setQueryData<Chat[]>(
        ["chats"],
        (prev) => [...(prev ?? []), chat],
      );
    }
  });
}

export function useMessagesQuery(chatId: number) {
  return useQuery<Message[]>({
    queryKey: ["messages", chatId],

    async queryFn() {
      const res = await fetch(API_URL + "/chats/" + chatId, {
        headers: withAuth(),
      });

      return res.json();
    },
  });
}

export function useSendMessageMutation(chatId: number) {
  const queryClient = useQueryClient();

  return useMutation<Message, unknown, SendMessageRequest>({
    async mutationFn(msgReq) {
      const res = await fetch(API_URL + "/chats/" + chatId, {
        method: "POST",
        headers: withAuth({
          "Content-Type": "application/json",
        }),
        body: JSON.stringify(msgReq),
      });

      return res.json();
    },

    onSuccess(message) {
      queryClient.setQueryData<Message[]>(
        ["messages", chatId],
        (prev) => [...(prev ?? []), message],
      );
    },

    onMutate(msgReq) {
      // optimistic update
      queryClient.setQueryData<Message[]>(
        ["messages", chatId],
        (prev) => [
          ...(prev ?? []),
          {
            id: (prev?.length ?? 0) + 999,
            role: "User",
            text: msgReq.text,
            chat_id: chatId,
          },
        ],
      );
    }
  });
}
