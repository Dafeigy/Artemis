<script setup>
import { ref } from 'vue'


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