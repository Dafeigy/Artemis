<script setup lang="ts">
import { nextTick, onBeforeUnmount, onMounted, ref, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { useColorMode } from '@vueuse/core'
import { FitAddon } from '@xterm/addon-fit'
import { Terminal } from '@xterm/xterm'
import '@xterm/xterm/css/xterm.css'
import { Eraser, Keyboard } from 'lucide-vue-next'

import { isSerialPortOpen } from '@/lib/serial'
import Button from '@/components/ui/button/Button.vue'

type LineEnding = 'cr' | 'lf' | 'crlf'

const host = ref<HTMLDivElement | null>(null)
const lineEnding = ref<LineEnding>('cr')
const localEcho = ref(false)
const colorMode = useColorMode()

let terminal: Terminal | null = null
let fitAddon: FitAddon | null = null
let resizeObserver: ResizeObserver | null = null
let unlistenSerialData: UnlistenFn | null = null
let writeQueue = Promise.resolve()
let reportedWriteError = false

const terminalTheme = () => colorMode.value === 'dark'
  ? {
      background: '#171719',
      foreground: '#e7e7e9',
      cursor: '#a9e5a9',
      cursorAccent: '#171719',
      selectionBackground: '#847cd066',
      black: '#171719',
      brightBlack: '#6b6b72',
      green: '#8bd98b',
      brightGreen: '#a9e5a9',
    }
  : {
      background: '#fbfbfc',
      foreground: '#202124',
      cursor: '#247a40',
      cursorAccent: '#fbfbfc',
      selectionBackground: '#7cd07c66',
      black: '#202124',
      brightBlack: '#686a70',
      green: '#247a40',
      brightGreen: '#2e9950',
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
  if (!isSerialPortOpen.value) {
    if (!reportedWriteError) {
      terminal?.writeln('\r\n\x1b[33m[Artemis] 请先打开串口。\x1b[0m')
      reportedWriteError = true
    }
    return
  }

  reportedWriteError = false
  renderLocalEcho(data)
  const bytes = Array.from(encodeInput(data))
  writeQueue = writeQueue
    .then(() => invoke<number>('send_to_serial_port', { data: bytes }))
    .then(() => undefined)
    .catch((error) => {
      terminal?.writeln(`\r\n\x1b[31m[Artemis] 发送失败: ${String(error)}\x1b[0m`)
    })
}

const focusTerminal = () => terminal?.focus()

const clearTerminal = () => {
  terminal?.clear()
  terminal?.write('\x1b[2J\x1b[H')
  focusTerminal()
}

onMounted(async () => {
  terminal = new Terminal({
    allowProposedApi: false,
    convertEol: false,
    cursorBlink: true,
    cursorStyle: 'block',
    fontFamily: 'depature, Consolas, monospace',
    fontSize: 14,
    lineHeight: 1.2,
    scrollback: 5000,
    theme: terminalTheme(),
  })
  fitAddon = new FitAddon()
  terminal.loadAddon(fitAddon)
  terminal.open(host.value!)
  terminal.onData(sendInput)
  terminal.writeln('\x1b[2mArtemis Interactive Terminal — 打开串口后可直接输入\x1b[0m')

  await nextTick()
  fitAddon.fit()
  terminal.focus()

  resizeObserver = new ResizeObserver(() => fitAddon?.fit())
  resizeObserver.observe(host.value!)

  unlistenSerialData = await listen<number[]>('serial_data_bytes', (event) => {
    terminal?.write(new Uint8Array(event.payload))
  })
})

watch(colorMode, () => {
  if (terminal) terminal.options.theme = terminalTheme()
})

watch(isSerialPortOpen, (isOpen) => {
  if (isOpen) {
    reportedWriteError = false
    nextTick(focusTerminal)
  }
})

onBeforeUnmount(() => {
  unlistenSerialData?.()
  resizeObserver?.disconnect()
  terminal?.dispose()
})
</script>

<template>
  <section class="flex h-full min-h-0 flex-col overflow-hidden rounded-md border bg-[#fbfbfc] dark:bg-[#171719]" aria-label="交互式串口终端">
    <div class="flex min-h-11 flex-wrap items-center justify-between gap-2 border-b bg-muted/40 px-3 py-1.5">
      <div class="flex items-center gap-2 text-xs text-muted-foreground">
        <Keyboard class="size-4" aria-hidden="true" />
        <span>{{ isSerialPortOpen ? '键盘输入将直接发送到串口' : '打开串口后即可交互' }}</span>
        <span class="flex items-center gap-1.5" :class="isSerialPortOpen ? 'text-green-700 dark:text-green-300' : ''">
          <span class="size-1.5 rounded-full" :class="isSerialPortOpen ? 'bg-green-500' : 'bg-muted-foreground/50'" aria-hidden="true" />
          {{ isSerialPortOpen ? '已连接' : '未连接' }}
        </span>
      </div>

      <div class="flex items-center gap-3 text-xs">
        <label class="flex items-center gap-1.5 text-muted-foreground" for="terminal-line-ending">
          回车
          <select id="terminal-line-ending" v-model="lineEnding" class="h-7 rounded border bg-background px-2 text-foreground outline-none focus-visible:ring-2 focus-visible:ring-ring">
            <option value="cr">CR</option>
            <option value="lf">LF</option>
            <option value="crlf">CRLF</option>
          </select>
        </label>
        <label class="flex cursor-pointer items-center gap-1.5 text-muted-foreground">
          <input v-model="localEcho" type="checkbox" class="size-3.5 accent-green-700" />
          本地回显
        </label>
        <Button variant="ghost" size="sm" class="h-7 gap-1 px-2" title="清空终端" aria-label="清空终端" @click="clearTerminal">
          <Eraser class="size-3.5" aria-hidden="true" />
          清空
        </Button>
      </div>
    </div>

    <div ref="host" class="terminal-host min-h-0 flex-1 p-2" tabindex="0" @click="focusTerminal" />
  </section>
</template>

<style scoped>
.terminal-host :deep(.xterm) {
  height: 100%;
}

.terminal-host :deep(.xterm-viewport) {
  scrollbar-width: thin;
}
</style>
