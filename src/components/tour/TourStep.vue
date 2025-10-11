<template>
  <div
    ref="cardRef"
    :class="[
      'tour-step-card bg-white rounded-2xl shadow-2xl p-6 w-[85vw] max-w-2xl',
      'border-2 border-primary fixed',
      isReady ? 'visible opacity-100' : 'invisible opacity-0',
    ]"
    :style="calculatePosition()"
    @click="$emit('next')"
  >
    <!-- Header -->
    <div class="flex items-start justify-between mb-4">
      <div class="flex-1">
        <h3 class="text-xl font-bold text-text-primary mb-1">
          {{ step.title }}
        </h3>
        <div class="flex items-center gap-2 text-sm text-text-secondary">
          <span>{{ currentStepNumber }} / {{ totalSteps }}</span>
          <div
            class="flex-1 h-2 bg-background-secondary rounded-full overflow-hidden"
          >
            <div
              class="h-full bg-gradient-primary transition-all duration-300"
              :style="{ width: `${progress}%` }"
            ></div>
          </div>
        </div>
      </div>

      <!-- Close button -->
      <button
        @click.stop="handleSkipClick"
        class="flex-shrink-0 ml-3 p-1 hover:bg-background-secondary rounded-lg transition-colors"
        title="Skip tour"
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          class="h-5 w-5 text-text-secondary hover:text-text-primary"
          fill="none"
          viewBox="0 0 24 24"
          stroke="currentColor"
          stroke-width="2"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            d="M6 18L18 6M6 6l12 12"
          />
        </svg>
      </button>
    </div>

    <!-- Description -->
    <p class="text-text-primary mb-6 leading-relaxed">
      {{ step.description }}
    </p>

    <!-- Actions -->
    <div class="flex flex-row justify-between">
      <div>
        <Button
          v-if="!isFirstStep"
          variant="secondary"
          size="sm"
          @click.stop="$emit('previous')"
          haptic-intensity="light"
        >
          Previous
        </Button>
      </div>
      <p class="flex items-center justify-center">Tap to continue</p>
    </div>

    <!-- Confirm Dialog -->
    <ConfirmDialog
      ref="confirmDialogRef"
      message="Are you sure you want to skip the tour? You can restart it later from the settings."
      confirm-text="Skip Tour"
      cancel-text="Continue"
      @confirm="emit('skip')"
    />
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import type { TourStep } from "@/config/tourSteps";
import Button from "@/components/common/Button.vue";
import ConfirmDialog from "@/components/common/ConfirmDialog.vue";

const isReady = ref(false);
interface Props {
  step: TourStep;
  currentStepNumber: number;
  totalSteps: number;
  progress: number;
  isFirstStep: boolean;
  isLastStep: boolean;
  targetPosition?: DOMRect | null;
}

const props = defineProps<Props>();

const emit = defineEmits<{
  next: [];
  previous: [];
  skip: [];
}>();

// Reference to the card element
const cardRef = ref<HTMLDivElement | null>(null);

// Reference to the confirm dialog
const confirmDialogRef = ref<InstanceType<typeof ConfirmDialog> | null>(null);

/**
 * Handle skip with confirmation
 */
const handleSkipClick = () => {
  confirmDialogRef.value?.show();
};

/**
 * Calculate position for all placements
 */
const calculatePosition = () => {
  // If no card ref yet, return empty style (will be hidden by v-show anyway)
  if (!cardRef.value) {
    return {};
  }

  const cardWidth = cardRef.value.offsetWidth;
  const cardHeight = cardRef.value.offsetHeight;
  const viewportWidth = window.innerWidth;
  const viewportHeight = window.innerHeight;
  console.log("Card dimensions:", cardWidth, cardHeight);
  console.log("Viewport dimensions:", viewportWidth, viewportHeight);
  const centerStyle = {
    top: `${(viewportHeight - cardHeight) / 2}px`,
    left: `${(viewportWidth - cardWidth) / 2}px`,
  };
  // If center placement, return center style
  if (props.step.placement === "center") {
    // Default to center position

    // Set isReady after a short delay to ensure smooth appearance
    if (!isReady.value) {
      setTimeout(() => {
        isReady.value = true;
      }, 100);
    }
    return centerStyle;
  }

  // If no target position, return center style
  if (!props.targetPosition) {
    if (!isReady.value) {
      setTimeout(() => {
        isReady.value = true;
      }, 100);
    }
    return centerStyle;
  }

  const rect = props.targetPosition;
  const offset = 20;
  const margin = 16; // Minimum margin from screen edges

  let calculatedStyle = centerStyle;

  switch (props.step.placement) {
    case "top":
      // Position above the target
      if (rect.top > cardHeight + offset + 20) {
        calculatedStyle = {
          left: `${rect.left + rect.width / 2 - cardWidth / 2}px`,
          top: `${rect.top - offset - cardHeight}px`,
        };
      }
      break;

    case "bottom":
      // Position below the target
      if (rect.bottom + cardHeight + offset + 20 < viewportHeight) {
        calculatedStyle = {
          left: `${rect.left + rect.width / 2 - cardWidth / 2}px`,
          top: `${rect.bottom + offset}px`,
        };
      }
      break;
  }

  // Adjust horizontal position to prevent overflow
  if (calculatedStyle.left) {
    let leftPos = parseFloat(calculatedStyle.left);

    // Ensure card doesn't overflow left
    if (leftPos < margin) {
      leftPos = margin;
    }

    // Ensure card doesn't overflow right
    if (leftPos + cardWidth > viewportWidth - margin) {
      leftPos = viewportWidth - cardWidth - margin;
    }

    calculatedStyle.left = `${leftPos}px`;
  }

  // Set isReady after calculating position
  if (!isReady.value) {
    setTimeout(() => {
      isReady.value = true;
    }, 100);
  }

  return calculatedStyle;
};
</script>

<style scoped>
.tour-step-card {
  z-index: 10002;
  animation: tour-card-appear 0.3s ease-out;
}

@keyframes tour-card-appear {
  from {
    opacity: 0;
    transform: scale(0.95);
  }
  to {
    opacity: 1;
    transform: scale(1);
  }
}
</style>
