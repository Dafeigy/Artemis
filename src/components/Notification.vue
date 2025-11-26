<script setup lang="ts">
import { Alert, AlertDescription, AlertTitle } from './ui/alert'
import { AlertCircle, CheckCircle, Info, X } from 'lucide-vue-next'
import { notificationService, type Notification } from '../lib/utils'

// 获取通知数据
const notifications = notificationService.getNotifications()

// 根据通知类型获取对应的图标组件
const getIconComponent = (type: Notification['type']) => {
  switch (type) {
    case 'success':
      return CheckCircle
    case 'error':
      return AlertCircle
    case 'info':
      return Info
    case 'warning':
      return AlertCircle
    default:
      return Info
  }
}

// 关闭通知
const handleClose = (id: string) => {
  notificationService.close(id)
}

// 获取通知对应的Alert variant
const getAlertVariant = (type: Notification['type']) => {
  switch (type) {
    case 'error':
      return 'destructive'
    default:
      return 'default'
  }
}
</script>

<template>
  <div class="fixed top-4 right-4 z-50 flex flex-col gap-2 w-80">
    <div
      v-for="notification in notifications"
      :key="notification.id"
      :class="[
        'transition-all duration-300 ease-in-out transform',
        notification.show
          ? 'opacity-100 translate-x-0'
          : 'opacity-0 translate-x-full pointer-events-none'
      ]"
    >
      <Alert :variant="getAlertVariant(notification.type)" class="relative">
        <component :is="getIconComponent(notification.type)" class="size-4" />
        <AlertTitle>{{ notification.title }}</AlertTitle>
        <AlertDescription>{{ notification.description }}</AlertDescription>
        <button
          type="button"
          class="absolute top-2 right-2 rounded-full p-1 hover:bg-white/10 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary"
          @click="handleClose(notification.id)"
        >
          <X class="h-4 w-4" />
        </button>
      </Alert>
    </div>
  </div>
</template>