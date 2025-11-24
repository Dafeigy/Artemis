<script setup lang="ts">
import { useColorMode } from '@vueuse/core';
import Header from "../components/Header.vue";
import Logger from "../components/Logger.vue";
import Controls from "../components/Controls.vue";

// 使用useColorMode获取颜色模式值
const mode = useColorMode();
</script>

<template>
    <Header />
    <Logger />
    <Controls/>
</template>