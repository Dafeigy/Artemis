<script setup>
import { ref } from 'vue'

const Loginfo = 
ref(`
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
  <div class="p-4 overflow-auto h-full" id="log-container">
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
  scrollbar-color: #c5c5c5 transparent;
}



*.dark::selection {
    background: #847CD0; /* 粉红色的底色 */
    color: #ebebeb; /* 文字的颜色 */
}
::selection {
    background: #7cd07c; /* 粉红色的底色 */
    color: #ebebeb; /* 文字的颜色 */
}

#log-container::-webkit-scrollbar {
  height: 6px;
  margin: 0 12px;
}
#log-container::-webkit-scrollbar-track {
  background-color: #2a2b2d;
  border-radius: 3px;
}

#log-container::-webkit-scrollbar-thumb {
  background: #555;
  border-radius: 3px;
  transition: background-color 0.2s ease;
}

#log-container::-webkit-scrollbar-thumb:hover {
  background: #777;
}

#log-container {  
  scrollbar-color: transparent transparent;
  transition: scrollbar-color 0.2s ease;
}
#log-container:hover {  
  scrollbar-color: #d3d3d345 transparent;
}
</style>