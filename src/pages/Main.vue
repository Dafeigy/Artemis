<script setup lang="ts">
import { useColorMode } from '@vueuse/core';
import Header from "../components/Header.vue";
import Logger from "../components/Logger.vue";
import ControlsBottom from "../components/ControlsBottom.vue";


</script>

<template>
    <div class="flex w-full h-full flex-col justify-center">
        <Header />
        <Logger />
        <ControlsBottom />
    </div>
</template>