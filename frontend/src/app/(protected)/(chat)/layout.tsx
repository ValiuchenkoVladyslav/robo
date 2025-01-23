"use client";

import { Plus } from "lucide-react";
import { useChatsQuery, useNewChatMutation } from "./_api";
import { useActiveChat } from "./_store";
import { useCurrentUser } from "~/current-user";

export default function ChatsLayout({ children }: React.PropsWithChildren) {
  const chats = useChatsQuery();
  const newChat = useNewChatMutation();
  const activeChat = useActiveChat();
  const currentUser = useCurrentUser().user;

  return (
    <div className="flex w-screen h-screen overflow-hidden">
      <aside className="w-[max(18vw,240px)] flex flex-col gap-3 pl-4 pt-4 bg-[#0f0f0f]">
        <header className="flex gap-3 items-center pr-4">      
          <h3 className="flex-1 text-lg text-nowrap text-ellipsis overflow-hidden">
            {currentUser?.name}
          </h3>

          <button
            onClick={() => newChat.mutate({ title: "New Chat", model: "phi3.5" })}
            className="w-[36px] h-[36px] flex items-center justify-center rounded-full bg-white/10"
          >
            <Plus />
          </button>
        </header>

        <nav
          className="overflow-y-hidden hover:overflow-y-auto scrollbar-thin flex flex-col"
          style={{ scrollbarGutter: "stable" }}
        >
          {chats?.data?.map((chat) => (
            <div
              key={chat.id}
              role="button"
              aria-current={(chat.id === activeChat.chat?.id) && "page"}
              onClick={() => activeChat.set(chat)}
              className="aria-[current=page]:bg-white/10 hover:bg-white/10 mr-2 px-2 py-1 rounded-md line-clamp-3 mb-2 text-left hover:cursor-pointer"
            >
              {chat.title}
            </div>
          ))}
        </nav>
      </aside>

      <main className="flex-1 h-screen overflow-y-scroll bg-[rgb(8,8,8)]">
        {activeChat.chat ? children : (
          <div className="flex items-center justify-center h-full">
            <p>Select a chat to start chatting</p>
          </div>
        )}
      </main>
    </div>
  );
}
