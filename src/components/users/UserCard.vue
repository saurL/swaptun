<template>
  <Card hoverable padding="md" variant="white">
    <div class="flex items-center justify-between gap-4">
      <div class="flex items-center gap-3 flex-1 min-w-0">
        <div class="flex-1 min-w-0">
          <h3 class="text-lg font-semibold text-text-primary truncate">
            {{ user.username }}
          </h3>
        </div>
      </div>
      <Button
        v-if="showAddButton"
        variant="primary"
        size="sm"
        @click="$emit('add', user)"
      >
        Add
      </Button>
      <Button
        v-if="showRemoveButton"
        variant="danger"
        size="sm"
        @click="$emit('remove', user)"
      >
        Remove
      </Button>
    </div>
  </Card>
</template>

<script setup lang="ts">
import Card from "@/components/common/Card.vue";
import Button from "@/components/common/Button.vue";
import type User from "@/models/user";

interface Props {
  user: User;
  showAddButton?: boolean;
  showRemoveButton?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  showAddButton: false,
  showRemoveButton: false,
});

defineEmits<{
  add: [user: User];
  remove: [user: User];
}>();
</script>
