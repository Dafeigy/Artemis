<script setup lang="ts">
import { Minimize, Maximize2, X } from "lucide-vue-next";
import { Button } from "@/components/ui/button";
import { getCurrentWindow  } from "@tauri-apps/api/window";

const appWindow = getCurrentWindow();

const minimize = async() => {
  await appWindow.minimize();
};

const maximize = async() => {
  await appWindow.toggleMaximize();
};

const close = async () => {
  await appWindow.close();
};
</script>

<template>
  <div class="title-bar w-full h-8 flex items-center justify-between bg-transparent select-none fixed">
    <!-- 左侧区域 - 用于拖动窗口 -->
    <div class="w-1/3 h-full"></div>
    
    <!-- 中间区域 - 标题 -->
    <div class="title text-center font-display text-sm w-1/3 font-light">
      Artemis
    </div>
    
    <!-- 右侧区域 - 窗口控制按钮 -->
    <div class="window-controls flex justify-end items-center h-full w-1/3">
      <Button 
        variant="ghost" 
        size="icon" 
        class=" hover:bg-secondary aspect-square rounded dark:hover:bg-sidebar-ring"
        @click="minimize"
      >
        <Minimize class="h-4 w-4" />
      </Button>
      <Button 
        variant="ghost" 
        size="icon" 
        class=" hover:bg-secondary aspect-square rounded dark:hover:bg-sidebar-ring"
        @click="maximize"
      >
        <Maximize2 class="h-4 w-4" />
      </Button>
      <Button 
        variant="ghost" 
        size="icon" 
        class=" hover:bg-red-400 aspect-square rounded dark:hover:bg-red-300"
        @click="close"
      >
        <X class="h-4 w-4" />
      </Button>
    </div>
  </div>
</template>

<style scoped>
.title-bar {
  /* 允许拖动窗口 */
  -webkit-app-region: drag;
}

.window-controls button {
  /* 禁用拖动区域，以便按钮可以响应点击 */
  -webkit-app-region: no-drag;
}
</style>