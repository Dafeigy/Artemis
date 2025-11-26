<script setup>
import { ref } from 'vue'

// 生成串口信息测试数据
const genSerialInfo = () => {
  const logContainer = document.getElementById('log-container')
  if (!logContainer) return
  
  // 测试数据数组
  const testData = [
    '{"command": "READ_STATUS", "status": "CONNECTED", "device": "COM3", "baudrate": 9600}',
    '{"command": "READ_CONFIG", "config": {"mode": "MASTER", "address": 1}}',
    '{"command": "DATA_TRANSFER", "direction": "RECEIVE", "bytes": 16}',
    '{"command": "ERROR", "code": "E01", "message": "Timeout error"}',
    '{"command": "CLOSE_PORT", "status": "SUCCESS", "device": "COM3"}'
  ]
  
  // 添加5个pre标签
  testData.forEach((data, index) => {
    const pre = document.createElement('pre')
    pre.textContent = data
    pre.className = 'mb-2 p-2 bg-gray-100 dark:bg-gray-800 rounded'
    logContainer.appendChild(pre)
  })
  
  // 自动滚动到底部
  logContainer.scrollTop = logContainer.scrollHeight
}

// 清空串口信息
const clearSerialInfo = () => {
  const logContainer = document.getElementById('log-container')
  if (logContainer) {
    logContainer.innerHTML = ''
  }
}
</script>
<template>
  <div class="p-4 overflow-auto h-full custom-selection" id="log-container">
    
  </div>
</template>

<style scoped>
pre {  
  scrollbar-color: transparent transparent;
  transition: scrollbar-color 0.2s ease;
}
pre:hover {  
  scrollbar-color: #c5c5c5 transparent;
}
#log-container::-webkit-scrollbar {
  height: 6px;
  margin: 0 12px;
}
#log-container::-webkit-scrollbar-track {
  background-color: #2a2b2d;
  border-radius: 3px;
}

#log-container::-webkit-scrollbar-thumb {
  background: #555;
  border-radius: 3px;
  transition: background-color 0.2s ease;
}

#log-container::-webkit-scrollbar-thumb:hover {
  background: #777;
}

#log-container {  
  scrollbar-color: transparent transparent;
  transition: scrollbar-color 0.2s ease;
}
#log-container:hover {  
  scrollbar-color: #d3d3d345 transparent;
}
</style>