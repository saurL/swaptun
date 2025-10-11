import { ref, computed } from 'vue'

interface CacheEntry<T> {
  data: T
  timestamp: number
  expiresIn: number // in milliseconds
}

const cache = new Map<string, CacheEntry<any>>()
const isOnline = ref(navigator.onLine)

// Listen to online/offline events
if (typeof window !== 'undefined') {
  window.addEventListener('online', () => {
    isOnline.value = true
  })

  window.addEventListener('offline', () => {
    isOnline.value = false
  })
}

export function useOfflineCache() {
  /**
   * Set data in cache with expiration
   */
  const setCache = <T>(key: string, data: T, expiresIn: number = 5 * 60 * 1000) => {
    cache.set(key, {
      data,
      timestamp: Date.now(),
      expiresIn
    })
  }

  /**
   * Get data from cache if not expired
   */
  const getCache = <T>(key: string): T | null => {
    const entry = cache.get(key)
    if (!entry) return null

    const now = Date.now()
    const age = now - entry.timestamp

    if (age > entry.expiresIn) {
      cache.delete(key)
      return null
    }

    return entry.data as T
  }

  /**
   * Clear specific cache entry or all cache
   */
  const clearCache = (key?: string) => {
    if (key) {
      cache.delete(key)
    } else {
      cache.clear()
    }
  }

  /**
   * Check if cache has valid entry
   */
  const hasCache = (key: string): boolean => {
    return getCache(key) !== null
  }

  /**
   * Fetch with cache support
   * If offline, returns cached data
   * If online, fetches fresh data and updates cache
   */
  const fetchWithCache = async <T>(
    key: string,
    fetcher: () => Promise<T>,
    options: {
      expiresIn?: number
      forceRefresh?: boolean
    } = {}
  ): Promise<T> => {
    const { expiresIn = 5 * 60 * 1000, forceRefresh = false } = options

    // If offline, return cached data
    if (!isOnline.value) {
      const cached = getCache<T>(key)
      if (cached) {
        return cached
      }
      throw new Error('No internet connection and no cached data available')
    }

    // If online and not forcing refresh, check cache first
    if (!forceRefresh) {
      const cached = getCache<T>(key)
      if (cached) {
        // Return cached data and optionally refresh in background
        fetcher()
          .then(data => setCache(key, data, expiresIn))
          .catch(() => {
            // Silently fail background refresh
          })
        return cached
      }
    }

    // Fetch fresh data
    try {
      const data = await fetcher()
      setCache(key, data, expiresIn)
      return data
    } catch (error) {
      // If fetch fails, try to return stale cache
      const cached = getCache<T>(key)
      if (cached) {
        console.warn('Fetch failed, returning stale cache')
        return cached
      }
      throw error
    }
  }

  return {
    isOnline: computed(() => isOnline.value),
    setCache,
    getCache,
    clearCache,
    hasCache,
    fetchWithCache
  }
}
