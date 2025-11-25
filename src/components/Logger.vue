<script setup>
import { ref } from 'vue'

const Loginfo = ref(`
  import type { AvatarFallbackProps } from "reka-ui"
  import type { HTMLAttributes } from "vue"
  import { reactiveOmit } from "@vueuse/core"
  import { AvatarFallback } from "reka-ui"
  import { cn } from "@/lib/utils"

  const props = defineProps<AvatarFallbackProps & { class?: HTMLAttributes["class"] }>()

  const delegatedProps = reactiveOmit(props, "class")
  <template>
    <AvatarFallback
      data-slot="avatar-fallback"
      v-bind="delegatedProps"
      :class="cn('bg-muted flex size-full items-center justify-center rounded-full', props.class)"
    >
      <slot />
    </AvatarFallback>
  </template>
  import type { AvatarFallbackProps } from "reka-ui"
  import type { HTMLAttributes } from "vue"
  import { reactiveOmit } from "@vueuse/core"
  import { AvatarFallback } from "reka-ui"
  import { cn } from "@/lib/utils"

  const props = defineProps<AvatarFallbackProps & { class?: HTMLAttributes["class"] }>()

  const delegatedProps = reactiveOmit(props, "class")
  <template>
    <AvatarFallback
      data-slot="avatar-fallback"
      v-bind="delegatedProps"
      :class="cn('bg-muted flex size-full items-center justify-center rounded-full', props.class)"
    >
      <slot />
    </AvatarFallback>
  </template>
  `)
</script>
<template>
  <div class=" p-4 overflow-auto dark:bg-[#291b1e]/80 dark:text-[#e4c8c1] bg-[#f9f0f3]/20">
    <h1 class="text-2xl font-bold">Loginfo</h1>
    <pre>
      {{ Loginfo }}
    </pre>
  </div>
</template>

<style scoped>
pre {  
  scrollbar-color: transparent transparent;
  transition: scrollbar-color 0.2s ease;
}
pre:hover {  
  scrollbar-color: #333 transparent;
}
</style>