<script setup lang="ts">
import { ref } from 'vue'

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

import Button from './ui/button/Button.vue';
import { useRouter } from 'vue-router';
const router = useRouter();
const avaliableCOMSs = ref([3, 4, 12, 14]);
const BaudRates = ref([9600, 115200]);

const GoToSettings = () => {
    router.push('/Settings');
}

</script>

<template>
    <div class="flex w-full items-center justify-between max-h-[96px]">
        <div id="topleft" class="flex items-center h-full opacity-100">
            <div class="mx-2 h-3/5 aspect-square justify-center items-center cursor-pointer rounded-full"
                @click="GoToSettings">
                <img src="https://pic1.imgdb.cn/item/692572533203f7be003184ae.png" alt="" class="rounded-full">
            </div>
            <div id="select-com" class="mx-2">
                <Select>
                    <SelectTrigger class="w-[180px]">
                        <SelectValue placeholder="Select COM" />
                    </SelectTrigger>
                    <SelectContent>
                        <SelectGroup>
                            <SelectLabel>Available COM</SelectLabel>
                            <SelectItem :value=comvalue v-for="comvalue in avaliableCOMSs">
                                COM {{ comvalue }}
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