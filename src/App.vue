<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/core";

interface TranslationData {
  original: string;
  translated: string;
}

const originalText = ref("等待划词...");
const translatedText = ref("...");
const showSettings = ref(false);
const apiKeyInput = ref("");
const saveStatus = ref("");

let unlisten: UnlistenFn | null = null;
const appWindow = getCurrentWindow();

onMounted(async () => {
  // 监听后端传来的原文和翻译
  unlisten = await listen<TranslationData>("selection-captured", (event) => {
    originalText.value = event.payload.original;
    translatedText.value = event.payload.translated;
  });

  // 加载已保存的 API Key
  try {
    const savedKey = await invoke<string>("load_api_key");
    if (savedKey) {
      apiKeyInput.value = savedKey;
    }
  } catch (e) {
    console.error("加载 Key 失败", e);
  }
});

onUnmounted(() => {
  if (unlisten) unlisten();
});

const handleClose = async () => {
  await appWindow.hide();
};

const handleMouseDown = async (e: MouseEvent) => {
  if ((e.target as HTMLElement).tagName === 'BUTTON' || (e.target as HTMLElement).tagName === 'INPUT') return;
  await appWindow.startDragging();
};

const saveApiKey = async () => {
  try {
    await invoke("save_api_key", { apiKey: apiKeyInput.value.trim() });
    saveStatus.value = "保存成功！";
    setTimeout(() => {
      saveStatus.value = "";
      showSettings.value = false;
    }, 1000);
  } catch (e) {
    saveStatus.value = "保存失败";
  }
};
</script>

<template>
  <div class="h-screen w-screen p-4 bg-zinc-900/95 text-zinc-100 backdrop-blur-md flex flex-col justify-between select-none overflow-hidden shadow-2xl">

    <!-- 顶部标题栏 -->
    <div class="flex-shrink-0 cursor-move flex items-center justify-between" @mousedown="handleMouseDown">
      <span class="text-xs text-zinc-400 font-semibold tracking-wide">Magno 划词翻译</span>
      <div class="flex items-center space-x-2">
        <button @click="showSettings = !showSettings" class="hover:text-emerald-400 cursor-pointer text-xs text-zinc-400">⚙️</button>
        <button @click="handleClose" class="hover:text-white cursor-pointer px-1 text-zinc-400">×</button>
      </div>
    </div>

    <!-- 中间内容区域 -->
    <div class="flex-1 my-2 overflow-y-auto pr-1 flex flex-col space-y-2">
      <!-- 设置面板 -->
      <div v-if="showSettings" class="flex flex-col space-y-2 bg-zinc-800/80 p-3 rounded border border-zinc-700">
        <span class="text-xs text-zinc-300 font-medium">配置 Google API Key：</span>
        <input
            type="password"
            v-model="apiKeyInput"
            placeholder="请输入 API Key..."
            class="bg-zinc-900 text-xs text-emerald-400 px-2 py-1.5 rounded border border-zinc-700 focus:outline-none focus:border-emerald-500"
        />
        <div class="flex justify-between items-center">
          <span class="text-[10px] text-emerald-400">{{ saveStatus }}</span>
          <button @click="saveApiKey" class="bg-emerald-600 hover:bg-emerald-500 text-white text-xs px-3 py-1 rounded cursor-pointer">
            保存
          </button>
        </div>
      </div>

      <!-- 双栏展示：原文 + 译文 -->
      <div v-else class="flex flex-col space-y-2">
        <!-- 原文区块 -->
        <div class="bg-zinc-800/40 p-2 rounded border border-zinc-700/40">
          <div class="text-[10px] text-zinc-500 mb-0.5">原文</div>
          <div class="text-xs text-zinc-300 break-words font-sans">
            {{ originalText }}
          </div>
        </div>

        <!-- 译文区块 -->
        <div class="bg-zinc-800/70 p-2 rounded border border-zinc-700/60">
          <div class="text-[10px] text-emerald-500/80 mb-0.5 font-semibold">翻译</div>
          <div class="text-xs font-medium text-emerald-400 break-words">
            {{ translatedText }}
          </div>
        </div>
      </div>
    </div>

    <!-- 底部提示 -->
    <div class="flex-shrink-0 text-[10px] text-zinc-500 text-right">
      按住 Ctrl 划词以捕获
    </div>
  </div>
</template>