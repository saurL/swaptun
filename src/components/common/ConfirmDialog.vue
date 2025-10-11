<template>
  <!-- Overlay -->
  <Transition name="overlay">
    <div
      v-if="isVisible"
      class="fixed inset-0 bg-black/50 z-[10003] flex items-center justify-center p-4"
      @click="handleCancel"
    >
      <!-- Dialog Card -->
      <Transition name="dialog">
        <div
          v-if="isVisible"
          class="bg-white rounded-2xl shadow-2xl p-6 w-full max-w-md"
          @click.stop
        >
          <!-- Message -->
          <p class="text-text-primary text-lg leading-relaxed mb-6">
            {{ message }}
          </p>

          <!-- Actions -->
          <div class="flex gap-3">
            <!-- Cancel Button -->
            <button
              @click="handleCancel"
              class="flex-1 px-4 py-3 rounded-xl font-medium
                     bg-background-secondary text-text-primary
                     hover:bg-gray-200 active:scale-95
                     transition-all duration-200 shadow-sm"
            >
              {{ cancelText }}
            </button>

            <!-- Confirm Button -->
            <button
              @click="handleConfirm"
              class="flex-1 px-4 py-3 rounded-xl font-medium
                     bg-gradient-to-br from-button-hover to-button-normal
                     text-button-text shadow-md
                     hover:shadow-lg hover:from-button-hover hover:to-button-hover
                     active:scale-95 transition-all duration-200"
            >
              {{ confirmText }}
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
  message: string;
  confirmText?: string;
  cancelText?: string;
}

const props = withDefaults(defineProps<Props>(), {
  confirmText: "Confirm",
  cancelText: "Cancel",
});

const emit = defineEmits<{
  confirm: [];
  cancel: [];
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
 * Handle confirm action
 */
const handleConfirm = () => {
  hide();
  emit("confirm");
};

/**
 * Handle cancel action
 */
const handleCancel = () => {
  hide();
  emit("cancel");
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
