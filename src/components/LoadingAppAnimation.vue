<template>
  <div class="loading-container">
    <div class="spinner"></div>
  </div>
</template>

<script lang="ts" setup>
defineExpose();
</script>

<style>
.loading-container {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100%;
}

.spinner {
  width: 40px;
  height: 40px;
  border: 4px solid transparent;
  border-top: 4px solid #00cfe8;
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}
</style>
