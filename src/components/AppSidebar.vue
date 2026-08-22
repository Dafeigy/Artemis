<script setup lang="ts">
import { onBeforeUnmount, onMounted, ref } from 'vue'
import {
  ChevronRight,
  CircleHelp,
  Cable,
  Settings,
} from 'lucide-vue-next'

import appLogo from '@/assets/Artemis.png'
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from '@/components/ui/collapsible'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import { Tooltip, TooltipContent, TooltipTrigger } from '@/components/ui/tooltip'
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarMenuSub,
  SidebarMenuSubButton,
  SidebarMenuSubItem,
  SidebarRail,
} from '@/components/ui/sidebar'
import {
  availablePorts,
  baudRates,
  openPortNames,
  portBaudRates,
  selectedPortName,
} from '@/lib/serial'

export type AppSection = 'connection' | 'settings' | 'about'

defineProps<{ section: AppSection }>()
const emit = defineEmits<{ selectSection: [section: AppSection] }>()
const avatarUrl = ref(localStorage.getItem('userAvatar') || appLogo)

const selectPort = (portName: string) => {
  selectedPortName.value = portName
  emit('selectSection', 'connection')
}

const updateAvatar = () => {
  avatarUrl.value = localStorage.getItem('userAvatar') || appLogo
}

onMounted(() => window.addEventListener('artemis-avatar-change', updateAvatar))
onBeforeUnmount(() => window.removeEventListener('artemis-avatar-change', updateAvatar))
</script>

<template>
  <Sidebar variant="inset" collapsible="icon">
    <SidebarHeader class="border-b border-sidebar-border/70">
      <SidebarMenu>
        <SidebarMenuItem>
          <SidebarMenuButton
            size="lg"
            tooltip="Artemis"
            :is-active="section === 'connection'"
            @click="emit('selectSection', 'connection')"
          >
            <img :src="avatarUrl" alt="Artemis 头像" class="size-8 shrink-0 rounded-lg object-cover ring-1 ring-sidebar-border" />
            <div class="grid flex-1 text-left leading-tight">
              <span class="truncate text-sm font-semibold tracking-wide">Artemis</span>
              <span class="truncate text-[11px] text-muted-foreground">Serial workspace</span>
            </div>
          </SidebarMenuButton>
        </SidebarMenuItem>
      </SidebarMenu>
    </SidebarHeader>

    <SidebarContent>
      <SidebarGroup>
        <SidebarGroupLabel>串口设备</SidebarGroupLabel>
        <SidebarGroupContent>
          <SidebarMenu>
            <Collapsible default-open class="group/collapsible">
              <SidebarMenuItem>
                <CollapsibleTrigger as-child>
                  <SidebarMenuButton tooltip="连接" class="h-8 text-sidebar-foreground/80">
                    <Cable class="text-muted-foreground" aria-hidden="true" />
                    <span class="font-medium">连接</span>
                    <span class="ml-auto flex h-5 min-w-5 items-center justify-center rounded-full bg-sidebar-accent/70 px-1.5 text-[10px] tabular-nums text-muted-foreground group-data-[collapsible=icon]:hidden">
                      {{ availablePorts.length }}
                    </span>
                    <ChevronRight class="text-muted-foreground transition-transform duration-200 group-data-[state=open]/collapsible:rotate-90" aria-hidden="true" />
                  </SidebarMenuButton>
                </CollapsibleTrigger>
                <CollapsibleContent>
                  <SidebarMenuSub class="relative mx-2.5 gap-0 border-sidebar-border/50 px-2 py-1">
                    <TransitionGroup name="device-list">
                    <SidebarMenuSubItem v-for="port in availablePorts" :key="port.name" class="relative py-0.5">
                      <Tooltip>
                        <TooltipTrigger as-child>
                          <SidebarMenuSubButton
                            as="button"
                            class="h-8 w-full rounded-md pr-16 text-sidebar-foreground/80 transition-colors data-[active=true]:bg-sidebar-accent/65 data-[active=true]:text-sidebar-accent-foreground data-[active=true]:ring-1 data-[active=true]:ring-sidebar-border/60"
                            :is-active="section === 'connection' && selectedPortName === port.name"
                            @click="selectPort(port.name)"
                          >
                            <span
                              class="size-1.5 shrink-0 rounded-full"
                              :class="openPortNames.has(port.name) ? 'bg-success shadow-[0_0_8px_color-mix(in_oklab,var(--success)_60%,transparent)]' : 'bg-muted-foreground/35'"
                              aria-hidden="true"
                            />
                            <span class="font-mono text-xs font-medium tracking-wide">{{ port.name }}</span>
                          </SidebarMenuSubButton>
                        </TooltipTrigger>
                        <TooltipContent side="right">{{ port.port_type }}</TooltipContent>
                      </Tooltip>

                      <Tooltip>
                        <TooltipTrigger as-child>
                          <span
                            class="absolute right-1.5 top-1/2 z-10 -translate-y-1/2 rounded focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-sidebar-ring"
                            :tabindex="openPortNames.has(port.name) ? 0 : -1"
                            :aria-label="openPortNames.has(port.name) ? `${port.name} 已连接，无法修改波特率` : undefined"
                          >
                            <DropdownMenu>
                              <DropdownMenuTrigger as-child>
                                <button
                                  type="button"
                                  class="h-5 rounded bg-sidebar-accent/65 px-1.5 font-mono text-[9px] tabular-nums text-muted-foreground transition-colors hover:bg-sidebar-accent hover:text-sidebar-accent-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-sidebar-ring disabled:opacity-60"
                                  :aria-label="`切换 ${port.name} 波特率，当前 ${portBaudRates[port.name]}`"
                                  :disabled="openPortNames.has(port.name)"
                                  @click.stop
                                >
                                  {{ portBaudRates[port.name] || 115200 }}
                                </button>
                              </DropdownMenuTrigger>
                              <DropdownMenuContent side="right" align="start" class="min-w-30">
                                <DropdownMenuItem
                                  v-for="baud in baudRates"
                                  :key="baud"
                                  class="justify-between font-mono text-xs"
                                  @select="portBaudRates[port.name] = baud"
                                >
                                  {{ baud }}
                                  <span v-if="portBaudRates[port.name] === baud" class="text-success">●</span>
                              </DropdownMenuItem>
                              </DropdownMenuContent>
                            </DropdownMenu>
                          </span>
                        </TooltipTrigger>
                        <TooltipContent side="right">
                          {{ openPortNames.has(port.name) ? '连接时不可修改波特率' : '点击切换波特率' }}
                        </TooltipContent>
                      </Tooltip>
                    </SidebarMenuSubItem>

                    <li v-if="availablePorts.length === 0" key="empty-device-list" class="px-2 py-3 text-xs leading-relaxed text-muted-foreground">
                      未扫描到串口设备
                    </li>
                    </TransitionGroup>
                  </SidebarMenuSub>
                </CollapsibleContent>
              </SidebarMenuItem>
            </Collapsible>
          </SidebarMenu>
        </SidebarGroupContent>
      </SidebarGroup>
    </SidebarContent>

    <SidebarFooter class="border-t border-sidebar-border/70">
      <SidebarMenu>
        <SidebarMenuItem>
          <SidebarMenuButton tooltip="设置" :is-active="section === 'settings'" @click="emit('selectSection', 'settings')">
            <Settings aria-hidden="true" />
            <span>设置</span>
          </SidebarMenuButton>
        </SidebarMenuItem>
        <SidebarMenuItem>
          <SidebarMenuButton tooltip="关于" :is-active="section === 'about'" @click="emit('selectSection', 'about')">
            <CircleHelp aria-hidden="true" />
            <span>关于</span>
          </SidebarMenuButton>
        </SidebarMenuItem>
      </SidebarMenu>
    </SidebarFooter>
    <SidebarRail />
  </Sidebar>
</template>

<style scoped>
.device-list-enter-active,
.device-list-leave-active,
.device-list-move {
  transition:
    opacity 180ms ease,
    transform 180ms ease;
}

.device-list-enter-from,
.device-list-leave-to {
  opacity: 0;
  transform: translateY(-4px);
}

@media (prefers-reduced-motion: reduce) {
  .device-list-enter-active,
  .device-list-leave-active,
  .device-list-move {
    transition-duration: 0.01ms;
  }
}
</style>
