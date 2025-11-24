<script setup lang="ts">
// import { ref } from "vue";
// import { invoke } from "@tauri-apps/api/core";

// const greetMsg = ref("");
// const name = ref("");

// async function greet() {
//   // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
//   greetMsg.value = await invoke("greet", { name: name.value });
// }
import { Button } from '@/components/ui/button'
</script>

<template>
  <h1 class="text-3xl font-bold underline bg-red-400">
    Hello world!
  </h1>
  <Button>Click me</Button>
</template>
