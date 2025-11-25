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
  <div class="w-full flex flex-col justify-center px-4 h-7/8">
    <div
      id="settings-title"
      class="w-full text-xl font-bold h-1/8 flex items-center mx-4 my-4"
    >
      <div
        class="mx-2 h-3/5 aspect-square justify-center items-center cursor-pointer rounded-full"
      >
        <img
          src="https://pic1.imgdb.cn/item/692572533203f7be003184ae.png"
          alt=""
          class="rounded-full"
        />
      </div>
      <h1 class="text-3xl">：不管怎么设置都会错</h1>
    </div>
    <div id="settings-content" class="w-full h-7/8 mx-4">
      <h2 class="text-xl">Theme</h2>
      <div class="w-full h-1/3 flex flex-col justify-center">
        <SelectMode />
      </div>
      <div class="">
        <ToggleMode />
      </div>
      <Button @click="BackToHome">Back to Home</Button>
    </div>
  </div>
</template>
