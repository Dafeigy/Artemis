<script setup lang="ts">
import Notification from "./components/Notification.vue";
</script>

<template>
  <div class="h-screen w-screen overflow-hidden">
    <main class="h-full min-h-0 overflow-hidden">
      <RouterView />
    </main>
    <Notification />
  </div>
</template>
