<script setup>
import Button from "./ui/button/Button.vue";
import Label from "./ui/label/Label.vue";
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
import { Save, Info } from "lucide-vue-next";
import { exportLogs, notificationService } from "../lib/utils";
// 生成串口信息测试数据
const genSerialInfo = () => {
  const logContainer = document.getElementById("log-container");
  if (!logContainer) return;

  // 测试数据数组
  const testData = [
    '   {"command": "READ_STATUS", "status": "CONNECTED", "device": "COM3", "baudrate": 9600}',
    '   {"command": "READ_CONFIG", "config": {"mode": "MASTER", "address": 1}}',
    '{"command": "DATA_TRANSFER", "direction": "RECEIVE", "bytes": 16}',
    '{"command": "ERROR", "code": "E01", "message": "Timeout error"}',
    '{"command": "CLOSE_PORT", "status": "SUCCESS", "device": "COM3"}',
  ];

  // 添加5个pre标签
  testData.forEach((data, index) => {
    const pre = document.createElement("pre");
    pre.textContent = data;
    pre.className = "";
    logContainer.appendChild(pre);
  });

  // 自动滚动到底部
  logContainer.scrollTop = logContainer.scrollHeight;
};

// 清空串口信息
const clearSerialInfo = () => {
  const logContainer = document.getElementById("log-container");
  if (logContainer) {
    logContainer.innerHTML = "";
  }
};

// 导出日志
const handleExportLogs = async () => {
  const success = await exportLogs();
  if (success) {
    notificationService.success("导出成功", "日志已成功导出到指定文件");
  } else {
    notificationService.error("导出失败", "日志导出失败或用户取消了操作");
  }
};
</script>
<template>
  <div class="justify-between flex w-full h-1/20 items-center max-h-[32px]">
    <div id="functions" class="flex justify-start text-xs">
      <Dialog>
        <form>
          <DialogTrigger as-child>
            <Button
              variant="ghost"
              size="xs"
              class="px-2 py-1 text-gray-500 italic"
              ><Info
            /></Button>
          </DialogTrigger>
          <DialogContent class="sm:max-w-[425px] font-display">
            <DialogHeader>
              <DialogTitle class="flex items-center text-xl"
                ><Settings class="mr-2" />关于</DialogTitle
              >

              <DialogDescription class="mx-4">
                作者很懒，只留下了这些东西：
              </DialogDescription>
            </DialogHeader>
            <div class="flex flex-col mx-4">
              <div class="flex">
                <Label for="name-1"><Palette />项目链接: </Label>
                <Button variant="link">👉这里</Button>
              </div>
            </div>
            <DialogFooter>
              <DialogClose as-child>
                <Button variant="outline"> 好哒 </Button>
              </DialogClose>
            </DialogFooter>
          </DialogContent>
        </form>
      </Dialog>
      <Button
        variant="ghost"
        size="xs"
        class="px-2 py-1 text-gray-500 italic"
        @click="handleExportLogs"
        title="导出日志"
      >
        <Save />
      </Button>
    </div>
    <div class="">
      <span class="text-xs mx-2 dark:text-white text-gray-500 select-none"
        >0-3630</span
      >
      <Button
        variant="ghost"
        size="xs"
        class="text-xs dark:text-white text-gray-500 px-4 py-2 italic"
        @click="genSerialInfo"
        >Generate</Button
      >
      <Button
        variant="ghost"
        size="xs"
        class="text-xs dark:text-white text-gray-500 px-4 py-2 italic"
        @click="clearSerialInfo"
        >Clear</Button
      >
    </div>
  </div>
</template>
