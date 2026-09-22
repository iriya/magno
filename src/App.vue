<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";

const selectedText = ref("等待划词...");
let unlisten: UnlistenFn | null = null;

onMounted(async () => {
  // 监听 Rust 发来的划词数据
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
  await getCurrentWindow().hide();
};
</script>

<template>
  <div class="p-4 bg-zinc-900/95 text-zinc-100 rounded-xl shadow-2xl border border-zinc-700 backdrop-blur-md w-full h-full flex flex-col justify-between select-none">
    <div>
      <div class="text-xs text-zinc-400 mb-1 flex justify-between items-center">
        <span>Magno 划词镜</span>
        <button @click="handleClose" class="hover:text-white cursor-pointer">×</button>
      </div>
      <div class="text-sm font-medium text-emerald-400 break-words mt-2 bg-zinc-800/50 p-2 rounded border border-zinc-700/50">
        {{ selectedText }}
      </div>
    </div>
    <div class="text-[10px] text-zinc-500 text-right mt-2">
      按住 Ctrl 划词以捕获
    </div>
  </div>
</template>