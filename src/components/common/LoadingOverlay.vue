<template>
  <div
    v-if="show"
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/40 backdrop-blur-sm"
  >
    <div class="flex flex-col items-center gap-4">
      <!-- Logo animation -->
      <div class="logo-spinner">
        <img
          src="/src/assets/images/icon.svg"
          alt="Loading"
          class="w-20 h-20"
        />
      </div>

      <!-- Optional message -->
      <p v-if="message" class="text-white text-lg font-semibold">
        {{ message }}
      </p>
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  show: boolean;
  message?: string;
}>();
</script>

<style scoped>
.logo-spinner {
  animation: spin-ease 2s cubic-bezier(0.4, 0, 0.2, 1) infinite;
}

@keyframes spin-ease {
  0% {
    transform: rotate(0deg);
  }
  50% {
    transform: rotate(180deg);
  }
  100% {
    transform: rotate(360deg);
  }
}

/* Alternative: acceleration then deceleration */
@keyframes spin-smooth {
  0% {
    transform: rotate(0deg);
    animation-timing-function: cubic-bezier(0.55, 0.085, 0.68, 0.53);
  }
  50% {
    transform: rotate(180deg);
    animation-timing-function: cubic-bezier(0.25, 0.46, 0.45, 0.94);
  }
  100% {
    transform: rotate(360deg);
  }
}
</style>
