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
  const testData = ['// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/\n', 'use serialport::available_ports;\n', 'use serde::{Deserialize, Serialize};\n', '\n', '// COM端口信息结构体\n', '#[derive(Debug, Serialize, Deserialize)]\n', 'pub struct PortInfo {\n', '    pub name: String,\n', '    pub port_type: String,\n', '}\n', '\n', '#[tauri::command]\n', 'fn greet(name: &str) -> String {\n', '    format!("Hello, {}! You\'ve been greeted from Rust!", name)\n', '}\n', '\n', '/// 获取可用的COM端口列表\n', '#[tauri::command]\n', 'fn get_available_ports() -> Result<Vec<PortInfo>, String> {\n', '    match available_ports() {\n', '        Ok(ports) => {\n', '            let port_infos: Vec<PortInfo> = ports\n', '                .iter()\n', '                .map(|port| PortInfo {\n', '                    name: port.port_name.clone(),\n', '                    port_type: match &port.port_type {\n', '                        serialport::SerialPortType::UsbPort(info) => {\n', '                            if let Some(product) = &info.product {\n', '                                product.clone()\n', '                            } else {\n', '                                format!("{:?}", port.port_type)\n', '                            }\n', '                        },\n', '                        _ => format!("{:?}", port.port_type),\n', '                    },\n', '                })\n', '                .collect();\n', '            Ok(port_infos)\n', '        }\n', '        Err(e) => Err(format!("Failed to list serial ports: {}", e)),\n', '    }\n', '}\n', '\n', '#[cfg_attr(mobile, tauri::mobile_entry_point)]\n', 'pub fn run() {\n', '    tauri::Builder::default()\n', '        .plugin(tauri_plugin_opener::init())\n', '        .plugin(tauri_plugin_dialog::init())\n', '        .plugin(tauri_plugin_fs::init())\n', '        .invoke_handler(tauri::generate_handler![greet, get_available_ports])\n', '        .run(tauri::generate_context!())\n', '        .expect("error while running tauri application");\n', '}']
  

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
