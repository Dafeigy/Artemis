<script setup lang="ts">
import { computed, onBeforeUnmount, ref } from 'vue'
import { Usb } from 'lucide-vue-next'

import {
  subscribeToDeviceChanges,
  type SerialDeviceChange,
} from '@/lib/serial'

const DISPLAY_DURATION = 1500
const queue: SerialDeviceChange[] = []
const current = ref<SerialDeviceChange | null>(null)
const visible = ref(false)
let dismissTimer: number | undefined

const isConnected = computed(() => current.value?.type === 'connected')
const title = computed(() => isConnected.value ? '发现新设备' : '设备已移除')
const description = computed(() => {
  if (!current.value) return ''
  return isConnected.value
    ? `${current.value.port.name} 已接入`
    : `${current.value.port.name} 已断开`
})

const showNext = () => {
  if (current.value || queue.length === 0) return
  current.value = queue.shift() ?? null
  visible.value = true
  dismissTimer = window.setTimeout(() => {
    visible.value = false
  }, DISPLAY_DURATION)
}

const enqueue = (event: SerialDeviceChange) => {
  queue.push(event)
  showNext()
}

const handleAfterLeave = () => {
  current.value = null
  showNext()
}

const unsubscribe = subscribeToDeviceChanges(enqueue)

onBeforeUnmount(() => {
  unsubscribe()
  if (dismissTimer !== undefined) window.clearTimeout(dismissTimer)
})
</script>

<template>
  <div
    class="pointer-events-none fixed inset-0 z-40 grid place-items-center p-6"
    aria-live="polite"
    aria-atomic="true"
  >
    <Transition name="device-dialog" @after-leave="handleAfterLeave">
      <section
        v-if="visible && current"
        :key="current.id"
        role="status"
        class="device-dialog w-full max-w-72 overflow-hidden rounded-2xl border border-border/70 bg-popover/95 px-5 pb-5 pt-4 text-center text-popover-foreground shadow-2xl shadow-foreground/10 backdrop-blur-xl"
      >
        <div
          class="device-scene relative mx-auto h-24 w-56 overflow-hidden rounded-xl border bg-muted/35"
          :class="isConnected ? 'is-connected' : 'is-removed'"
          aria-hidden="true"
        >
          <div class="absolute inset-0 opacity-40 [background-image:radial-gradient(circle_at_center,var(--border)_1px,transparent_1px)] [background-size:12px_12px]" />

          <div class="device-body absolute right-5 top-1/2 z-20 flex h-14 w-20 -translate-y-1/2 items-center justify-center rounded-xl border bg-card shadow-sm">
            <Usb class="size-6 text-muted-foreground" :class="isConnected ? 'device-usb-icon text-success' : ''" />
            <span class="absolute -left-1 top-1/2 h-5 w-1 -translate-y-1/2 rounded-l bg-foreground/70" />
          </div>

          <div class="usb-cable absolute left-0 top-1/2 z-10 flex items-center">
            <span class="h-1 w-20 rounded-full bg-foreground/55" />
            <span class="h-7 w-10 rounded-l-md border border-foreground/35 bg-secondary shadow-sm" />
            <span class="relative h-5 w-5 border-y border-r border-foreground/40 bg-muted">
              <span class="absolute right-1 top-1 h-0.5 w-1.5 rounded-full bg-foreground/55" />
              <span class="absolute bottom-1 right-1 h-0.5 w-1.5 rounded-full bg-foreground/55" />
            </span>
          </div>

          <span class="connection-point absolute right-25 top-1/2 z-30 size-3 -translate-y-1/2 rounded-full bg-success shadow-[0_0_16px_var(--success)]" />
        </div>

        <div class="mt-4 flex items-center justify-center gap-2">
          <span
            class="size-2 rounded-full"
            :class="isConnected ? 'bg-success' : 'bg-muted-foreground/50'"
            aria-hidden="true"
          />
          <h2 class="text-sm font-semibold">{{ title }}</h2>
        </div>
        <p class="mt-1 font-mono text-xs text-muted-foreground">{{ description }}</p>
      </section>
    </Transition>
  </div>
</template>

<style scoped>
.device-dialog-enter-active {
  transition: opacity 180ms ease-out, transform 220ms cubic-bezier(0.16, 1, 0.3, 1);
}

.device-dialog-leave-active {
  transition: opacity 220ms ease-in, transform 220ms ease-in;
}

.device-dialog-enter-from,
.device-dialog-leave-to {
  opacity: 0;
  transform: translateY(8px) scale(0.97);
}

.is-connected .usb-cable {
  animation: plug-in 420ms cubic-bezier(0.16, 1, 0.3, 1) both;
}

.is-removed .usb-cable {
  animation: unplug 420ms cubic-bezier(0.4, 0, 0.2, 1) both;
}

.connection-point {
  opacity: 0;
}

.is-connected .connection-point {
  animation: connection-pulse 420ms 260ms ease-out both;
}

.is-connected .device-usb-icon {
  animation: icon-wake 360ms 240ms ease-out both;
}

@keyframes plug-in {
  from { transform: translate(-24px, -50%); }
  to { transform: translate(0, -50%); }
}

@keyframes unplug {
  from { transform: translate(0, -50%); }
  to { transform: translate(-28px, -50%); }
}

@keyframes connection-pulse {
  0% { opacity: 0; scale: 0.5; }
  45% { opacity: 1; scale: 1.35; }
  100% { opacity: 1; scale: 1; }
}

@keyframes icon-wake {
  from { opacity: 0.45; transform: scale(0.85); }
  to { opacity: 1; transform: scale(1); }
}

@media (prefers-reduced-motion: reduce) {
  .device-dialog-enter-active,
  .device-dialog-leave-active {
    transition-duration: 0.01ms;
  }

  .usb-cable,
  .connection-point,
  .device-usb-icon {
    animation-duration: 0.01ms !important;
    animation-delay: 0ms !important;
  }
}
</style>
