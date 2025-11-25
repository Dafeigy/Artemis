<script setup>
import Button
from './ui/button/Button.vue';
import Label from './ui/label/Label.vue';

// 生成串口信息测试数据
const genSerialInfo = ()=>{
  const logContainer = document.getElementById('log-container');
  if (!logContainer) return;
  
  // 测试数据数组
  const testData = [
    '   {"command": "READ_STATUS", "status": "CONNECTED", "device": "COM3", "baudrate": 9600}',
    '   {"command": "READ_CONFIG", "config": {"mode": "MASTER", "address": 1}}',
    '{"command": "DATA_TRANSFER", "direction": "RECEIVE", "bytes": 16}',
    '{"command": "ERROR", "code": "E01", "message": "Timeout error"}',
    '{"command": "CLOSE_PORT", "status": "SUCCESS", "device": "COM3"}'
  ];
  
  // 添加5个pre标签
  testData.forEach((data, index) => {
    const pre = document.createElement('pre');
    pre.textContent = data;
    pre.className = '';
    logContainer.appendChild(pre);
  });
  
  // 自动滚动到底部
  logContainer.scrollTop = logContainer.scrollHeight;
}

// 清空串口信息
const clearSerialInfo = ()=>{
  const logContainer = document.getElementById('log-container');
  if (logContainer) {
    logContainer.innerHTML = '';
  }
}
</script>
<template>
    <div class="justify-end flex w-full h-1/20 items-center max-h-[32px]">
        <div class="">
            <span class="text-xs mx-2 dark:text-white text-gray-500 select-none">0-3630</span>
            <Button variant="ghost" size="xs" class="text-xs dark:text-white text-gray-500 px-4 py-2 italic" @click="genSerialInfo">Generate</Button>
            <Button variant="ghost" size="xs" class="text-xs dark:text-white text-gray-500 px-4 py-2 italic" @click="clearSerialInfo">Clear</Button>
        </div>
    </div>
</template>