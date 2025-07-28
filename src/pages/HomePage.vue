<template>
  <MainLayout>
    <GreetingCard :userName=userStore.username />

    <button @click="connectToSpotify" class="connect-button">
      Connect to Spotify
    </button>

    <button @click="connectToYoutube" class="connect-button">
      Connect to youtube
    </button>
    <div class="flex flex-col flex-1 min-h-0 overflow-y-auto">
      <PlaylistList />
    </div>
  </MainLayout>
</template>

<script setup>
import MainLayout from "@/layouts/MainLayout.vue";
import GreetingCard from "@/components/GreetingCard.vue";
import PlaylistList from "@/components/PlaylistList.vue";
import { useUserStore } from "@/store/user";
import { invoke } from "@tauri-apps/api/core";
import { onMounted } from "vue";
import { info } from "@tauri-apps/plugin-log";

const userStore = useUserStore();
const connectToSpotify = async () => {
  info("actual url:" + window.location.href);
  let url = await invoke("get_autorization_url_spotify");
  // window.location.href = url;
};

const connectToYoutube = async () => {
  info("actual url:" + window.location.href);
  let url = await invoke("connect_youtube");
  // window.location.href = url;
};

onMounted(() => {
  // Initial setup if needed
});
</script>

<style scoped>
.connect-button {
  padding: 10px 20px;
  background-color: #1db954;
  color: white;
  border: none;
  border-radius: 20px;
  cursor: pointer;
  font-weight: bold;
  margin: 20px 0;
}

.connect-button:hover {
  background-color: #1ed760;
}
</style>
