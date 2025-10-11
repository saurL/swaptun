<template>
  <Teleport to="body">
    <Transition name="tour-fade">
      <div v-if="isActive" class="tour-overlay-container">
        <!-- Single persistent overlay with optional cutout -->
        <svg class="tour-overlay-svg" @click="handleOverlayClick">
          <defs>
            <mask id="spotlight-mask">
              <!-- White background (visible) -->
              <rect x="0" y="0" width="100%" height="100%" fill="white" />
              <!-- Black cutout (invisible) - only when highlighting -->
              <rect
                v-if="shouldShowSpotlight"
                :x="spotlightStyle.left"
                :y="spotlightStyle.top"
                :width="spotlightStyle.width"
                :height="spotlightStyle.height"
                :rx="12"
                fill="black"
              />
            </mask>
          </defs>
          <!-- Overlay with mask applied -->
          <rect
            x="0"
            y="0"
            width="100%"
            height="100%"
            fill="rgba(0, 0, 0, 0.5)"
            mask="url(#spotlight-mask)"
          />
        </svg>

        <!-- Spotlight border for target element -->
        <div
          v-if="shouldShowSpotlight"
          class="tour-spotlight-border"
          :style="spotlightBorderStyle"
        ></div>

        <!-- Tour step card -->
        <TourStep
          v-if="currentStep"
          :step="currentStep"
          :current-step-number="currentStepIndex + 1"
          :total-steps="totalSteps"
          :progress="progress"
          :is-first-step="isFirstStep"
          :is-last-step="isLastStep"
          :target-position="targetRect"
          @next="handleNext"
          @previous="handlePrevious"
          @skip="handleSkip"
        />
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted, nextTick } from "vue";
import { useTour } from "@/composables/useTour";
import TourStep from "./TourStep.vue";

const {
  isActive,
  currentStep,
  currentStepIndex,
  totalSteps,
  progress,
  isFirstStep,
  isLastStep,
  nextStep,
  previousStep,
  skipTour,
  getTargetElement,
} = useTour();

const targetRect = ref<DOMRect | null>(null);

/**
 * Update target element position
 */
const updateTargetPosition = async () => {
  await nextTick();

  const element = getTargetElement();
  if (element) {
    targetRect.value = element.getBoundingClientRect();
  } else {
    targetRect.value = null;
  }
};

/**
 * Check if we should show the spotlight (highlight with cutout)
 */
const shouldShowSpotlight = computed(() => {
  return !!(currentStep.value && currentStep.value.highlight && targetRect.value);
});

/**
 * Spotlight style for highlighting target element
 */
const spotlightStyle = computed(() => {
  if (!targetRect.value) return { left: 0, top: 0, width: 0, height: 0 };

  const rect = targetRect.value;
  const padding = 8; // Extra space around element

  return {
    left: rect.left - padding,
    top: rect.top - padding,
    width: rect.width + padding * 2,
    height: rect.height + padding * 2,
  };
});

/**
 * Spotlight border style (for the orange glow animation)
 */
const spotlightBorderStyle = computed(() => {
  if (!targetRect.value) return {};

  const rect = targetRect.value;
  const padding = 8;

  return {
    left: `${rect.left - padding}px`,
    top: `${rect.top - padding}px`,
    width: `${rect.width + padding * 2}px`,
    height: `${rect.height + padding * 2}px`,
  };
});

/**
 * Handle next step
 */
const handleNext = async () => {
  await nextStep();
  await updateTargetPosition();
};

/**
 * Handle previous step
 */
const handlePrevious = async () => {
  await previousStep();
  await updateTargetPosition();
};

/**
 * Handle skip tour
 */
const handleSkip = () => {
  skipTour();
};

/**
 * Handle overlay click (skip tour)
 */
const handleOverlayClick = () => {
  handleNext();
};

// Watch for step changes
watch(currentStep, async () => {
  if (currentStep.value) {
    await updateTargetPosition();
  }
});

// Watch for window resize
let resizeObserver: ResizeObserver | null = null;

onMounted(() => {
  updateTargetPosition();

  // Update position on window resize
  window.addEventListener("resize", updateTargetPosition);
  window.addEventListener("scroll", updateTargetPosition, true);

  // Observe DOM changes for target element
  resizeObserver = new ResizeObserver(() => {
    updateTargetPosition();
  });

  const targetElement = getTargetElement();
  if (targetElement) {
    resizeObserver.observe(targetElement);
  }
});

onUnmounted(() => {
  window.removeEventListener("resize", updateTargetPosition);
  window.removeEventListener("scroll", updateTargetPosition, true);

  if (resizeObserver) {
    resizeObserver.disconnect();
  }
});
</script>

<style scoped>
.tour-overlay-container {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  z-index: 10000;
  pointer-events: auto;
}

.tour-overlay {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  pointer-events: auto;
  animation: tour-overlay-appear 0.3s ease-out;
}

.tour-overlay-svg {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  z-index: 10000;
  pointer-events: auto;
}

.tour-spotlight-border {
  position: absolute;
  border-radius: 12px;
  z-index: 10001;
  pointer-events: none;
  transition: all 0.3s ease-out;
  animation: tour-spotlight-pulse 2s infinite;
  box-shadow: 0 0 0 3px rgba(203, 85, 32, 0.8),
    0 0 20px 3px rgba(203, 85, 32, 0.4);
}

/* Animations */
@keyframes tour-overlay-appear {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}

@keyframes tour-spotlight-pulse {
  0%,
  100% {
    box-shadow: 0 0 0 3px rgba(203, 85, 32, 0.8),
      0 0 20px 3px rgba(203, 85, 32, 0.4);
  }
  50% {
    box-shadow: 0 0 0 3px rgba(203, 85, 32, 1),
      0 0 30px 3px rgba(203, 85, 32, 0.6);
  }
}

/* Transition */
.tour-fade-enter-active,
.tour-fade-leave-active {
  transition: opacity 0.3s ease;
}

.tour-fade-enter-from,
.tour-fade-leave-to {
  opacity: 0;
}
</style>
