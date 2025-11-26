<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

import {
    Select,
    SelectContent,
    SelectGroup,
    SelectItem,
    SelectLabel,
    SelectTrigger,
    SelectValue,
} from '@/components/ui/select/index.js'
import { InputGroup, InputGroupAddon, InputGroupButton, InputGroupInput } from '@/components/ui/input-group'
import InlineSettings from './InlineSettings.vue';
import Button from './ui/button/Button.vue';
import { RefreshCcw } from 'lucide-vue-next';
// 定义PortInfo接口
interface PortInfo {
    name: string;
    port_type: string;
}

const avaliableCOMSs = ref<PortInfo[]>([]);
const BaudRates = ref([9600, 115200]);
// const portTimer = ref<number | null>(null);
const selectedCOM = ref<string>('');
const selectedBaudRate = ref<number>(9600);
const isPortOpen = ref<boolean>(false);
const sendInput = ref<HTMLInputElement | null>(null);
let unlistenSerialData: (() => void) | null = null;
let unlistenSerialMessage: (() => void) | null = null;
let unlistenSerialError: (() => void) | null = null;

// 获取可用COM端口列表
const fetchAvailablePorts = async () => {
    try {
        const ports = await invoke<PortInfo[]>('get_available_ports');
        avaliableCOMSs.value = ports;
    } catch (error) {
        console.error('Failed to fetch COM ports:', error);
        // 如果获取失败，使用默认值
        avaliableCOMSs.value = [{ name: 'COM3', port_type: 'USB' }, { name: 'COM4', port_type: 'USB' }];
    }
};

// 组件挂载时获取COM端口列表并设置事件监听
onMounted(async () => {
    fetchAvailablePorts(); // 立即执行一次
    
    // 监听串口数据事件
    unlistenSerialData = await listen<string>('serial_data', (event) => {
        const data = event.payload;
        console.log('Serial data received:', data);
        addLogToContainer(data);
    });
    
    // 监听串口消息事件
    unlistenSerialMessage = await listen<string>('serial_message', (event) => {
        const message = event.payload;
        console.log('Serial message:', message);
        addLogToContainer(message);
    });
    
    // 监听串口错误事件
    unlistenSerialError = await listen<string>('serial_error', (event) => {
        const error = event.payload;
        console.error('Serial error:', error);
        addLogToContainer(`Error: ${error}`);
        isPortOpen.value = false;
    });
});

// 组件卸载时清理事件监听器
onUnmounted(() => {
    if (unlistenSerialData) unlistenSerialData();
    if (unlistenSerialMessage) unlistenSerialMessage();
    if (unlistenSerialError) unlistenSerialError();
    
    // 如果端口是打开状态，尝试关闭它
    if (isPortOpen.value) {
        closeCOM().catch(err => console.error('Failed to close port on unmount:', err));
    }
});

// // 组件卸载时清除定时器，防止内存泄漏
// onUnmounted(() => {
//     if (portTimer.value !== null) {
//         clearInterval(portTimer.value);
//     }
// });

const GoToSettings = () => {
    // router.push('/Settings');
}

const toggleOpenCloseCOM = async () => {
    if (isPortOpen.value) {
        await closeCOM();
    } else {
        await openCOM();
    }
}
// 打开串口
const openCOM = async () => {
    if (!selectedCOM.value) {
        console.error('Please select a COM port');
        return;
    }
    
    try {
        const result = await invoke<string>('open_serial_port', {
            portName: selectedCOM.value,
            baudRate: selectedBaudRate.value
        });
        console.log(result);
        isPortOpen.value = true;
    } catch (error) {
        console.error('Failed to open COM port:', error);
        addLogToContainer(`Error: ${error}`);
    }
};

// 关闭串口
const closeCOM = async () => {
    try {
        const result = await invoke<string>('close_serial_port');
        console.log(result);
        isPortOpen.value = false;
    } catch (error) {
        console.error('Failed to close COM port:', error);
        addLogToContainer(`Error: ${error}`);
    }
};

// 发送数据到串口
const sendToCOM = async () => {
    if (!sendInput.value || !isPortOpen.value) {
        console.error('Cannot send data: Either no data provided or port is not open');
        return;
    }
    
    const data = sendInput.value.value;
    if (!data) {
        console.error('Cannot send empty data');
        return;
    }
    
    try {
        const result = await invoke<string>('send_to_serial_port', {
            data
        });
        console.log('Data sent:', result);
        // 清空输入框
        sendInput.value.value = '';
    } catch (error) {
        console.error('Failed to send data:', error);
        addLogToContainer(`Error: ${error}`);
    }
};

// 添加日志到容器
const addLogToContainer = (message: string) => {
    const container = document.getElementById('log-container');
    if (container) {
        const logEntry = document.createElement('pre');
        logEntry.textContent = `${message}`;
        container.appendChild(logEntry);
        // 自动滚动到底部
        container.scrollTop = container.scrollHeight;
    }
};
</script>

<template>
    <div class="flex w-full items-center justify-between max-h-[96px]">
        <div id="topleft" class="flex items-center h-full opacity-100">
            <div class="mx-2 h-3/5 aspect-square justify-center items-center cursor-pointer rounded-full"
                @click="GoToSettings">
                <InlineSettings />
            </div>
            <div id="select-com" class="mx-2">
                <Select v-model="selectedCOM">
                    <SelectTrigger class="w-[180px]">
                                <SelectValue placeholder="Select COM" />
                            </SelectTrigger>
                    <SelectContent>
                        <SelectGroup>
                            <SelectLabel class="items-center flex">Available COM<Button variant="ghost" class="mx-1" size="sm" @click="fetchAvailablePorts">
                                <RefreshCcw />
                            </Button></SelectLabel>
                            <SelectItem :value=com.name v-for="com in avaliableCOMSs" :key="com.name">
                                {{ com.name }} ({{ com.port_type }})
                            </SelectItem>
                        </SelectGroup>
                    </SelectContent>
                </Select>
            </div>
            <div id="select-baud" class="mx-2">
                <Select v-model="selectedBaudRate">
                    <SelectTrigger class="w-[180px]">
                                <SelectValue placeholder="Baud Rate" />
                            </SelectTrigger>
                    <SelectContent>
                        <SelectGroup>
                            <SelectLabel>Available Baud</SelectLabel>
                            <SelectItem :value=baud v-for="baud in BaudRates" :key="baud">
                                {{ baud }}
                            </SelectItem>
                        </SelectGroup>
                    </SelectContent>
                </Select>
            </div>
            <Button @click="toggleOpenCloseCOM" :disabled="isPortOpen || !selectedCOM" class="mx-1">
                        Open COM
                    </Button>
                    <!-- <Button @click="closeCOM" :disabled="!isPortOpen" class="mx-1">
                        Close COM
                    </Button> -->
        </div>

        <div id="topright" class="flex justify-start mx-5">
            <InputGroup>
                    <InputGroupInput ref="sendInput" placeholder="Send to COM..." />
                    <InputGroupAddon align="inline-end">
                        <InputGroupButton 
                            variant="secondary" 
                            @click="sendToCOM()"
                            :disabled="!isPortOpen"
                        >
                            Send
                        </InputGroupButton>
                    </InputGroupAddon>
                </InputGroup>
        </div>

    </div>
</template>

<style scoped>
pre{
    font-family: var(--font-mono);
}
</style>