<script setup>
import Button from "@/components/ui/button/Button.vue";
import { useRouter } from "vue-router";
import { HomeIcon, Palette, Accessibility, Info, Link, BookMarked } from "lucide-vue-next";
import Input from "@/components/ui/input/Input.vue";
import SelectMode from "@/components/SelectMode.vue";
const router = useRouter();
const BackToHome = () => {
  router.push("/");
};
</script>
<template>
  <div class="w-full flex flex-col justify-between h-full">
    <div class="flex w-full items-center justify-start max-h-[96px]">
      <div id="topleft" class="flex items-center h-full opacity-100">
        <div class="mx-2 h-3/5 aspect-square justify-center items-center cursor-pointer rounded-full"
          @click="GoToSettings">
          <img src="https://pic1.imgdb.cn/item/692572533203f7be003184ae.png" alt="" class="rounded-full">
        </div>
      </div>
      <h1 class="text-2xl mx-2">:不管怎么样设置你开心就好:)</h1>
    </div>
    <div id="settings-content" class="w-full flex flex-col justify-start  h-7/8 px-12">
      <div class="text-3xl my-2 w-fit rounded-md flex justify-center items-center">
        <Palette />
        <span class="mx-4">偏好设置</span>
      </div>
      <div class="my-4 px-6">
        <SelectMode />
      </div>
      <div class="text-3xl my-2 w-fit px-2 rounded-md flex justify-center items-center">
        <Accessibility />
        <span class="mx-4">头像设置</span>
      </div>
      <div class="my-4 px-6 w-1/2 max-w-[600px] flex">
        <Input placeholder="Enter image url or base64 string"/>
        <Button class="mx-2">应用</Button>
        <Button class="">重置</Button>
      </div>
      <div class="text-3xl my-2 w-fit px-2 rounded-md flex justify-center items-center">
        <Info />
        <span class="mx-4">关于</span>
      </div>
      <div class="my-2 px-6 flex flex-col">
        <Label class="text-2xl my-2">作者很懒，只留下了这些东西：</Label>
        <Label class="text-md flex justify-start items-center"><Link  class="mx-5"/>项目链接-><Button variant="link">这里</Button></Label>
        <Label class="text-md flex justify-start items-center"><BookMarked  class="mx-5"/>制作心得-><Button variant="link">这里</Button></Label>
      </div>
    </div>
    <div id="settings-navigate" class=" flex justify-end">
      <Button @click="BackToHome" variant="secondary">
        <HomeIcon />
      </Button>
    </div>
    
  </div>
</template>
