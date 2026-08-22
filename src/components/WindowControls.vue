<script setup lang="ts">
import { Maximize2, Minus, X } from 'lucide-vue-next'
import { getCurrentWindow } from '@tauri-apps/api/window'

import Button from '@/components/ui/button/Button.vue'
import { Tooltip, TooltipContent, TooltipTrigger } from '@/components/ui/tooltip'

const appWindow = '__TAURI_INTERNALS__' in window ? getCurrentWindow() : null

const minimize = () => appWindow?.minimize()
const maximize = () => appWindow?.toggleMaximize()
const close = () => appWindow?.close()
</script>

<template>
  <div class="window-controls flex h-full items-stretch" aria-label="窗口控制">
    <Tooltip>
      <TooltipTrigger as-child>
        <Button
          variant="ghost"
          size="icon"
          class="h-full w-11 rounded-none text-muted-foreground hover:bg-accent hover:text-foreground"
          aria-label="最小化窗口"
          @click="minimize"
        >
          <Minus class="size-4" aria-hidden="true" />
        </Button>
      </TooltipTrigger>
      <TooltipContent side="bottom">最小化</TooltipContent>
    </Tooltip>
    <Tooltip>
      <TooltipTrigger as-child>
        <Button
          variant="ghost"
          size="icon"
          class="h-full w-11 rounded-none text-muted-foreground hover:bg-accent hover:text-foreground"
          aria-label="最大化或还原窗口"
          @click="maximize"
        >
          <Maximize2 class="size-3.5" aria-hidden="true" />
        </Button>
      </TooltipTrigger>
      <TooltipContent side="bottom">最大化或还原</TooltipContent>
    </Tooltip>
    <Tooltip>
      <TooltipTrigger as-child>
        <Button
          variant="ghost"
          size="icon"
          class="h-full w-11 rounded-none rounded-tr-xl text-muted-foreground hover:bg-red-500 hover:text-white dark:hover:bg-red-600"
          aria-label="关闭窗口"
          @click="close"
        >
          <X class="size-4" aria-hidden="true" />
        </Button>
      </TooltipTrigger>
      <TooltipContent side="bottom">关闭</TooltipContent>
    </Tooltip>
  </div>
</template>

<style scoped>
.window-controls,
.window-controls * {
  -webkit-app-region: no-drag;
}
</style>
