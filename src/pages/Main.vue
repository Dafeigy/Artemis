<script setup lang="ts">
import { defineAsyncComponent, ref } from 'vue'
import { List, SquareTerminal } from 'lucide-vue-next'
import Header from "../components/Header.vue";
import Logger from "../components/Logger.vue";
import ControlsBottom from "../components/ControlsBottom.vue";
const TerminalView = defineAsyncComponent(() => import('../components/TerminalView.vue'))

type WorkspaceMode = 'logger' | 'terminal'
const workspaceMode = ref<WorkspaceMode>('logger')
const terminalStarted = ref(false)

const setWorkspaceMode = (mode: WorkspaceMode) => {
    workspaceMode.value = mode
    if (mode === 'terminal') terminalStarted.value = true
}
</script>

<template>
    <div class="flex h-full min-h-0 w-full flex-col">
        <Header />
        <div class="flex items-center border-y bg-muted/30 px-4 py-1" role="tablist" aria-label="工作模式">
            <button
                type="button"
                role="tab"
                :aria-selected="workspaceMode === 'logger'"
                class="flex h-8 items-center gap-1.5 rounded px-3 text-xs transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
                :class="workspaceMode === 'logger' ? 'bg-background text-foreground shadow-sm' : 'text-muted-foreground hover:text-foreground'"
                @click="setWorkspaceMode('logger')"
            >
                <List class="size-3.5" aria-hidden="true" />
                日志监视器
            </button>
            <button
                type="button"
                role="tab"
                :aria-selected="workspaceMode === 'terminal'"
                class="flex h-8 items-center gap-1.5 rounded px-3 text-xs transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
                :class="workspaceMode === 'terminal' ? 'bg-background text-foreground shadow-sm' : 'text-muted-foreground hover:text-foreground'"
                @click="setWorkspaceMode('terminal')"
            >
                <SquareTerminal class="size-3.5" aria-hidden="true" />
                交互终端
            </button>
        </div>

        <div class="min-h-0 flex-1 p-2">
            <Logger v-show="workspaceMode === 'logger'" class="h-full" />
            <TerminalView v-if="terminalStarted" v-show="workspaceMode === 'terminal'" />
        </div>
        <ControlsBottom v-if="workspaceMode === 'logger'" />
    </div>
</template>
