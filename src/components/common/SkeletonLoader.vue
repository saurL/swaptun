<template>
  <div
    :class="[
      'animate-pulse bg-gradient-to-r from-background-secondary via-[#F4C9A6]/20 to-background-secondary bg-[length:200%_100%]',
      rounded ? `rounded-${rounded}` : '',
      className
    ]"
    :style="{
      width: width,
      height: height
    }"
  ></div>
</template>

<script setup lang="ts">
interface Props {
  width?: string
  height?: string
  rounded?: 'sm' | 'md' | 'lg' | 'xl' | '2xl' | 'full'
  className?: string
}

withDefaults(defineProps<Props>(), {
  width: '100%',
  height: '20px',
  rounded: 'md',
  className: ''
})
</script>

<style scoped>
@keyframes shimmer {
  0% {
    background-position: -200% 0;
  }
  100% {
    background-position: 200% 0;
  }
}

.animate-pulse {
  animation: shimmer 2s infinite;
}
</style>
