<script setup>
import { ref, onMounted, onUnmounted } from 'vue';
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
import { Save, Info, Settings, Palette } from "lucide-vue-next";
import { exportLogs, notificationService } from "../lib/utils";

// 行数状态
const totalLines = ref(0);
const selectedStart = ref(0);
const selectedEnd = ref(0);
const isSelected = ref(false);

// 更新总行数
const updateTotalLines = () => {
  const logContainer = document.getElementById("log-container");
  if (logContainer) {
    totalLines.value = logContainer.querySelectorAll('pre').length;
  }
};

// 处理选择事件
const handleSelection = () => {
  const logContainer = document.getElementById("log-container");
  if (!logContainer) return;

  const selection = window.getSelection();
  if (!selection || selection.isCollapsed) {
    isSelected.value = false;
    return;
  }

  const range = selection.getRangeAt(0);
  const startNode = range.startContainer;
  const endNode = range.endContainer;

  // 获取包含起始节点的pre标签
  let startPre = startNode;
  while (startPre && startPre.tagName !== 'PRE') {
    startPre = startPre.parentNode;
  }

  // 获取包含结束节点的pre标签
  let endPre = endNode;
  while (endPre && endPre.tagName !== 'PRE') {
    endPre = endPre.parentNode;
  }

  if (startPre && endPre) {
    const allPres = Array.from(logContainer.querySelectorAll('pre'));
    selectedStart.value = allPres.indexOf(startPre) + 1;
    selectedEnd.value = allPres.indexOf(endPre) + 1;
    isSelected.value = true;
  }
};

// 清空串口信息
const clearSerialInfo = () => {
  const logContainer = document.getElementById("log-container");
  if (logContainer) {
    logContainer.innerHTML = "";
  }
  updateTotalLines();
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

// 监听日志添加事件
const mutationObserver = new MutationObserver(() => {
  updateTotalLines();
});

onMounted(() => {
  // 初始更新总行数
  updateTotalLines();
  
  // 监听日志容器的变化
  const logContainer = document.getElementById("log-container");
  if (logContainer) {
    mutationObserver.observe(logContainer, {
      childList: true,
      subtree: true
    });
  }
  
  // 监听选择事件
  document.addEventListener('selectionchange', handleSelection);
});

onUnmounted(() => {
  mutationObserver.disconnect();
  document.removeEventListener('selectionchange', handleSelection);
});
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
              class="px-2 py-1  italic"
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
        class="px-2 py-1 italic"
        @click="handleExportLogs"
        title="导出日志"
      >
        <Save />
      </Button>
    </div>
    <div class="">
      <span class="text-xs mx-2 dark:text-white text-gray-500 select-none"
        >0-{{ totalLines }}<template v-if="isSelected">({{ selectedStart }}:{{ selectedEnd }})</template></span
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
