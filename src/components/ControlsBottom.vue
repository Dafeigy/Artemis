<script setup>
import Button
from './ui/button/Button.vue';
import Label from './ui/label/Label.vue';
</script>
<template>
    <div class="justify-end flex w-full h-1/20 items-center max-h-[32px]">
        <div class="">
            <span class="text-xs mx-2 dark:text-white text-gray-500 select-none">0-3630</span>
            <Button variant="ghost" size="xs" class="text-xs dark:text-white text-gray-500 px-4 py-2 italic">Clear</Button>
        </div>
    </div>
</template>

