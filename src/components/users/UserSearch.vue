<template>
  <SearchInputBackend
    placeholder="Search for a user..."
    :search-function="searchUsers"
  >
    <template #default="{ results, loading, searchQuery }">
      <LoadingSpinner v-if="loading" size="lg" container-class="py-8" />

      <div v-else-if="results.length > 0" class="space-y-3">
        <UserCard
          v-for="user in results"
          :key="user.id"
          :user="user"
          show-add-button
          @add="$emit('add', user)"
        />
      </div>

      <div
        v-else-if="searchQuery && !loading"
        class="py-12 text-center text-gray-400"
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          class="h-16 w-16 mx-auto mb-4 text-gray-600"
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
        <p>No user found</p>
      </div>
    </template>
  </SearchInputBackend>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import SearchInputBackend from "@/components/common/SearchInputBackend.vue";
import LoadingSpinner from "@/components/common/LoadingSpinner.vue";
import UserCard from "./UserCard.vue";
import type User from "@/models/user";
defineEmits<{
  add: [user: User];
}>();

const searchUsers = async (term: string): Promise<User[]> => {
  return await invoke<User[]>("search_non_friends_users", { search: term });
};
</script>
