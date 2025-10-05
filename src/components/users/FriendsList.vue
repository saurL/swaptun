<template>
  <div class="space-y-4">
    <div class="flex items-center justify-between">
      <h2 class="text-2xl font-bold text-text-primary">My friends</h2>
      <span
        class="px-3 py-1 rounded-full bg-background-secondary text-text-secondary border border-secondary text-sm"
      >
        {{ friends.length }} friend{{ friends.length > 1 ? "s" : "" }}
      </span>
    </div>

    <LoadingSpinner v-if="loading" size="lg" container-class="py-12" />

    <div v-else-if="friends.length > 0" class="space-y-3">
      <UserCard
        v-for="friend in friends"
        :key="friend.id"
        :user="friend"
        show-remove-button
        @remove="$emit('remove', friend)"
      />
    </div>

    <div
      v-else
      class="py-12 rounded-xl bg-background-secondary border border-secondary text-center"
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        class="h-16 w-16 mx-auto mb-4 text-text-secondary"
        fill="none"
        viewBox="0 0 24 24"
        stroke="currentColor"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z"
        />
      </svg>
      <p class="text-text-secondary text-lg mb-4">You don't have any friends yet</p>
      <p class="text-text-secondary text-sm">
        Search for users to add them as friends
      </p>
    </div>
  </div>
</template>

<script setup lang="ts">
import LoadingSpinner from "@/components/common/LoadingSpinner.vue";
import UserCard from "./UserCard.vue";
import type User from "@/models/user";

interface Props {
  friends: User[];
  loading?: boolean;
}

withDefaults(defineProps<Props>(), {
  loading: false,
});

defineEmits<{
  remove: [user: User];
}>();
</script>
