<script setup lang="ts">
import { ref, watch } from "vue";

interface ErrorNotification {
  type: "server_error" | "network_error";
  message: string;
}

const props = defineProps<{
  error: ErrorNotification | null;
}>();

const emit = defineEmits<{
  dismiss: [];
}>();

const isVisible = ref(false);
const autoHideTimer = ref<number | null>(null);

watch(
  () => props.error,
  (newError) => {
    if (newError) {
      isVisible.value = true;
      startAutoHideTimer();
    }
  }
);

const startAutoHideTimer = () => {
  if (autoHideTimer.value) {
    clearTimeout(autoHideTimer.value);
  }
  autoHideTimer.value = window.setTimeout(() => {
    dismiss();
  }, 5000);
};

const dismiss = () => {
  isVisible.value = false;
  if (autoHideTimer.value) {
    clearTimeout(autoHideTimer.value);
  }
  emit("dismiss");
};

const getErrorIcon = () => {
  return props.error?.type === "network_error" ? "📡" : "⚠️";
};

const getErrorTitle = () => {
  return props.error?.type === "network_error"
    ? "Connection Error"
    : "Server Error";
};
</script>

<template>
  <Transition name="slide-down">
    <div
      v-if="isVisible && error"
      @click="dismiss"
      class="fixed top-[var(--safe-area-inset-top)] left-1/2 -translate-x-1/2 z-50 w-[90%] max-w-md cursor-pointer"
    >
      <div
        class="bg-[#E74C3C] p-4 text-white rounded-lg shadow-lg flex items-start gap-3"
      >
        <div class="text-2xl">{{ getErrorIcon() }}</div>
        <div class="flex-1">
          <div class="font-semibold mb-1">{{ getErrorTitle() }}</div>
          <div class="text-sm opacity-90">{{ error.message }}</div>
        </div>
        <button
          @click.stop="dismiss"
          class="text-white opacity-75 hover:opacity-100 transition-opacity"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            class="h-5 w-5"
            viewBox="0 0 20 20"
            fill="currentColor"
          >
            <path
              fill-rule="evenodd"
              d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z"
              clip-rule="evenodd"
            />
          </svg>
        </button>
      </div>
    </div>
  </Transition>
</template>

<style scoped>
.slide-down-enter-active,
.slide-down-leave-active {
  transition: all 0.3s ease;
}

.slide-down-enter-from {
  opacity: 0;
  transform: translate(-50%, -100%);
}

.slide-down-leave-to {
  opacity: 0;
  transform: translate(-50%, -20px);
}
</style>
