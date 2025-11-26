<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from "@tauri-apps/api/core";

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

// 定义PortInfo接口
interface PortInfo {
    name: string;
    port_type: string;
}

const avaliableCOMSs = ref<PortInfo[]>([]);
const BaudRates = ref([9600, 115200]);

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

// 组件挂载时获取COM端口列表
onMounted(() => {
    fetchAvailablePorts();
});

// const GoToSettings = () => {
//     router.push('/Settings');
// }

const GoToSettings = () => {
    // router.push('/Settings');
}
</script>

<template>
    <div class="flex w-full items-center justify-between max-h-[96px]">
        <div id="topleft" class="flex items-center h-full opacity-100">
            <div class="mx-2 h-3/5 aspect-square justify-center items-center cursor-pointer rounded-full"
                @click="GoToSettings">
                <InlineSettings />
            </div>
            <div id="select-com" class="mx-2">
                <Select>
                    <SelectTrigger class="w-[180px]">
                        <SelectValue placeholder="Select COM" />
                    </SelectTrigger>
                    <SelectContent>
                        <SelectGroup>
                            <SelectLabel>Available COM</SelectLabel>
                            <SelectItem :value=com.name v-for="com in avaliableCOMSs" :key="com.name">
                                {{ com.name }} ({{ com.port_type }})
                            </SelectItem>
                        </SelectGroup>
                    </SelectContent>
                </Select>
            </div>
            <div id="select-baud" class="mx-2">
                <Select>
                    <SelectTrigger class="w-[180px]">
                        <SelectValue placeholder="Baud Rate" />
                    </SelectTrigger>
                    <SelectContent>
                        <SelectGroup>
                            <SelectLabel>Available COM</SelectLabel>
                            <SelectItem :value=baud v-for="baud in BaudRates">
                                {{ baud }}
                            </SelectItem>
                        </SelectGroup>
                    </SelectContent>
                </Select>
            </div>
            <Button>Open COM</Button>
        </div>

        <div id="topright" class="flex justify-start mx-5">
            <InputGroup>
                    <InputGroupInput placeholder="Send to COM..." />
                    <InputGroupAddon align="inline-end">
                        <InputGroupButton variant="secondary">
                            Send
                        </InputGroupButton>
                    </InputGroupAddon>
                </InputGroup>
        </div>

    </div>
</template>

<style scoped></style>