<template>
  <div
    :class="[
      'rounded-xl transition-all duration-200',
      variantClasses,
      hoverable ? 'hover:shadow-card-hover hover:-translate-y-0.5 cursor-pointer' : '',
      paddingClasses,
    ]"
  >
    <slot />
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";

interface Props {
  variant?: "default" | "glass" | "white";
  hoverable?: boolean;
  padding?: "none" | "sm" | "md" | "lg";
}

const props = withDefaults(defineProps<Props>(), {
  variant: "default",
  hoverable: false,
  padding: "md",
});

const variantClasses = computed(() => {
  switch (props.variant) {
    case 'white':
      // Pure white card with subtle shadow for depth
      return 'bg-white border border-secondary shadow-card';
    case 'glass':
      // Glass morphism effect with blur
      return 'bg-white/80 backdrop-blur-md border border-secondary/50 shadow-md';
    case 'default':
    default:
      // Warm secondary background with subtle border
      return 'bg-background-secondary border border-secondary/60 shadow-sm';
  }
});

const paddingClasses = computed(() => {
  switch (props.padding) {
    case "none":
      return "p-0";
    case "sm":
      return "p-3";
    case "md":
      return "p-5";
    case "lg":
      return "p-8";
    default:
      return "p-5";
  }
});
</script>
