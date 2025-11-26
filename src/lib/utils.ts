import type { ClassValue } from "clsx"
import { clsx } from "clsx"
import { twMerge } from "tailwind-merge"
import { save } from "@tauri-apps/plugin-dialog"
import { writeTextFile } from "@tauri-apps/plugin-fs"
import { ref, reactive } from 'vue'

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

// 通知类型定义
export type NotificationType = 'success' | 'error' | 'info' | 'warning'

// 通知接口
export interface Notification {
  id: string
  type: NotificationType
  title: string
  description: string
  duration?: number
  show: boolean
}

// 通知服务类
export class NotificationService {
  private notifications = reactive<Notification[]>([])
  private counter = 0

  // 获取所有通知
  getNotifications() {
    return this.notifications
  }

  // 创建唯一ID
  private generateId(): string {
    return `notification-${Date.now()}-${this.counter++}`
  }

  // 显示通知
  show(type: NotificationType, title: string, description: string, duration: number = 3000): Notification {
    const notification: Notification = {
      id: this.generateId(),
      type,
      title,
      description,
      duration,
      show: true
    }

    this.notifications.push(notification)

    // 自动关闭通知
    if (duration > 0) {
      setTimeout(() => {
        this.close(notification.id)
      }, duration)
    }

    return notification
  }

  // 显示成功通知
  success(title: string, description: string, duration?: number) {
    return this.show('success', title, description, duration)
  }

  // 显示错误通知
  error(title: string, description: string, duration?: number) {
    return this.show('error', title, description, duration)
  }

  // 显示信息通知
  info(title: string, description: string, duration?: number) {
    return this.show('info', title, description, duration)
  }

  // 显示警告通知
  warning(title: string, description: string, duration?: number) {
    return this.show('warning', title, description, duration)
  }

  // 关闭通知
  close(id: string) {
    const index = this.notifications.findIndex(n => n.id === id)
    if (index !== -1) {
      this.notifications[index].show = false
      // 动画完成后移除
      setTimeout(() => {
        this.notifications.splice(index, 1)
      }, 300)
    }
  }

  // 清除所有通知
  clear() {
    this.notifications.forEach(n => n.show = false)
    setTimeout(() => {
      this.notifications.length = 0
    }, 300)
  }
}

// 创建全局通知服务实例
export const notificationService = new NotificationService()


/**
 * 导出日志内容到文本文件
 * @returns Promise<boolean> 是否成功导出
 */
export async function exportLogs(): Promise<boolean> {
  try {
    // 获取日志容器元素
    const logContainer = document.getElementById('log-container')
    if (!logContainer) {
      console.error('日志容器未找到')
      return false
    }
    
    // 获取所有pre标签
    const preElements = logContainer.querySelectorAll('pre')
    
    // 按顺序提取所有pre标签的内容
    const logContent = Array.from(preElements)
      .map(pre => pre.textContent || '')
      .join('\n')
    
    if (logContent.trim() === '') {
      console.warn('日志内容为空')
      return false
    }
    
    // 打开保存对话框
    const result = await save({
      title: '导出日志',
      defaultPath: 'serial_logs.txt',
      filters: [
        {
          name: '文本文件',
          extensions: ['txt']
        }
      ]
    })
    
    // 如果用户选择了文件路径
    if (result) {
      // 写入文件
      await writeTextFile(result, logContent)
      return true
    }
    
    return false
  } catch (error) {
    console.error('导出日志时出错:', error)
    return false
  }
}
