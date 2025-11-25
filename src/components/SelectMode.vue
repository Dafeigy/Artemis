<script setup>
import { useColorMode } from '@vueuse/core'
import { Label } from "@/components/ui/label";
import { RadioGroup, RadioGroupItem } from "@/components/ui/radio-group";
const mode = useColorMode()
</script>
<template>
    <RadioGroup >    

        <div class="flex items-center space-x-2">
          <RadioGroupItem id="r1" value="light" @click="mode = 'light'"/>
          <Label for="r1">Light</Label>
        </div>
        <div class="flex items-center space-x-2">
          <RadioGroupItem id="r2" value="dark" @click="mode = 'dark'"/>
          <Label for="r2">Dark</Label>
        </div>
        <div class="flex items-center space-x-2">
          <RadioGroupItem id="r3" value="auto" @click="mode = 'auto'"/>
          <Label for="r3">Auto</Label>
        </div>
      </RadioGroup>
</template>