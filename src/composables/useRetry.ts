import { useToast } from './useToast'

interface RetryOptions {
  maxAttempts?: number
  delay?: number
  backoff?: boolean
  onRetry?: (attempt: number, error: any) => void
  shouldRetry?: (error: any) => boolean
}

export function useRetry() {
  const toast = useToast()

  /**
   * Execute a function with automatic retry on failure
   */
  const withRetry = async <T>(
    fn: () => Promise<T>,
    options: RetryOptions = {}
  ): Promise<T> => {
    const {
      maxAttempts = 3,
      delay = 1000,
      backoff = true,
      onRetry,
      shouldRetry = (error) => {
        // Retry on network errors by default
        return (
          error?.message?.includes('network') ||
          error?.message?.includes('timeout') ||
          error?.message?.includes('connection') ||
          error?.code === 'ECONNREFUSED' ||
          error?.code === 'ETIMEDOUT'
        )
      }
    } = options

    let lastError: any

    for (let attempt = 1; attempt <= maxAttempts; attempt++) {
      try {
        return await fn()
      } catch (error) {
        lastError = error

        // Check if we should retry
        if (attempt >= maxAttempts || !shouldRetry(error)) {
          throw error
        }

        // Calculate delay with optional exponential backoff
        const retryDelay = backoff ? delay * Math.pow(2, attempt - 1) : delay

        // Notify about retry
        if (onRetry) {
          onRetry(attempt, error)
        }

        // Show toast notification
        toast.warning(
          `Connection failed. Retrying (${attempt}/${maxAttempts})...`,
          'Network Error',
          2000
        )

        // Wait before retrying
        await new Promise(resolve => setTimeout(resolve, retryDelay))
      }
    }

    throw lastError
  }

  /**
   * Retry with exponential backoff
   */
  const withExponentialBackoff = async <T>(
    fn: () => Promise<T>,
    maxAttempts: number = 3
  ): Promise<T> => {
    return withRetry(fn, {
      maxAttempts,
      delay: 1000,
      backoff: true
    })
  }

  /**
   * Retry with fixed delay
   */
  const withFixedDelay = async <T>(
    fn: () => Promise<T>,
    maxAttempts: number = 3,
    delay: number = 1000
  ): Promise<T> => {
    return withRetry(fn, {
      maxAttempts,
      delay,
      backoff: false
    })
  }

  return {
    withRetry,
    withExponentialBackoff,
    withFixedDelay
  }
}
