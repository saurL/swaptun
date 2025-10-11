import { ref, computed, watch } from 'vue'
import { useRouter } from 'vue-router'
import { tourSteps, TOUR_VERSION, type TourStep } from '@/config/tourSteps'
import { useHaptics } from './useHaptics'

const STORAGE_KEY = 'swaply_tour_state'

interface TourState {
  completed: boolean
  skipped: boolean
  version: string
  currentStep: number
}

// Global state
const isActive = ref(false)
const currentStepIndex = ref(0)

export function useTour() {
  const haptics = useHaptics()
  const router = useRouter()

  const currentStep = computed<TourStep | null>(() => {
    if (currentStepIndex.value >= 0 && currentStepIndex.value < tourSteps.length) {
      return tourSteps[currentStepIndex.value]
    }
    return null
  })

  const totalSteps = computed(() => tourSteps.length)

  const isFirstStep = computed(() => currentStepIndex.value === 0)
  const isLastStep = computed(() => currentStepIndex.value === tourSteps.length - 1)

  const progress = computed(() => {
    return Math.round(((currentStepIndex.value + 1) / totalSteps.value) * 100)
  })

  /**
   * Load tour state from localStorage
   */
  const loadTourState = (): TourState => {
    try {
      const stored = localStorage.getItem(STORAGE_KEY)
      if (stored) {
        return JSON.parse(stored)
      }
    } catch (error) {
      console.error('Failed to load tour state:', error)
    }

    return {
      completed: false,
      skipped: false,
      version: TOUR_VERSION,
      currentStep: 0
    }
  }

  /**
   * Save tour state to localStorage
   */
  const saveTourState = (state: Partial<TourState>) => {
    try {
      const current = loadTourState()
      const updated = { ...current, ...state }
      localStorage.setItem(STORAGE_KEY, JSON.stringify(updated))
    } catch (error) {
      console.error('Failed to save tour state:', error)
    }
  }

  /**
   * Check if user has seen the tour
   */
  const hasSeenTour = computed(() => {
    const state = loadTourState()
    return (state.completed || state.skipped) && state.version === TOUR_VERSION
  })

  /**
   * Start the tour
   */
  const startTour = async () => {
    currentStepIndex.value = 0
    isActive.value = true
    await haptics.light()

    // Navigate to first step route if specified
    const firstStep = tourSteps[0]
    if (firstStep.route) {
      await router.push(firstStep.route)
    }

    // Execute first step action if any
    if (firstStep.action) {
      await firstStep.action()
    }

    saveTourState({ currentStep: 0 })
  }

  /**
   * Go to next step
   */
  const nextStep = async () => {
    if (isLastStep.value) {
      await completeTour()
      return
    }

    currentStepIndex.value++
    await haptics.light()

    const step = tourSteps[currentStepIndex.value]

    // Navigate to step route if specified
    if (step.route && router.currentRoute.value.path !== step.route) {
      await router.push(step.route)
    }

    // Execute step action if any
    if (step.action) {
      await step.action()
    }

    // Scroll to target element
    await scrollToTarget(step.target)

    saveTourState({ currentStep: currentStepIndex.value })
  }

  /**
   * Go to previous step
   */
  const previousStep = async () => {
    if (isFirstStep.value) return

    currentStepIndex.value--
    await haptics.light()

    const step = tourSteps[currentStepIndex.value]

    // Navigate to step route if specified
    if (step.route && router.currentRoute.value.path !== step.route) {
      await router.push(step.route)
    }

    // Scroll to target element
    await scrollToTarget(step.target)

    saveTourState({ currentStep: currentStepIndex.value })
  }

  /**
   * Skip the tour
   */
  const skipTour = async () => {
    isActive.value = false
    await haptics.medium()
    saveTourState({
      skipped: true,
      completed: false,
      version: TOUR_VERSION
    })
  }

  /**
   * Complete the tour
   */
  const completeTour = async () => {
    isActive.value = false
    await haptics.success()
    saveTourState({
      completed: true,
      skipped: false,
      version: TOUR_VERSION
    })
  }

  /**
   * Reset tour (for testing or user request)
   */
  const resetTour = () => {
    localStorage.removeItem(STORAGE_KEY)
    currentStepIndex.value = 0
    isActive.value = false
  }

  /**
   * Scroll to target element smoothly
   */
  const scrollToTarget = async (selector: string) => {
    if (selector === 'body') return

    // Wait a bit for DOM to update
    await new Promise(resolve => setTimeout(resolve, 100))

    const element = document.querySelector(selector)
    if (element) {
      element.scrollIntoView({
        behavior: 'smooth',
        block: 'center'
      })
    }
  }

  /**
   * Get target element for current step
   */
  const getTargetElement = (): HTMLElement | null => {
    if (!currentStep.value) return null
    if (currentStep.value.target === 'body') return document.body

    return document.querySelector(currentStep.value.target) as HTMLElement | null
  }

  const awaitTourEnd = async () => {
    if (localStorage.getItem(STORAGE_KEY)) return Promise.resolve();

    return new Promise<void>(resolve => {
      const stopWatch = watch(isActive, (newVal) => {
        console.log("Tour active state changed:", newVal);
        if (!newVal) {
          stopWatch();
          resolve();
        }
      });
    });
  }

  return {
    // State
    isActive,
    currentStep,
    currentStepIndex,
    totalSteps,
    progress,
    isFirstStep,
    isLastStep,
    hasSeenTour,

    // Methods
    startTour,
    nextStep,
    previousStep,
    skipTour,
    completeTour,
    resetTour,
    getTargetElement,
    awaitTourEnd
  }
}
