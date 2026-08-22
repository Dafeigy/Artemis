<script setup lang="ts">
import { computed, nextTick, ref, watch } from 'vue'
import { ListRestart } from 'lucide-vue-next'

import ControlsBottom from '@/components/ControlsBottom.vue'
import { clearPortLogs, portLogs } from '@/lib/serial'

const props = defineProps<{ portName: string }>()
const logContainer = ref<HTMLDivElement | null>(null)
const logs = computed(() => portLogs[props.portName] ?? [])

const entryClass = (message: string) => {
  if (/\[w\]|\[warning\]/i.test(message)) return 'text-amber-700 dark:text-amber-300'
  if (/\[e\]|\[error\]/i.test(message)) return 'text-red-700 dark:text-red-300'
  if (/\[i\]|\[info\]/i.test(message)) return 'text-sky-700 dark:text-sky-300'
  return ''
}

watch(
  () => logs.value.length,
  () => nextTick(() => {
    if (logContainer.value) logContainer.value.scrollTop = logContainer.value.scrollHeight
  }),
)
</script>

<template>
  <section class="flex h-full min-h-0 flex-col overflow-hidden bg-[#fbfbfc] dark:bg-[#151618]" :aria-label="`${portName} 串口日志视图`">
    <div id="log-container" ref="logContainer" class="custom-selection min-h-0 flex-1 overflow-auto p-4 font-mono text-sm leading-6">
      <pre v-for="(entry, index) in logs" :key="`${index}-${entry.length}`" class="whitespace-pre-wrap break-words" :class="entryClass(entry)">{{ entry }}</pre>
      <div v-if="logs.length === 0" class="flex h-full flex-col items-center justify-center gap-3 text-center text-muted-foreground">
        <ListRestart class="size-6 opacity-50" aria-hidden="true" />
        <p class="text-xs">{{ portName }} 暂无日志</p>
      </div>
    </div>
    <footer class="min-h-8 shrink-0 border-t border-border/50 bg-transparent px-2 py-0.5" aria-label="日志控制">
      <ControlsBottom class="min-w-0" @clear="clearPortLogs(portName)" />
    </footer>
  </section>
</template>

<style scoped>
section { position: relative; }
footer {
  position: absolute;
  right: 0.5rem;
  bottom: 0.25rem;
  left: 0.5rem;
  z-index: 10;
  min-height: 0;
  padding: 0;
  border: 0;
  background: transparent;
  pointer-events: none;
}
footer > * { pointer-events: auto; }
#log-container { scrollbar-color: transparent transparent; transition: scrollbar-color 0.2s ease; }
#log-container:hover { scrollbar-color: color-mix(in oklab, currentColor 25%, transparent) transparent; }
#log-container::-webkit-scrollbar { width: 6px; height: 6px; }
#log-container::-webkit-scrollbar-track { background: transparent; }
#log-container::-webkit-scrollbar-thumb { background: transparent; border-radius: 3px; }
#log-container:hover::-webkit-scrollbar-thumb { background: color-mix(in oklab, currentColor 25%, transparent); }
</style>
