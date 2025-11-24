<script setup>
import Button from '@/components/ui/button/Button.vue';
import { useRouter } from 'vue-router';
import ToggleMode from '@/components/ToggleMode.vue';

const router = useRouter();
const BackToHome = () => {
    router.push('/');
}


</script>
<template>
    <div id="settings" class="h-full w-full flex flex-col justify-center items-center">
        <div class="mb-4">
            <ToggleMode />
        </div>
        <Button @click="BackToHome">Back to Home</Button>
        
    </div>
</template>