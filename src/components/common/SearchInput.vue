<template>
  <div class="relative w-full">
    <div class="absolute left-4 top-1/2 -translate-y-1/2 text-text-secondary">
      <svg
        xmlns="http://www.w3.org/2000/svg"
        class="h-5 w-5"
        fill="none"
        viewBox="0 0 24 24"
        stroke="currentColor"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
        />
      </svg>
    </div>
    <input
      :value="modelValue"
      @input="$emit('update:modelValue', ($event.target as HTMLInputElement).value)"
      type="text"
      :placeholder="placeholder"
      class="w-full pl-12 pr-4 py-3 rounded-xl bg-background-secondary text-text-primary placeholder-text-secondary border border-secondary focus:outline-none focus:ring-2 focus:ring-primary focus:border-primary focus:shadow-glow transition-all"
    />
    <button
      v-if="modelValue && clearable"
      @click="$emit('update:modelValue', '')"
      class="absolute right-4 top-1/2 -translate-y-1/2 text-text-secondary hover:text-text-primary transition-colors"
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        class="h-5 w-5"
        viewBox="0 0 20 20"
        fill="currentColor"
      >
        <path
          fill-rule="evenodd"
          d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z"
          clip-rule="evenodd"
        />
      </svg>
    </button>
  </div>
</template>

<script setup lang="ts">
interface Props {
  modelValue: string;
  placeholder?: string;
  clearable?: boolean;
}

withDefaults(defineProps<Props>(), {
  placeholder: "Rechercher...",
  clearable: true,
});

defineEmits<{
  "update:modelValue": [value: string];
}>();
</script>
