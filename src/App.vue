<script setup lang="ts">
import DeviceChangeOverlay from "./components/DeviceChangeOverlay.vue";
import Notification from "./components/Notification.vue";
</script>

<template>
  <div class="app-acrylic h-screen w-screen overflow-hidden">
    <main class="h-full min-h-0 overflow-hidden">
      <RouterView />
    </main>
    <DeviceChangeOverlay />
    <Notification />
  </div>
</template>
