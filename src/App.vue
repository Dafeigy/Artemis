<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

import Header from "./components/Header.vue";
import Logger from "./components/Logger.vue";
import ControlsBottom from "./components/ControlsBottom.vue";
import Notification from "./components/Notification.vue";
import TitleBar from "./components/TitleBar.vue";

</script>

<template>
  <div class="w-screen h-screen flex flex-col">
    <TitleBar />
    <main class="min-h-0 flex-1">
      <RouterView />
    </main>
    <Notification />
  </div>
</template>
