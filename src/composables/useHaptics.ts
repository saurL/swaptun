import {
  vibrate,
  impactFeedback,
  notificationFeedback,
  selectionFeedback
} from '@tauri-apps/plugin-haptics'

/**
 * Composable for managing haptic feedback throughout the app
 */
export function useHaptics() {
  /**
   * Light haptic for subtle interactions (navigation, selection)
   */
  const light = async () => {
    try {
      await impactFeedback('light')
    } catch (error) {
      console.warn('Haptic feedback not available:', error)
    }
  }

  /**
   * Medium haptic for standard button presses
   */
  const medium = async () => {
    try {
      await impactFeedback('medium')
    } catch (error) {
      console.warn('Haptic feedback not available:', error)
    }
  }

  /**
   * Heavy haptic for important actions (delete, send)
   */
  const heavy = async () => {
    try {
      await impactFeedback('heavy')
    } catch (error) {
      console.warn('Haptic feedback not available:', error)
    }
  }

  /**
   * Selection haptic for scrolling through items
   */
  const selection = async () => {
    try {
      await selectionFeedback()
    } catch (error) {
      console.warn('Haptic feedback not available:', error)
    }
  }

  /**
   * Success notification haptic
   */
  const success = async () => {
    try {
      await notificationFeedback('success')
    } catch (error) {
      console.warn('Haptic feedback not available:', error)
    }
  }

  /**
   * Warning notification haptic
   */
  const warning = async () => {
    try {
      await notificationFeedback('warning')
    } catch (error) {
      console.warn('Haptic feedback not available:', error)
    }
  }

  /**
   * Error notification haptic
   */
  const error = async () => {
    try {
      await notificationFeedback('error')
    } catch (error) {
      console.warn('Haptic feedback not available:', error)
    }
  }

  /**
   * Custom vibration with duration in seconds
   */
  const vibration = async (duration: number = 0.5) => {
    try {
      await vibrate(duration)
    } catch (error) {
      console.warn('Haptic feedback not available:', error)
    }
  }

  return {
    light,
    medium,
    heavy,
    selection,
    success,
    warning,
    error,
    vibration
  }
}
