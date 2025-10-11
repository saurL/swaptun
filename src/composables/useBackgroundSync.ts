import { ref, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useOfflineCache } from './useOfflineCache'
import { useToast } from './useToast'

export function useBackgroundSync() {
  const { isOnline } = useOfflineCache()
  const toast = useToast()

  const isSyncing = ref(false)
  const lastSyncTime = ref<Date | null>(null)
  let syncInterval: number | null = null

  /**
   * Sync pending operations when coming back online
   */
  const syncPendingOperations = async () => {
    if (isSyncing.value || !isOnline.value) return

    try {
      isSyncing.value = true

      // Get pending operations from localStorage
      const pending = getPendingOperations()

      if (pending.length === 0) {
        lastSyncTime.value = new Date()
        return
      }

      // Process each pending operation
      for (const operation of pending) {
        try {
          await processOperation(operation)
          removePendingOperation(operation.id)
        } catch (error) {
          console.error('Failed to sync operation:', operation, error)
        }
      }

      lastSyncTime.value = new Date()
      toast.success('All changes synced successfully')
    } catch (error) {
      console.error('Background sync failed:', error)
    } finally {
      isSyncing.value = false
    }
  }

  /**
   * Add operation to pending queue
   */
  const queueOperation = (operation: {
    type: string
    data: any
    timestamp: number
  }) => {
    const operations = getPendingOperations()
    const id = Date.now() + Math.random()

    operations.push({
      id,
      ...operation
    })

    localStorage.setItem('pendingOperations', JSON.stringify(operations))

    // Try to sync immediately if online
    if (isOnline.value) {
      syncPendingOperations()
    }
  }

  /**
   * Get pending operations from storage
   */
  const getPendingOperations = (): any[] => {
    try {
      const stored = localStorage.getItem('pendingOperations')
      return stored ? JSON.parse(stored) : []
    } catch {
      return []
    }
  }

  /**
   * Remove operation from pending queue
   */
  const removePendingOperation = (id: number) => {
    const operations = getPendingOperations()
    const filtered = operations.filter(op => op.id !== id)
    localStorage.setItem('pendingOperations', JSON.stringify(filtered))
  }

  /**
   * Process a single operation
   */
  const processOperation = async (operation: any) => {
    switch (operation.type) {
      case 'share_playlist':
        await invoke('share_playlist', operation.data)
        break
      case 'add_friend':
        await invoke('add_friend', operation.data)
        break
      case 'remove_friend':
        await invoke('remove_friend', operation.data)
        break
      // Add more operation types as needed
      default:
        console.warn('Unknown operation type:', operation.type)
    }
  }

  /**
   * Start periodic background sync
   */
  const startBackgroundSync = (intervalMs: number = 30000) => {
    if (syncInterval) return

    syncInterval = window.setInterval(() => {
      if (isOnline.value) {
        syncPendingOperations()
      }
    }, intervalMs)
  }

  /**
   * Stop periodic background sync
   */
  const stopBackgroundSync = () => {
    if (syncInterval) {
      clearInterval(syncInterval)
      syncInterval = null
    }
  }

  // Auto-sync when coming back online
  onMounted(() => {
    // Sync on mount if online
    if (isOnline.value) {
      syncPendingOperations()
    }

    // Start periodic sync
    startBackgroundSync()
  })

  onUnmounted(() => {
    stopBackgroundSync()
  })

  return {
    isSyncing,
    lastSyncTime,
    syncPendingOperations,
    queueOperation,
    getPendingOperations,
    startBackgroundSync,
    stopBackgroundSync
  }
}
