<script setup lang="ts">
import { computed, defineAsyncComponent, onBeforeUnmount, onMounted, ref } from 'vue'
import { Cable, Github, List, LoaderCircle, Power, SquareTerminal } from 'lucide-vue-next'

import appLogo from '@/assets/Artemis.png'
import AppSidebar, { type AppSection } from '@/components/AppSidebar.vue'
import Logger from '@/components/Logger.vue'
import SettingsPanel from '@/components/SettingsPanel.vue'
import WindowControls from '@/components/WindowControls.vue'
import Button from '@/components/ui/button/Button.vue'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { SidebarInset, SidebarProvider, SidebarTrigger } from '@/components/ui/sidebar'
import { Tooltip, TooltipContent, TooltipTrigger } from '@/components/ui/tooltip'
import {
  disposeSerialWorkspace,
  initializeSerialWorkspace,
  openPortNames,
  selectedPortName,
  togglePort,
} from '@/lib/serial'

const TerminalView = defineAsyncComponent(() => import('@/components/TerminalView.vue'))
type WorkspaceMode = 'logger' | 'terminal'
type ReturnableSection = Exclude<AppSection, 'settings'>

const section = ref<AppSection>('connection')
const sectionBeforeSettings = ref<ReturnableSection>('connection')
const aboutDialogOpen = ref(false)
const workspaceMode = ref<WorkspaceMode>('terminal')
const togglingConnection = ref(false)
const isSelectedPortOpen = computed(() => openPortNames.has(selectedPortName.value))

const toggleWorkspaceMode = () => {
  workspaceMode.value = workspaceMode.value === 'terminal' ? 'logger' : 'terminal'
}

const selectSection = (nextSection: AppSection) => {
  const currentSection = section.value
  if (nextSection === 'settings' && currentSection !== 'settings') {
    sectionBeforeSettings.value = currentSection
  }
  section.value = nextSection
}

const leaveSettings = () => {
  section.value = sectionBeforeSettings.value
}

const toggleSelectedConnection = async () => {
  if (!selectedPortName.value) return
  togglingConnection.value = true
  try {
    await togglePort(selectedPortName.value)
  } catch (error) {
    console.error(`Failed to toggle ${selectedPortName.value}:`, error)
  } finally {
    togglingConnection.value = false
  }
}

onMounted(initializeSerialWorkspace)
onBeforeUnmount(disposeSerialWorkspace)
</script>

<template>
  <SidebarProvider
    class="h-full min-h-0 overflow-hidden bg-transparent"
    :default-open="true"
    :style="{
      '--sidebar-width': '16rem',
      '--header-height': '3rem',
    }"
  >
    <AppSidebar
      :section="section"
      @select-section="selectSection"
      @show-about="aboutDialogOpen = true"
    />

    <SidebarInset class="acrylic-panel min-h-0 overflow-hidden">
      <div class="flex h-full min-h-0 flex-col">
        <header
          data-tauri-drag-region
          class="acrylic-header window-drag-region flex h-(--header-height) shrink-0 items-center gap-3 border-b border-border/50 pl-3 transition-colors md:pl-4"
        >
          <SidebarTrigger class="-ml-1 shrink-0" />
          <div class="h-4 w-px shrink-0 bg-border" aria-hidden="true" />
          <div v-if="section === 'settings'" data-tauri-drag-region class="min-w-0 select-none">
            <p class="truncate text-sm font-medium">
              设置
            </p>
          </div>

          <div v-if="section === 'connection' && selectedPortName" class="workspace-controls flex shrink-0 items-center gap-1.5">
            <Tooltip>
              <TooltipTrigger as-child>
                <Button
                  variant="ghost"
                  size="icon-sm"
                  :disabled="togglingConnection"
                  :aria-label="isSelectedPortOpen ? `关闭 ${selectedPortName}` : `打开 ${selectedPortName}`"
                  @click="toggleSelectedConnection"
                >
                  <LoaderCircle v-if="togglingConnection" class="size-4 animate-spin motion-reduce:animate-none" aria-hidden="true" />
                  <Power v-else class="size-4" :class="isSelectedPortOpen ? 'text-success' : ''" aria-hidden="true" />
                </Button>
              </TooltipTrigger>
              <TooltipContent>
                {{ isSelectedPortOpen ? `关闭 ${selectedPortName}` : `打开 ${selectedPortName}` }}
              </TooltipContent>
            </Tooltip>
            <span class="flex items-center gap-1.5 whitespace-nowrap text-xs text-muted-foreground" :class="isSelectedPortOpen ? 'text-success' : ''">
              <span class="size-1.5 rounded-full" :class="isSelectedPortOpen ? 'bg-success' : 'bg-muted-foreground/40'" aria-hidden="true" />
              {{ isSelectedPortOpen ? '已连接' : '未连接' }}
            </span>
            <Tooltip>
              <TooltipTrigger as-child>
                <Button
                  variant="ghost"
                  size="icon-sm"
                  :aria-label="workspaceMode === 'terminal' ? '切换到日志视图' : '切换到终端视图'"
                  @click="toggleWorkspaceMode"
                >
                  <List v-if="workspaceMode === 'terminal'" class="size-4" aria-hidden="true" />
                  <SquareTerminal v-else class="size-4" aria-hidden="true" />
                </Button>
              </TooltipTrigger>
              <TooltipContent>
                {{ workspaceMode === 'terminal' ? '切换到日志视图' : '切换到终端视图' }}
              </TooltipContent>
            </Tooltip>
          </div>
          <div data-tauri-drag-region class="h-full min-w-8 flex-1" aria-hidden="true" />
          <WindowControls class="shrink-0" />
        </header>

        <main class="min-h-0 flex-1 overflow-hidden">
          <template v-if="section === 'connection'">
            <div v-if="selectedPortName" class="relative h-full min-h-0 overflow-hidden">
              <div
                class="absolute inset-0 transition-opacity duration-200 ease-out motion-reduce:transition-none"
                :class="workspaceMode === 'logger' ? 'opacity-100' : 'pointer-events-none opacity-0'"
                :aria-hidden="workspaceMode !== 'logger'"
                :inert="workspaceMode !== 'logger'"
              >
                <Logger :port-name="selectedPortName" class="h-full" />
              </div>
              <div
                class="absolute inset-0 transition-opacity duration-200 ease-out motion-reduce:transition-none"
                :class="workspaceMode === 'terminal' ? 'opacity-100' : 'pointer-events-none opacity-0'"
                :aria-hidden="workspaceMode !== 'terminal'"
                :inert="workspaceMode !== 'terminal'"
              >
                <TerminalView :port-name="selectedPortName" class="h-full" />
              </div>
            </div>

            <div v-else class="grid h-full place-items-center p-8 text-center">
              <div class="max-w-sm">
                <div class="mx-auto mb-4 flex size-12 items-center justify-center rounded-2xl border bg-muted/50">
                  <Cable class="size-5 text-muted-foreground" aria-hidden="true" />
                </div>
                <h1 class="text-base font-medium">等待串口设备</h1>
                <p class="mt-2 text-sm leading-6 text-muted-foreground">连接设备后，Artemis 会自动扫描并将它显示在左侧连接列表中。</p>
              </div>
            </div>
          </template>

          <SettingsPanel v-else class="h-full overflow-auto" @back="leaveSettings" />
        </main>
      </div>
    </SidebarInset>

    <Dialog v-model:open="aboutDialogOpen">
      <DialogContent class="sm:max-w-md">
        <DialogHeader>
          <div class="flex items-center gap-3 pr-8">
            <img
              :src="appLogo"
              alt="Artemis Logo"
              class="size-12 shrink-0 rounded-xl object-cover ring-1 ring-border"
            />
            <div class="min-w-0 text-left leading-tight">
              <DialogTitle class="truncate text-2xl">Artemis</DialogTitle>
              <p class="mt-1 truncate font-mono text-xs tracking-wide text-muted-foreground">Serial workspace</p>
            </div>
          </div>
          <DialogDescription class="pt-2 text-left leading-6">
            简单、好看且实用的串口调试小工具。
          </DialogDescription>
        </DialogHeader>
        <a
          class="inline-flex h-9 w-fit cursor-pointer items-center gap-2 rounded-md border px-3 text-sm transition-colors hover:bg-accent focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
          href="https://github.com/Dafeigy/artemis"
          target="_blank"
          rel="noreferrer"
        >
          <Github class="size-4" aria-hidden="true" />
          项目主页
        </a>
      </DialogContent>
    </Dialog>
  </SidebarProvider>
</template>

<style scoped>
.window-drag-region,
.window-drag-region [data-tauri-drag-region] {
  -webkit-app-region: drag;
}

.workspace-controls,
.workspace-controls * {
  -webkit-app-region: no-drag;
}
</style>
