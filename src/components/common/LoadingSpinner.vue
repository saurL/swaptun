<template>
  <div :class="['flex items-center justify-center', containerClass]">
    <div class="logo-spinner" :class="sizeClasses">
      <img
        src="/src/assets/images/icon.svg"
        alt="Loading"
        class="w-full h-full"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";

interface Props {
  size?: "sm" | "md" | "lg" | "xl";
  containerClass?: string;
}

const props = withDefaults(defineProps<Props>(), {
  size: "md",
  containerClass: "",
});

const sizeClasses = computed(() => {
  switch (props.size) {
    case "sm":
      return "w-8 h-8";
    case "md":
      return "w-12 h-12";
    case "lg":
      return "w-16 h-16";
    case "xl":
      return "w-24 h-24";
    default:
      return "w-12 h-12";
  }
});
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
</style>
