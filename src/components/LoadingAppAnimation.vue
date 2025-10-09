<template>
  <div class="loading-container">
    <div class="logo-spinner">
      <img
        src="/src/assets/images/icon.svg"
        alt="Loading"
        class="w-24 h-24"
      />
    </div>
  </div>
</template>

<script lang="ts" setup>
defineExpose();
</script>

<style scoped>
.loading-container {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100%;
}

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
</style>
