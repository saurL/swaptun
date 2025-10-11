<template>
  <!-- Overlay -->
  <Transition name="overlay">
    <div
      v-if="isVisible"
      class="fixed inset-0 bg-black/50 z-[10003] flex items-center justify-center p-4"
      @click="handleClose"
    >
      <!-- Dialog Card -->
      <Transition name="dialog">
        <div
          v-if="isVisible"
          class="bg-white rounded-2xl shadow-2xl p-6 w-full max-w-md"
          @click.stop
        >
          <!-- Success Icon -->
          <div class="flex justify-center mb-4">
            <div class="w-16 h-16 rounded-full bg-success/10 flex items-center justify-center">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                class="h-8 w-8 text-success"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M5 13l4 4L19 7"
                />
              </svg>
            </div>
          </div>

          <!-- Platform Icon and Message -->
          <div class="flex flex-col items-center mb-6">
            <img
              v-if="platformIcon"
              :src="platformIcon"
              :alt="platformLabel"
              class="w-12 h-12 object-contain mb-3"
            />
            <p class="text-text-primary text-lg font-semibold text-center">
              Playlist transferred successfully!
            </p>
            <p class="text-text-secondary text-sm text-center mt-2">
              Your playlist has been transferred to {{ platformLabel }}
            </p>
          </div>

          <!-- Actions -->
          <div class="flex flex-col gap-3">
            <!-- Open App Button -->
            <button
              @click="handleOpenApp"
              class="w-full px-4 py-3 rounded-xl font-medium
                     bg-gradient-to-br from-button-hover to-button-normal
                     text-button-text shadow-md
                     hover:shadow-lg hover:from-button-hover hover:to-button-hover
                     active:scale-95 transition-all duration-200"
            >
              Open {{ platformLabel }}
            </button>

            <!-- Close Button -->
            <button
              @click="handleClose"
              class="w-full px-4 py-3 rounded-xl font-medium
                     bg-background-secondary text-text-primary
                     hover:bg-gray-200 active:scale-95
                     transition-all duration-200 shadow-sm"
            >
              Close
            </button>
          </div>
        </div>
      </Transition>
    </div>
  </Transition>
</template>

<script setup lang="ts">
import { ref } from "vue";

interface Props {
  platformLabel: string;
  platformIcon?: string;
  platformName: string;
}

const props = defineProps<Props>();

const emit = defineEmits<{
  openApp: [platformName: string];
  close: [];
}>();

const isVisible = ref(false);

/**
 * Show the dialog
 */
const show = () => {
  isVisible.value = true;
};

/**
 * Hide the dialog
 */
const hide = () => {
  isVisible.value = false;
};

/**
 * Handle open app action
 */
const handleOpenApp = () => {
  emit("openApp", props.platformName);
  hide();
};

/**
 * Handle close action
 */
const handleClose = () => {
  hide();
  emit("close");
};

// Expose methods for parent component
defineExpose({
  show,
  hide,
});
</script>

<style scoped>
/* Overlay transitions */
.overlay-enter-active,
.overlay-leave-active {
  transition: opacity 0.2s ease;
}

.overlay-enter-from,
.overlay-leave-to {
  opacity: 0;
}

/* Dialog transitions */
.dialog-enter-active {
  transition: all 0.3s ease-out;
}

.dialog-leave-active {
  transition: all 0.2s ease-in;
}

.dialog-enter-from {
  opacity: 0;
  transform: scale(0.9) translateY(-20px);
}

.dialog-leave-to {
  opacity: 0;
  transform: scale(0.95);
}
</style>
