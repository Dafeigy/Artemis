<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { ArrowLeft, ImagePlus, MonitorCog } from 'lucide-vue-next'

import appLogo from '@/assets/Artemis.png'
import SelectMode from '@/components/SelectMode.vue'
import Button from '@/components/ui/button/Button.vue'
import { Tooltip, TooltipContent, TooltipTrigger } from '@/components/ui/tooltip'

const emit = defineEmits<{ back: [] }>()
const avatarUrl = ref(localStorage.getItem('userAvatar') || appLogo)
const fileInput = ref<HTMLInputElement | null>(null)

const selectAvatar = () => fileInput.value?.click()

const handleAvatar = (event: Event) => {
  const input = event.target as HTMLInputElement
  const file = input.files?.[0]
  if (!file) return
  const reader = new FileReader()
  reader.onload = () => {
    avatarUrl.value = String(reader.result)
    localStorage.setItem('userAvatar', avatarUrl.value)
    window.dispatchEvent(new Event('artemis-avatar-change'))
  }
  reader.readAsDataURL(file)
}

const resetAvatar = () => {
  avatarUrl.value = appLogo
  localStorage.removeItem('userAvatar')
  window.dispatchEvent(new Event('artemis-avatar-change'))
}

onMounted(() => {
  avatarUrl.value = localStorage.getItem('userAvatar') || appLogo
})
</script>

<template>
  <section class="mx-auto flex w-full max-w-3xl flex-col gap-6 p-6 md:p-10" aria-labelledby="settings-title">
    <div class="flex items-center gap-3">
      <Tooltip>
        <TooltipTrigger as-child>
          <Button
            variant="outline"
            size="icon-lg"
            class="size-12 rounded-xl"
            aria-label="返回上一页面"
            @click="emit('back')"
          >
            <ArrowLeft class="size-5" aria-hidden="true" />
          </Button>
        </TooltipTrigger>
        <TooltipContent side="bottom">返回上一页面</TooltipContent>
      </Tooltip>
      <div class="min-w-0 leading-tight">
        <h1 id="settings-title" class="truncate text-2xl font-semibold">设置</h1>
        <p class="mt-1 truncate text-sm text-muted-foreground">调整 Artemis 的外观与身份标识。</p>
      </div>
    </div>

    <div class="grid gap-4 rounded-xl border bg-card/70 p-5 shadow-sm">
      <div class="flex items-center gap-2">
        <MonitorCog class="size-4 text-muted-foreground" aria-hidden="true" />
        <h2 class="text-sm font-medium">外观</h2>
      </div>
      <SelectMode />
    </div>

    <div class="grid gap-4 rounded-xl border bg-card/70 p-5 shadow-sm">
      <div class="flex items-center gap-2">
        <ImagePlus class="size-4 text-muted-foreground" aria-hidden="true" />
        <h2 class="text-sm font-medium">侧边栏头像</h2>
      </div>
      <div class="flex flex-wrap items-center gap-4">
        <img :src="avatarUrl" alt="当前头像预览" class="size-16 rounded-2xl object-cover ring-1 ring-border" />
        <div class="flex gap-2">
          <Button variant="outline" @click="selectAvatar">选择图片</Button>
          <Button variant="ghost" @click="resetAvatar">恢复默认</Button>
        </div>
        <input ref="fileInput" class="sr-only" type="file" accept="image/*" @change="handleAvatar" />
      </div>
    </div>
  </section>
</template>
