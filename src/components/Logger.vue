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
  <header class=" opacity-50 p-4 h-full overflow-auto">
    <h1 class="text-2xl font-bold">Loginfo</h1>
    <pre>
      {{ Loginfo }}
    </pre>
  </header>
</template>