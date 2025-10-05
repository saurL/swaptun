<template>
  <div>
    <!-- Recherche d'utilisateurs -->
    <section class="mb-8">
      <h2 class="text-xl font-bold text-text-primary mb-4">Search for friends</h2>
      <UserSearch @add="handleAddFriend" />
    </section>

    <!-- Liste des amis -->
    <section class="mb-8">
      <FriendsList
        :friends="userStore.friends"
        :loading="loadingFriends"
        @remove="handleRemoveFriend"
      />
    </section>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { useUserStore } from "@/store/user";
import UserSearch from "@/components/users/UserSearch.vue";
import FriendsList from "@/components/users/FriendsList.vue";
import type User from "@/models/user";

const userStore = useUserStore();
const loadingFriends = ref(false);

const handleAddFriend = (user: User) => {
  userStore.addFriend(user);
};

const handleRemoveFriend = (user: User) => {
  userStore.removeFriend(user);
};
</script>
