<script setup lang="ts">
import { computed, nextTick, onBeforeUnmount, onMounted, ref, watch } from 'vue'
import { useColorMode } from '@vueuse/core'
import { FitAddon } from '@xterm/addon-fit'
import { Terminal } from '@xterm/xterm'
import '@xterm/xterm/css/xterm.css'
import { Eraser } from 'lucide-vue-next'

import Button from '@/components/ui/button/Button.vue'
import { Tooltip, TooltipContent, TooltipTrigger } from '@/components/ui/tooltip'
import {
  clearTerminalBuffer,
  getTerminalBuffer,
  openPortNames,
  sendToPort,
  subscribeToTerminal,
} from '@/lib/serial'

type LineEnding = 'cr' | 'lf' | 'crlf'

const props = defineProps<{ portName: string }>()
const host = ref<HTMLDivElement | null>(null)
const lineEnding = ref<LineEnding>('cr')
const localEcho = ref(false)
const colorMode = useColorMode()
const isOpen = computed(() => openPortNames.has(props.portName))

let terminal: Terminal | null = null
let fitAddon: FitAddon | null = null
let resizeObserver: ResizeObserver | null = null
let unsubscribeTerminal: (() => void) | null = null
let writeQueue = Promise.resolve()
let reportedWriteError = false

const terminalTheme = () => colorMode.value === 'dark'
  ? {
      background: '#151618', foreground: '#e6e7e9', cursor: '#8ee6b1', cursorAccent: '#151618',
      selectionBackground: '#4ade8060', black: '#151618', brightBlack: '#70737a', green: '#72d69a', brightGreen: '#8ee6b1',
    }
  : {
      background: '#fbfbfc', foreground: '#202124', cursor: '#18794e', cursorAccent: '#fbfbfc',
      selectionBackground: '#22c55e40', black: '#202124', brightBlack: '#686a70', green: '#18794e', brightGreen: '#249f67',
    }

const encodeInput = (data: string) => {
  if (data !== '\r') return new TextEncoder().encode(data)
  if (lineEnding.value === 'lf') return new Uint8Array([0x0a])
  if (lineEnding.value === 'crlf') return new Uint8Array([0x0d, 0x0a])
  return new Uint8Array([0x0d])
}

const renderLocalEcho = (data: string) => {
  if (!terminal || !localEcho.value) return
  if (data === '\r') terminal.write('\r\n')
  else if (data === '\u007f') terminal.write('\b \b')
  else terminal.write(data)
}

const sendInput = (data: string) => {
  if (!isOpen.value) {
    if (!reportedWriteError) {
      terminal?.writeln('\r\n\x1b[33m[Artemis] 请先打开当前串口。\x1b[0m')
      reportedWriteError = true
    }
    return
  }

  reportedWriteError = false
  renderLocalEcho(data)
  const bytes = Array.from(encodeInput(data))
  writeQueue = writeQueue
    .then(() => sendToPort(props.portName, bytes))
    .then(() => undefined)
    .catch((error) => terminal?.writeln(`\r\n\x1b[31m[Artemis] 发送失败：${String(error)}\x1b[0m`))
}

const focusTerminal = () => terminal?.focus()

const clearTerminal = () => {
  clearTerminalBuffer(props.portName)
  terminal?.clear()
  terminal?.write('\x1b[2J\x1b[H')
  focusTerminal()
}

const renderPortBuffer = () => {
  if (!terminal) return
  terminal.reset()
  terminal.options.theme = terminalTheme()
  terminal.writeln(`\x1b[2mArtemis Interactive Terminal · ${props.portName}\x1b[0m`)
  const buffer = getTerminalBuffer(props.portName)
  if (buffer.length) terminal.write(buffer)
  nextTick(() => fitAddon?.fit())
}

onMounted(async () => {
  terminal = new Terminal({
    allowProposedApi: false,
    convertEol: false,
    cursorBlink: true,
    cursorStyle: 'block',
    fontFamily: 'depature, Consolas, monospace',
    fontSize: 14,
    lineHeight: 1.25,
    scrollback: 5000,
    theme: terminalTheme(),
  })
  fitAddon = new FitAddon()
  terminal.loadAddon(fitAddon)
  terminal.open(host.value!)
  terminal.onData(sendInput)
  renderPortBuffer()

  await nextTick()
  fitAddon.fit()
  terminal.focus()
  resizeObserver = new ResizeObserver(() => fitAddon?.fit())
  resizeObserver.observe(host.value!)
  unsubscribeTerminal = subscribeToTerminal((event) => {
    if (event.port_name === props.portName) terminal?.write(new Uint8Array(event.data))
  })
})

watch(colorMode, () => {
  if (terminal) terminal.options.theme = terminalTheme()
})
watch(() => props.portName, renderPortBuffer)
watch(isOpen, (open) => {
  if (open) {
    reportedWriteError = false
    nextTick(focusTerminal)
  }
})

onBeforeUnmount(() => {
  unsubscribeTerminal?.()
  resizeObserver?.disconnect()
  terminal?.dispose()
})
</script>

<template>
  <section class="flex h-full min-h-0 flex-col overflow-hidden bg-[#fbfbfc] dark:bg-[#151618]" :aria-label="`${portName} 交互式串口终端`">
    <div ref="host" class="terminal-host min-h-0 min-w-0 flex-1 overflow-hidden p-2" tabindex="0" @click="focusTerminal" />

    <footer class="flex min-h-8 shrink-0 flex-wrap items-center justify-end gap-2 border-t border-border/50 bg-transparent px-2 py-0.5 text-[11px] text-muted-foreground" aria-label="终端控制">
        <label class="flex items-center gap-1 text-muted-foreground" for="terminal-line-ending">
          回车
          <select id="terminal-line-ending" v-model="lineEnding" class="h-6 rounded border bg-transparent px-1.5 text-foreground outline-none focus-visible:ring-2 focus-visible:ring-ring">
            <option value="cr">CR</option>
            <option value="lf">LF</option>
            <option value="crlf">CRLF</option>
          </select>
        </label>
        <label class="flex cursor-pointer items-center gap-1 text-muted-foreground">
          <input v-model="localEcho" type="checkbox" class="size-3 accent-emerald-600" />
          本地回显
        </label>
        <Tooltip>
          <TooltipTrigger as-child>
            <Button variant="ghost" size="sm" class="h-6 gap-1 px-1.5 text-[11px] text-muted-foreground" aria-label="清空终端" @click="clearTerminal">
              <Eraser class="size-3" aria-hidden="true" />
              清空
            </Button>
          </TooltipTrigger>
          <TooltipContent>清空终端</TooltipContent>
        </Tooltip>
    </footer>
  </section>
</template>

<style scoped>
section { position: relative; }
footer {
  position: absolute;
  right: 0.5rem;
  bottom: 0.25rem;
  z-index: 10;
  min-height: 0;
  padding: 0;
  border: 0;
  background: transparent;
  pointer-events: none;
}
footer > * { pointer-events: auto; }
.terminal-host :deep(.xterm) { height: 100%; overflow: hidden; }
.terminal-host :deep(.xterm-viewport) {
  overflow-x: hidden !important;
  background-color: #fbfbfc !important;
  scrollbar-width: thin;
}
:global(.dark) .terminal-host :deep(.xterm-viewport) { background-color: #151618 !important; }
</style>
