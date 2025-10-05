<template>
  <button
    :class="[
      'relative rounded-xl font-semibold transition-all duration-200 transform',
      'active:scale-95',
      variantClasses,
      sizeClasses,
      disabled ? 'opacity-50 cursor-not-allowed' : '',
    ]"
    :disabled="disabled || loading"
    @click="$emit('click', $event)"
  >
    <span v-if="loading" class="inline-block animate-spin mr-2">⚙</span>
    <slot />
  </button>
</template>

<script setup lang="ts">
import { computed } from "vue";

interface Props {
  variant?: "primary" | "secondary" | "outline" | "danger";
  size?: "sm" | "md" | "lg";
  disabled?: boolean;
  loading?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  variant: "primary",
  size: "md",
  disabled: false,
  loading: false,
});

defineEmits<{
  click: [event: MouseEvent];
}>();

const variantClasses = computed(() => {
  switch (props.variant) {
    case "primary":
      // Gradient background with depth, shadow effects, and glow on hover
      return "bg-gradient-primary text-text-button shadow-button hover:shadow-button-hover hover:shadow-glow";
    case "secondary":
      return "bg-secondary text-text-primary hover:bg-secondary/90 shadow-sm hover:shadow-md";
    case "outline":
      return "border-2 border-primary text-primary hover:bg-gradient-primary hover:text-text-button hover:shadow-button transition-colors";
    case "danger":
      return "bg-error text-text-button hover:bg-error/90 shadow-sm hover:shadow-md";
    default:
      return "";
  }
});

const sizeClasses = computed(() => {
  switch (props.size) {
    case "sm":
      return "text-sm px-4 py-2";
    case "md":
      return "text-base px-6 py-3";
    case "lg":
      return "text-lg px-8 py-4";
    default:
      return "";
  }
});
</script>
