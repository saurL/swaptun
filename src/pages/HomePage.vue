<template>
  <MainLayout>
    <router-view />
  </MainLayout>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted } from "vue";
import { useUserStore } from "@/store/user";
import { useNotifications } from "@/composables/useNotifications";
import { usePlaylistManagement } from "@/composables/usePlaylistManagement";
import MainLayout from "@/layouts/MainLayout.vue";

const userStore = useUserStore();

const { setupNotifications, cleanup: cleanupNotifications } =
  useNotifications();
const {
  setupPlaylistListeners,
  fetchAllPlaylists,
  cleanup: cleanupPlaylists,
} = usePlaylistManagement();

onMounted(async () => {
  // Setup notifications
  await setupNotifications();

  // Setup playlist listeners
  await setupPlaylistListeners();

  // Fetch all playlists
  await fetchAllPlaylists();
});

onUnmounted(() => {
  cleanupNotifications();
  cleanupPlaylists();
});
</script>
