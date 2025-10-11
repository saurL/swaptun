import { ref } from 'vue'
import { useHaptics } from './useHaptics'

export interface ToastOptions {
  type?: 'success' | 'error' | 'warning' | 'info'
  title?: string
  message: string
  duration?: number
}

interface Toast extends ToastOptions {
  id: number
  show: boolean
}

const toasts = ref<Toast[]>([])
let nextId = 0

export function useToast() {
  const haptics = useHaptics()

  const show = (options: ToastOptions) => {
    const id = nextId++
    const toast: Toast = {
      id,
      show: true,
      type: options.type || 'info',
      title: options.title,
      message: options.message,
      duration: options.duration ?? 3000
    }

    toasts.value.push(toast)

    // Trigger haptic based on type
    switch (toast.type) {
      case 'success':
        haptics.success()
        break
      case 'error':
        haptics.error()
        break
      case 'warning':
        haptics.warning()
        break
      case 'info':
      default:
        haptics.light()
        break
    }

    if (toast.duration && toast.duration > 0) {
      setTimeout(() => {
        remove(id)
      }, toast.duration)
    }

    return id
  }

  const remove = (id: number) => {
    const index = toasts.value.findIndex(t => t.id === id)
    if (index !== -1) {
      toasts.value[index].show = false
      setTimeout(() => {
        toasts.value.splice(index, 1)
      }, 300) // Wait for animation to complete
    }
  }

  const success = (message: string, title?: string, duration?: number) => {
    return show({ type: 'success', message, title, duration })
  }

  const error = (message: string, title?: string, duration?: number) => {
    return show({ type: 'error', message, title, duration })
  }

  const warning = (message: string, title?: string, duration?: number) => {
    return show({ type: 'warning', message, title, duration })
  }

  const info = (message: string, title?: string, duration?: number) => {
    return show({ type: 'info', message, title, duration })
  }

  return {
    toasts,
    show,
    remove,
    success,
    error,
    warning,
    info
  }
}
