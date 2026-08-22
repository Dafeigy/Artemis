<script setup>
import { onMounted, onUnmounted } from 'vue'
import { Eraser, Info, Rows3, Save, Settings } from 'lucide-vue-next'

import Button from './ui/button/Button.vue'
import { Tooltip, TooltipContent, TooltipTrigger } from '@/components/ui/tooltip'
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog'
import { exportLogs, notificationService } from '@/lib/utils'
import { isSelected, selectedEnd, selectedStart, totalLines } from '@/lib/utils'

const emit = defineEmits(['clear'])

const updateTotalLines = () => {
  const logContainer = document.getElementById('log-container')
  if (logContainer) totalLines.value = logContainer.querySelectorAll('pre').length
}

const handleSelection = () => {
  const logContainer = document.getElementById('log-container')
  const selection = window.getSelection()
  if (!logContainer || !selection || selection.isCollapsed) {
    isSelected.value = false
    return
  }

  const range = selection.getRangeAt(0)
  let startNode = range.startContainer
  let endNode = range.endContainer

  while (startNode && startNode.tagName !== 'PRE') startNode = startNode.parentNode
  while (endNode && endNode.tagName !== 'PRE') endNode = endNode.parentNode

  if (startNode && endNode) {
    const entries = Array.from(logContainer.querySelectorAll('pre'))
    selectedStart.value = entries.indexOf(startNode) + 1
    selectedEnd.value = entries.indexOf(endNode) + 1
    isSelected.value = true
  }
}

const clearSerialInfo = () => {
  emit('clear')
  updateTotalLines()
}

const handleExportLogs = async () => {
  const success = await exportLogs()
  if (success) {
    notificationService.success('导出成功', '日志已保存到指定文件')
  } else {
    notificationService.error('导出失败', '日志为空、导出失败或操作已取消')
  }
}

const mutationObserver = new MutationObserver(updateTotalLines)

onMounted(() => {
  updateTotalLines()
  const logContainer = document.getElementById('log-container')
  if (logContainer) mutationObserver.observe(logContainer, { childList: true, subtree: true })
  document.addEventListener('selectionchange', handleSelection)
})

onUnmounted(() => {
  mutationObserver.disconnect()
  document.removeEventListener('selectionchange', handleSelection)
})
</script>

<template>
  <div class="flex min-w-0 items-center justify-between gap-2 text-[11px] text-muted-foreground">
    <div class="flex items-center gap-1">
      <Tooltip>
        <Dialog>
          <TooltipTrigger as-child>
            <DialogTrigger as-child>
              <Button variant="ghost" size="icon-sm" class="size-6 text-muted-foreground" aria-label="关于 Artemis">
                <Info class="size-3" aria-hidden="true" />
              </Button>
            </DialogTrigger>
          </TooltipTrigger>
          <DialogContent class="font-display sm:max-w-[425px]">
            <DialogHeader>
              <DialogTitle class="flex items-center gap-2 text-xl">
                <Settings class="size-5" aria-hidden="true" />
                关于 Artemis
              </DialogTitle>
              <DialogDescription>用于串口日志监视和交互式终端操作。</DialogDescription>
            </DialogHeader>
            <DialogFooter>
              <DialogClose as-child>
                <Button variant="outline">确定</Button>
              </DialogClose>
            </DialogFooter>
          </DialogContent>
        </Dialog>
        <TooltipContent>关于 Artemis</TooltipContent>
      </Tooltip>

      <Tooltip>
        <TooltipTrigger as-child>
          <Button variant="ghost" size="icon-sm" class="size-6 text-muted-foreground" aria-label="导出日志" @click="handleExportLogs">
            <Save class="size-3" aria-hidden="true" />
          </Button>
        </TooltipTrigger>
        <TooltipContent>导出日志</TooltipContent>
      </Tooltip>
    </div>

    <div class="flex items-center gap-2 text-muted-foreground">
      <Tooltip>
        <TooltipTrigger as-child>
          <span class="flex items-center gap-1.5 whitespace-nowrap">
            <Rows3 class="size-3" aria-hidden="true" />
            {{ totalLines }} 行
            <template v-if="isSelected"> · 已选 {{ selectedStart }}–{{ selectedEnd }}</template>
          </span>
        </TooltipTrigger>
        <TooltipContent>日志行数</TooltipContent>
      </Tooltip>
      <Tooltip>
        <TooltipTrigger as-child>
          <Button variant="ghost" size="sm" class="h-6 gap-1 px-1.5 text-[11px] text-muted-foreground" aria-label="清空日志" @click="clearSerialInfo">
            <Eraser class="size-3" aria-hidden="true" />
            清空
          </Button>
        </TooltipTrigger>
        <TooltipContent>清空日志</TooltipContent>
      </Tooltip>
    </div>
  </div>
</template>
