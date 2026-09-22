<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";

const selectedText = ref("等待划词...");
let unlisten: UnlistenFn | null = null;
const appWindow = getCurrentWindow();

onMounted(async () => {
  unlisten = await listen<string>("selection-captured", (event) => {
    console.log("收到后端传来的划词:", event.payload);
    selectedText.value = event.payload;
  });
});

onUnmounted(() => {
  if (unlisten) {
    unlisten();
  }
});

const handleClose = async () => {
  await appWindow.hide();
};

const handleMouseDown = async (e: MouseEvent) => {
  if ((e.target as HTMLElement).tagName === 'BUTTON') return;
  await appWindow.startDragging();
};
</script>

<template>
  <div class="h-screen w-screen p-4 bg-zinc-900/95 text-zinc-100 backdrop-blur-md flex flex-col justify-between select-none box-border overflow-hidden rounded-xl border border-zinc-700/80 shadow-2xl">
    <div
        class="flex-shrink-0 cursor-move flex items-center justify-between"
        @mousedown="handleMouseDown"
    >
      <span class="text-xs text-zinc-400 font-semibold tracking-wide">Magno 划词翻译</span>
      <button @click="handleClose" class="hover:text-white cursor-pointer px-1 text-zinc-400">×</button>
    </div>
    <div class="flex-1 my-2 overflow-y-auto pr-1">
      <div class="text-sm font-medium text-emerald-400 break-words bg-zinc-800/50 p-2.5 rounded border border-zinc-700/50">
        {{ selectedText }}
      </div>
    </div>
    <div class="flex-shrink-0 text-[10px] text-zinc-500 text-right">
      按住 Ctrl 划词以捕获
    </div>
  </div>
</template>