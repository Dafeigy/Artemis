<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

import Header from "./components/Header.vue";
import Loginfo from "./components/Loginfo.vue";
import Settings from "./components/Settings.vue";
import Sidebar from "./components/Sidebar.vue";
import Controls from "./components/Controls.vue";
import Title from "./components/Title.vue";
import Bottom from "./components/Bottom.vue";
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
    <Header />
    <div class="flex w-full h-9/10">
      <div class="w-full h-full flex flex-col">
        <Loginfo />
      </div>
    </div>
    <Bottom />
  </div>
</template>


