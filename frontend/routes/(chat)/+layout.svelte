<script lang="ts">
  import { user, chats, activeChat } from "$lib/state";
  import { Plus } from "lucide-svelte";
  import type { Chat } from "~/types/gen";
  import { newChat } from "./api";
  import { API_URL } from "$lib/api";

	let { children } = $props();

  $effect(() => {
    fetch(API_URL + "/chats", {
      headers: {
        Authorization: `Bearer ${localStorage.getItem("JWT_TOKEN")}`,
      },
    })
    .then((res) => res.json())
    .then(chats.set);
  });
</script>

{#snippet chatLink(chat: Chat)}
  <!-- line clamp not working with a button -->
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_interactive_supports_focus -->
  <div
    role="button"
    aria-current={(chat.id === $activeChat?.id) && "page"}
    onclick={() => activeChat.set(chat)}
    class="aria-[current=page]:bg-white/10 hover:bg-white/10 mr-2 px-2 py-1 rounded-md line-clamp-3 mb-2 text-left hover:cursor-pointer"
  >
    {chat.title}
  </div>
{/snippet}

<div class="flex w-screen h-screen overflow-hidden">
  <aside class="w-[max(18vw,240px)] flex flex-col gap-3 pl-4 pt-4 bg-[#0f0f0f]">
    <header class="flex gap-3 items-center pr-4">      
      <h3 class="flex-1 text-lg text-nowrap text-ellipsis overflow-hidden">
        {$user?.name}
      </h3>

      <button
        onclick={newChat}
        class="w-[36px] h-[36px] flex items-center justify-center rounded-full bg-white/10"
      >
        <Plus />
      </button>
    </header>

    <div
      class="overflow-y-hidden hover:overflow-y-auto scrollbar-thin flex flex-col"
      style="scrollbar-gutter: stable;"
    >
      {#each $chats as chat}
        {@render chatLink(chat)}
      {/each}
    </div>
  </aside>

  <main class="flex-1 h-screen overflow-y-scroll bg-[rgb(8,8,8)]">
    {#if $activeChat}
      {@render children()}
    {:else}
      <div class="flex items-center justify-center h-full">
        <p>Select a chat to start chatting</p>
      </div>
    {/if}
  </main>
</div>
