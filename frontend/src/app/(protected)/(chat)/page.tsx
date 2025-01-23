"use client";

import { Chat } from "~/types/gen";
import { useMessagesQuery, useSendMessageMutation } from "./_api";
import { useActiveChat } from "./_store";
import { Send } from "lucide-react";
import { useEffect, useRef } from "react";

function Messages({ chat }: { chat: Chat }) {
  const messages = useMessagesQuery(chat.id);

  return (
    <section className="flex flex-col items-center mt-16">
      <div className="w-[max(48vw,264px)] px-2">
        {messages?.data?.map((message) => (
          <article key={message.id}>
            <h1 className="text-sm pl-3">{message.role}</h1>

            <div className="bg-[#1f1f1f] rounded-xl px-3 py-2 mb-3">
              {message.text}
            </div>
          </article>
        ))}
      </div>
    </section>
  );
}

const maxRows = 6;

function ChatInput({ chat }: { chat: Chat }) {
  const sendMessage = useSendMessageMutation(chat.id);
  const textArea = useRef<HTMLTextAreaElement>(null);

  function updateInputSize() {
    const lineHeight = parseFloat(getComputedStyle(textArea.current!).lineHeight);
    const maxHeight = maxRows * lineHeight;

    textArea.current!.style.height = "auto";
    textArea.current!.style.height = `${Math.min(textArea.current!.scrollHeight, maxHeight)}px`;
  }

  useEffect(() => {
    const _textArea = textArea.current;

    _textArea?.addEventListener("input", updateInputSize);

    return () => {
      _textArea?.removeEventListener("input", updateInputSize);
    }
  }, [textArea]);

  return (
    <div>
      <textarea
        ref={textArea}
        rows={3}
        placeholder="Type something..."
        className="bg-[#1f1f1f] rounded-xl w-[max(48vw,264px)] py-2 pl-3 pr-14 overflow-y-scroll resize-none"
      ></textarea>

      <div className="relative ml-auto">
        <button
          className="absolute bg-white/90 w-[36px] h-[36px] rounded-full bottom-[16px] right-[28px] flex items-center justify-center hover:cursor-pointer"
          onClick={() => {
            sendMessage.mutate({ text: textArea.current!.value });
            textArea.current!.value = "";
          }}
        >
          <Send width={24} height={24} color="#1f1f1f" className="mr-[4px] mt-[4px]" />
        </button>
      </div>
    </div>
  );
}

export default function ChatPage() {
  const activeChat = useActiveChat().chat;

  return (
    <div>
      {activeChat && <Messages chat={activeChat} />}

      <section className="fixed bottom-0 pb-8 w-[calc(100vw-max(18vw,240px))] flex justify-center">
        <ChatInput chat={activeChat!} />
      </section>
    </div>
  );
}
