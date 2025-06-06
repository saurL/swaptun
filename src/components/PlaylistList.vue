<template>
  <div class="space-y-6">
    <!-- Playlists Spotify -->
    <div class="space-y-3">
      <h2 class="text-xl font-semibold text-white">Playlists Spotify</h2>
      <div v-if="isLoadingSpotify" class="flex justify-center">
        <div
          class="animate-spin rounded-full h-8 w-8 border-b-2 border-[#00CFE8]"
        ></div>
      </div>
      <div v-else-if="spotifyError" class="text-red-500">
        {{ spotifyError }}
      </div>
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        <div
          v-for="playlist in spotifyPlaylists"
          :key="playlist.id"
          class="bg-[#1E1E1E] p-4 rounded-lg hover:bg-[#2A2A2A] transition-colors"
        >
          <h3 class="text-lg font-medium text-white">{{ playlist.name }}</h3>
          <p class="text-sm text-gray-400">
            {{ playlist.tracks_count }} titres
          </p>
          <p v-if="playlist.description" class="text-sm text-gray-500 mt-2">
            {{ playlist.description }}
          </p>
        </div>
      </div>
    </div>

    <!-- Playlists Deezer -->
    <div class="space-y-3">
      <h2 class="text-xl font-semibold text-white">Playlists Deezer</h2>
      <div v-if="isLoadingDeezer" class="flex justify-center">
        <div
          class="animate-spin rounded-full h-8 w-8 border-b-2 border-[#00CFE8]"
        ></div>
      </div>
      <div v-else-if="deezerError" class="text-red-500">
        {{ deezerError }}
      </div>
      <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        <div
          v-for="playlist in deezerPlaylists"
          :key="playlist.id"
          class="bg-[#1E1E1E] p-4 rounded-lg hover:bg-[#2A2A2A] transition-colors"
        >
          <h3 class="text-lg font-medium text-white">{{ playlist.name }}</h3>
          <p class="text-sm text-gray-400">
            {{ playlist.tracks_count }} titres
          </p>
          <p v-if="playlist.description" class="text-sm text-gray-500 mt-2">
            {{ playlist.description }}
          </p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

interface Playlist {
  id: string;
  name: string;
  description?: string;
  tracks_count: number;
  owner_id: string;
  created_at: string;
  updated_at: string;
}

interface PlaylistsResponse {
  vec: Playlist[];
  total: number;
}

const spotifyPlaylists = ref<Playlist[]>([]);
const deezerPlaylists = ref<Playlist[]>([]);
const isLoadingSpotify = ref(true);
const isLoadingDeezer = ref(true);
const spotifyError = ref<string | null>(null);
const deezerError = ref<string | null>(null);

let unlistenSpotifyPlaylists: (() => void) | null = null;
let unlistenDeezerPlaylists: (() => void) | null = null;

const setupSpotifyPlaylistsListener = async () => {
  unlistenSpotifyPlaylists = await listen<PlaylistsResponse>(
    "spotify_playlists",
    (event) => {
      spotifyPlaylists.value = event.payload.vec;
    }
  );
};

const setupDeezerPlaylistsListener = async () => {
  unlistenDeezerPlaylists = await listen<PlaylistsResponse>(
    "deezer_playlists",
    (event) => {
      deezerPlaylists.value = event.payload.vec;
    }
  );
};

const fetchSpotifyPlaylists = async () => {
  try {
    isLoadingSpotify.value = true;
    spotifyError.value = null;
    const response = await invoke<PlaylistsResponse>("get_playlists_spotify");
    spotifyPlaylists.value = response.vec;
  } catch (error) {
    spotifyError.value = error as string;
    console.error("Error fetching Spotify playlists:", error);
  } finally {
    isLoadingSpotify.value = false;
  }
};

const fetchDeezerPlaylists = async () => {
  try {
    isLoadingDeezer.value = true;
    deezerError.value = null;
    const response = await invoke<PlaylistsResponse>("get_playlists_deezer");
    deezerPlaylists.value = response.vec;
  } catch (error) {
    deezerError.value = error as string;
    console.error("Error fetching Deezer playlists:", error);
  } finally {
    isLoadingDeezer.value = false;
  }
};

onMounted(async () => {
  await Promise.all([
    fetchSpotifyPlaylists(),
    fetchDeezerPlaylists(),
    setupSpotifyPlaylistsListener(),
    setupDeezerPlaylistsListener(),
  ]);
});

onUnmounted(() => {
  if (unlistenSpotifyPlaylists) {
    unlistenSpotifyPlaylists();
  }
  if (unlistenDeezerPlaylists) {
    unlistenDeezerPlaylists();
  }
});
</script>
