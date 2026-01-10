<script setup lang="ts">
import { Button } from "@/components/ui/button";
import { Settings, RotateCcw, Palette, CircleUserRound, File, Folder } from "lucide-vue-next";
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import SelectMode from "./SelectMode.vue";
import { ref, onMounted } from "vue";
// 使用useColorMode获取颜色模式值
import { useColorMode } from "@vueuse/core";
const mode = useColorMode();
const avatarUrl = ref("");
const fileInput = ref<HTMLInputElement | null>(null);
const beforeAvatar = ref("");
  const beforeColorMode = ref("");
// 从localStorage加载头像
onMounted(() => {
  const savedAvatar = localStorage.getItem("userAvatar");
  const currentMode = localStorage.getItem("vueuse-color-scheme");
  if (savedAvatar) {
    avatarUrl.value = savedAvatar;
  }

  
  if (savedAvatar) {
    beforeAvatar.value = savedAvatar;
  }
  else{
    beforeAvatar.value = "../src/assets/warma.jpg";
  }
  if (currentMode ) {
    beforeColorMode.value = currentMode;
  }
  else{
    beforeColorMode.value = "light";
  }

  
});

// 触发文件选择
const selectImage = () => {
  fileInput.value?.click();
};

// 处理文件选择
const handleFileChange = (event: Event) => {
  const input = event.target as HTMLInputElement;
  if (input.files && input.files[0]) {
    const file = input.files[0];
    const reader = new FileReader();
    reader.onload = (e) => {
      avatarUrl.value = e.target?.result as string;
    };
    reader.readAsDataURL(file);
  }
};

// 保存头像到localStorage
const saveAvatar = () => {
  console.log(avatarUrl.value)
  if (avatarUrl.value) {
    localStorage.setItem("userAvatar", avatarUrl.value);
  }
};

const cancelChange = ()=>{
  avatarUrl.value = beforeAvatar.value;
  localStorage.setItem("userAvatar", avatarUrl.value);
  // localStorage.setItem("vueuse-color-scheme", beforeColorMode.value);
  mode.value = beforeColorMode.value as "light" | "dark" | "auto";
}
</script>

<template>
  <Dialog >
    <form @submit.prevent="saveAvatar">
      <DialogTrigger as-child>
        <img :src="avatarUrl || '../src/assets/warma.jpg'" alt="用户头像" class="rounded-full h-full aspect-square"/>
      </DialogTrigger>
      <DialogContent class="sm:max-w-[425px] font-display bg-secondary">
        <DialogHeader>
          <DialogTitle class="flex items-center text-xl" ><Settings class="mr-2"/>设置</DialogTitle>
          
          <DialogDescription>
            不管怎么设置都不影响正常运行:)
          </DialogDescription>
        </DialogHeader>
        <div class="grid gap-4">
          <div class="grid gap-3">
            <Label for="name-1"><Palette />主题</Label>
            <SelectMode />
          </div>
          <div class="grid gap-3">
            <Label for="username-1"><CircleUserRound />头像</Label>
            <div id="avatar-controls" class="flex items-center justify-center">
              <Input 
                id="username-1" 
                name="username" 
                placeholder="点击选择图片" 
                class="w-5/6" 
                v-model="avatarUrl"
                @change="handleFileChange"
                readonly
              />
              <Button variant="ghost" class="dark:text-white text-gray-500 italic mx-2" @click="selectImage">
                <Folder />
              </Button>
            </div>
            <input 
              type="file" 
              ref="fileInput" 
              accept="image/*" 
              class="hidden" 
              @change="handleFileChange"
            />
          </div>
        </div>
        <DialogFooter>
          <DialogClose as-child>
            <Button variant="outline" @click="cancelChange"> 取消 </Button>
          </DialogClose>
          <DialogClose as-child>
            <Button type="submit" @click="saveAvatar"> 保存更改 </Button>
          </DialogClose>
        </DialogFooter>
      </DialogContent>
    </form>
  </Dialog>
</template>
