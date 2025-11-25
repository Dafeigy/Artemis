<script setup>
import Button
from './ui/button/Button.vue';
import Label from './ui/label/Label.vue';
</script>
<template>
    <div class="justify-end flex w-full h-1/20 items-center max-h-[32px] dark:bg-[#291b1e]/80">
        <div class="">
            <span class="text-xs text-white ">0-3630</span>
            <Button variant="ghost" size="xs" class="text-xs text-white px-4 py-2 italic">Clear</Button>
        </div>
    </div>
</template>

