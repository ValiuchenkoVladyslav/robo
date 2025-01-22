<script lang="ts">
  import { Send } from "lucide-svelte";
  import { messages, activeChat } from "$lib/state";
  import { getMessages, sendMessage } from "./api";

  const maxRows = 6;
  let textArea = $state<HTMLTextAreaElement>();

  function updateInputSize() {
    const lineHeight = parseFloat(getComputedStyle(textArea!).lineHeight);
    const maxHeight = maxRows * lineHeight;

    textArea!.style.height = "auto";
    textArea!.style.height = `${Math.min(textArea!.scrollHeight, maxHeight)}px`;
  }

  $effect(() => {
    textArea?.addEventListener("input", updateInputSize);

    return () => {
      textArea?.removeEventListener("input", updateInputSize);
    };
  });

  $effect(() => {
    if ($activeChat?.id) getMessages($activeChat.id);
  });
</script>

<div>
  <section class="flex flex-col items-center mt-16">
    <div class="w-[max(48vw,264px)] px-2">
      {#each $messages as message}
        <article>
          <h1 class="text-sm pl-3">{message.role}</h1>

          <div class="bg-[#1f1f1f] rounded-xl px-3 py-2 mb-3">
            {message.text}
          </div>
        </article>
      {/each}
    </div>
  </section>

  <section class="fixed bottom-0 pb-8 w-[calc(100vw-max(18vw,240px))] flex justify-center">
    <div>
      <textarea
        bind:this={textArea}
        rows="3"
        placeholder="Type something..."
        class="bg-[#1f1f1f] rounded-xl w-[max(48vw,264px)] py-2 pl-3 pr-14 overflow-y-scroll resize-none"
      ></textarea>

      <div class="relative ml-auto">
        <button
          class="absolute bg-white/90 w-[36px] h-[36px] rounded-full bottom-[16px] right-[28px] flex items-center justify-center hover:cursor-pointer"
          onclick={() => {
            sendMessage($activeChat!.id, textArea!.value);
            textArea!.value = "";
          }}
        >
          <Send width={24} height={24} color="#1f1f1f" class="mr-[4px] mt-[4px]" />
        </button>
      </div>
    </div>
  </section>
</div>
