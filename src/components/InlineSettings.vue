<script setup lang="ts">
import { Button } from "@/components/ui/button";
import { Settings, RotateCcw, Palette, CircleUserRound, Info, Link, FolderInput, Download } from "lucide-vue-next";
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
</script>

<template>
  <Dialog >
    <form >
      <DialogTrigger as-child>
        <img src="https://pic1.imgdb.cn/item/692661363203f7be00338e80.gif" alt="" class="rounded-full h-full aspect-square"></img>
      </DialogTrigger>
      <DialogContent class="sm:max-w-[425px] font-display">
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
            <Label for="username-1"><CircleUserRound />头像URL</Label>
            <div id="avatar-controls" class="flex items-center justify-center">
              <Input id="username-1" name="username" placeholder="http(s)://开头或base64 编码" class="w-5/6"/>
              <Button variant="ghost"class="dark:text-white text-gray-500 italic mx-2" @click=""><RotateCcw /></Button>
            </div>
            <div class="grid gap-3 hidden">
              <Label for="name-1"><Download />日志存储</Label>
              <div id="avatar-controls" class="flex items-center justify-center">
              <Input id="username-1" name="username" placeholder="日志保存目录" class="w-5/6"/>
              <Button variant="ghost"class="dark:text-white text-gray-500 italic mx-2" @click=""><FolderInput /></Button>
            </div>
          </div>
           
          </div>
        </div>
        <DialogFooter>
          <DialogClose as-child>
            <Button variant="outline"> 取消 </Button>
          </DialogClose>
          <Button type="submit"> 保存更改 </Button>
        </DialogFooter>
      </DialogContent>
    </form>
  </Dialog>
</template>
