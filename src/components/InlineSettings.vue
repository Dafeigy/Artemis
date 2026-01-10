<script setup lang="ts">
import { Button } from "@/components/ui/button";
import { Settings, Palette, CircleUserRound, Folder } from "lucide-vue-next";
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
    beforeAvatar.value = "data:image/jpeg;base64,/9j/4AAQSkZJRgABAQAAAQABAAD/4gIoSUNDX1BST0ZJTEUAAQEAAAIYAAAAAAIQAABtbnRyUkdCIFhZWiAAAAAAAAAAAAAAAABhY3NwAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAQAA9tYAAQAAAADTLQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAlkZXNjAAAA8AAAAHRyWFlaAAABZAAAABRnWFlaAAABeAAAABRiWFlaAAABjAAAABRyVFJDAAABoAAAAChnVFJDAAABoAAAAChiVFJDAAABoAAAACh3dHB0AAAByAAAABRjcHJ0AAAB3AAAADxtbHVjAAAAAAAAAAEAAAAMZW5VUwAAAFgAAAAcAHMAUgBHAEIAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAFhZWiAAAAAAAABvogAAOPUAAAOQWFlaIAAAAAAAAGKZAAC3hQAAGNpYWVogAAAAAAAAJKAAAA+EAAC2z3BhcmEAAAAAAAQAAAACZmYAAPKnAAANWQAAE9AAAApbAAAAAAAAAABYWVogAAAAAAAA9tYAAQAAAADTLW1sdWMAAAAAAAAAAQAAAAxlblVTAAAAIAAAABwARwBvAG8AZwBsAGUAIABJAG4AYwAuACAAMgAwADEANv/bAEMAAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAf/bAEMBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAf/AABEIAfQB9AMBIgACEQEDEQH/xAAfAAEAAgMAAwEBAQAAAAAAAAAACQoHCAsDBAYFAgH/xABAEAAABgICAQMDAwIEBAQEBwABAgMEBQYABwgRCRITFAoVIRYiMSNBFyQyUTM0QmEYJSZSU2JxgTVEY5GSsdH/xAAcAQEAAgMBAQEAAAAAAAAAAAAAAwQCBQYBBwj/xABAEQABBAIBAgQEAwQHCAIDAAABAAIDEQQhEgUxEyJBURQyYXEGI4FCkbHBFSQzQ1Kh8AcIFiVi0eHxU8I0grP/2gAMAwEAAhEDEQA/ANsMYxldfWExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYzxKHApDCcSkAvQnMc5kiFAf7gr6QAfz0H46/noev4EF4TX39BvZ9tAn9wJqzWl5cZ+jFVez2P2k69XLPO+sPQU0LXJeaTXKPQFEh4tk4QAQN+AEvRgEBL0Ah+Pvf8Cd3+z8j/CHbHten3fc/w7tvo9HX89fZf/b+Ov8A3/n0+r8Z7QruO11R0BW9Aiv4etHSi8dgcWuIaRWi+Md67gvDhV0bbs9rBtYxxnsS0RMQb8zGciZaFkCp9/bZmLk4d8CZlhSK4Bq/TRVFEf3drlblEAEOzgUes9fPPS/qR9de47j9QP41I1wdsbFAgi6IPsarvYIBJHrVhMYxhZJjGMImMYwiYxjCJjGMImMYwiYxjCJjGMImMYwiYxjCJjGMImMZ6gukEzAmK7Yvp/BwUcJEUIIf9PtlL6ew/jr1F66/v/OP/H+f+t/v7A14TRA3sH69q9vuvbxnhTMChfUX0GIb9xFCqe8mI9/2H8fwIAIAX9v4HoQEM82ekV6/w9h6gkf+toDYutfYj39CAf1qjekxjGeL1MYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMYxhExjGETGMYRMZ+tUqtNXu0V+kVpmV9PWqajIGEaGXTbEcykq4SbNkzuFRBu2R/q+t0uQwA2QTXdj6QEBCXXaFX8bHjR1VVbRzZtlTlrnZ/fLH/AH2OsFun7hMMUW6krHa61dAoyT55BQnvtk15ZeEVM1K4QdWGbbfNbN0fa0TYAAJ5EgNoEdzujVmj2A3Vqhk9QZjOij8OSaeYkRwQtL5XcSQ41QbxBBHLluiQCA6ocfcT/wDiE/8A5F//ANz+gEBABD8gIAID/wBh/IZts587Pg+Zrg0baKub9IhugfNOKUAVASiPfuAnIyTKU9PY/g4tPUAh0AB0HX3da8on0/G3niKE5JVrW710UCkG56a2trNqmf8A0gRaarsGSAb9Ab8LqyabXroAcD+wBxEkDvlmjJuvmBHp6gVe/f0P2UXx2Y0F0vSeoho9WY7nGteb5u3f7kUDdhaH4yYSmaM8U/J8q6nHbk/SpWUW/DSK11viqWxzHqq/8IFanZnM1O+0H8lbuCoCYQ6A4iHQ/aVfxJwURseBlLLs41y1mzVUeTVeUgDV6xTCqI9tYVaRZSLxqEM9UHuXdsixsmsgUWrQUfeByhnxJriQbvYOgRvZ+v270KNqI9cw2lzZWzwvbdxyxObISADxA2OVkCiQKNgnstCuL3CDaHJMUrCVUlG1cVdVJe8S7RV2aYOioCDhvTYcF0AnnCLgqqLmWcuGsKg4RcIg5cuWxI8Nwtv7P8R3jCSQbbzvNSsG127MXLeqS6Cm4tyPffTSKku01xAs37KpsVwSUM0k5GErcWACcgS4gUQyIzzK+d+boM/YuE/j4nWlNbUQy1D2hvuppNyuo2ViQLFP9ZaOM3RWZQ41xZJSAsd9QaKv2sykpC0EiDuOXsJdNuB/05nKPlggz3Xy5uU7xzolyXCyhGTTNS18jL8V8b5Jpudj7Ku4Qo60v7iywyV9czVwVA6Lh5UWvyAc5C+Yl5jx2eK8WC5x8jLIs2T77qxuqdQJEJhfLC3N6plnpmE8Aw4kJLciZgojxHBrn28U8sa2SmkFzI2hzhv5tb6rvWsJ64fjnw3tk5FMjHQYzG1tgV7XLFNqkJgSFvVqJD7DXSTMQAVFBxMx3sFMIG/IBmrqX1ZPJEZEPVxO0EqwA3qFijsrYX3IURDr9rwIldMBEexBX7SYoB1/T/HqywlpTwFeLbS7Bgkfjgx2/NsFxcDat62Gb2RKu1A9oCfIh3jljR00y+0UQQY1Fmj3+DEMUCgFcD6lO7cc9W7E1Rwz47aK0";
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
