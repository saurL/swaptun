<template>
  <div :class="['flex items-center justify-center', containerClass]">
    <div
      :class="[
        'rounded-full border-2 border-transparent animate-spin',
        sizeClasses,
        colorClasses,
      ]"
    />
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";

interface Props {
  size?: "sm" | "md" | "lg" | "xl";
  color?: "primary" | "secondary" | "white";
  containerClass?: string;
}

const props = withDefaults(defineProps<Props>(), {
  size: "md",
  color: "primary",
  containerClass: "",
});

const sizeClasses = computed(() => {
  switch (props.size) {
    case "sm":
      return "h-4 w-4 border-2";
    case "md":
      return "h-8 w-8 border-2";
    case "lg":
      return "h-12 w-12 border-3";
    case "xl":
      return "h-16 w-16 border-4";
    default:
      return "h-8 w-8 border-2";
  }
});

const colorClasses = computed(() => {
  switch (props.color) {
    case "primary":
      return "border-t-[#00CFE8] border-r-[#00CFE8]";
    case "secondary":
      return "border-t-[#FFC436] border-r-[#FFC436]";
    case "white":
      return "border-t-white border-r-white";
    default:
      return "border-t-[#00CFE8] border-r-[#00CFE8]";
  }
});
</script>
