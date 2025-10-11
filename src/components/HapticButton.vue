<template>
  <button
    :class="buttonClasses"
    :type="type"
    :disabled="disabled"
    @click="handleClick"
  >
    <slot />
  </button>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useHaptics } from '@/composables/useHaptics'

interface Props {
  type?: 'button' | 'submit' | 'reset'
  variant?: 'primary' | 'secondary' | 'danger' | 'ghost'
  size?: 'sm' | 'md' | 'lg'
  disabled?: boolean
  hapticIntensity?: 'light' | 'medium' | 'heavy'
}

const props = withDefaults(defineProps<Props>(), {
  type: 'button',
  variant: 'primary',
  size: 'md',
  disabled: false,
  hapticIntensity: 'medium'
})

const emit = defineEmits<{
  click: [event: MouseEvent]
}>()

const haptics = useHaptics()

const buttonClasses = computed(() => {
  const base = 'font-medium rounded-lg transition-all duration-200 disabled:opacity-50 disabled:cursor-not-allowed'

  const sizes = {
    sm: 'px-3 py-1.5 text-sm',
    md: 'px-4 py-2 text-base',
    lg: 'px-6 py-3 text-lg'
  }

  const variants = {
    primary: 'bg-gradient-to-r from-[#FF985C] to-[#E87A3A] text-white shadow-md hover:shadow-lg hover:from-[#FF985C] hover:to-[#FF985C] active:scale-95',
    secondary: 'bg-[#F4C9A6] text-[#2E2E2E] shadow-sm hover:shadow-md hover:bg-[#FF985C] hover:text-white active:scale-95',
    danger: 'bg-[#E74C3C] text-white shadow-md hover:shadow-lg hover:bg-[#C0392B] active:scale-95',
    ghost: 'bg-transparent text-[#CB5520] hover:bg-[#FFF8F3] active:scale-95'
  }

  return `${base} ${sizes[props.size]} ${variants[props.variant]}`
})

const handleClick = async (event: MouseEvent) => {
  if (props.disabled) return

  // Trigger haptic feedback based on intensity
  switch (props.hapticIntensity) {
    case 'light':
      await haptics.light()
      break
    case 'heavy':
      await haptics.heavy()
      break
    case 'medium':
    default:
      await haptics.medium()
      break
  }

  emit('click', event)
}
</script>
