import { ref } from 'vue'

// 串口连接状态由 Header 更新，终端和其他视图共享使用。
export const isSerialPortOpen = ref(false)
