<template>
  <Card hoverable padding="md">
    <div class="flex items-center gap-4">
      <div
        class="flex-shrink-0 w-16 h-16 rounded-2xl bg-gradient-to-br from-[#2A2A2A] to-[#1E1E1E] flex items-center justify-center"
      >
        <img :src="icon" :alt="name" class="w-10 h-10" />
      </div>
      <div class="flex-1 min-w-0">
        <h3 class="text-lg font-semibold text-white">{{ name }}</h3>
        <p class="text-sm text-gray-400">
          {{ connected ? "Connecté" : "Non connecté" }}
        </p>
      </div>
      <Button
        :variant="connected ? 'outline' : 'primary'"
        size="sm"
        :loading="loading"
        @click="$emit('connect')"
      >
        {{ connected ? "Synchroniser" : "Connecter" }}
      </Button>
    </div>
  </Card>
</template>

<script setup lang="ts">
import Card from "@/components/common/Card.vue";
import Button from "@/components/common/Button.vue";

interface Props {
  name: string;
  icon: string;
  connected?: boolean;
  loading?: boolean;
}

withDefaults(defineProps<Props>(), {
  connected: false,
  loading: false,
});

defineEmits<{
  connect: [];
}>();
</script>
