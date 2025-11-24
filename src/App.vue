<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

import Header from "./components/Header.vue";
import Logger from "./components/Logger.vue";
import ControlsBottom from "./components/ControlsBottom.vue";

const greetMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet", { name: name.value });
}
</script>

<template>
  <div class="w-screen h-screen flex flex-col">
    <!-- <Title /> -->
    <RouterView />
    
  </div>
</template>