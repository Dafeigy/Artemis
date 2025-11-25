<script setup>
import Button from "@/components/ui/button/Button.vue";
import { useRouter } from "vue-router";
import ToggleMode from "@/components/ToggleMode.vue";

import SelectMode from "@/components/SelectMode.vue";
const router = useRouter();
const BackToHome = () => {
  router.push("/");
};

</script>
<template>
  <div class="h-full w-full flex flex-col justify-center px-4">
    <div
      id="settings-title"
      class="w-full text-xl font-bold h-1/8 flex items-center mx-4 my-4"
    >
      <h1 class=" text-3xl">User Settings</h1>
    </div>
    <div id="settings-content" class="w-full h-7/8 mx-4 my-4">
        <h2 class=" text-xl">Theme</h2>
      <div class="w-full h-1/3 flex flex-col justify-center">
          <SelectMode />
      </div>
    </div>
  </div>
  <div
    id="settings"
    class="h-full w-full flex flex-col justify-center items-center"
  >
    <div class="mb-4">
      <ToggleMode />
    </div>
    <Button @click="BackToHome">Back to Home</Button>
  </div>
</template>
