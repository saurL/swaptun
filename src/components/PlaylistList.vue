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
          <p class="text-sm text-gray-400"></p>
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
          <p class="text-sm text-gray-400"></p>
          <p v-if="playlist.description" class="text-sm text-gray-500 mt-2">
            {{ playlist.description }}
          </p>
        </div>
      </div>
    </div>

    <!-- Playlists YouTube Music -->
    <div class="space-y-3">
      <h2 class="text-xl font-semibold text-white">Playlists YouTube Music</h2>
      <div v-if="isLoadingYoutubeMusic" class="flex justify-center">
        <div
          class="animate-spin rounded-full h-8 w-8 border-b-2 border-[#00CFE8]"
        ></div>
      </div>
      <div v-else-if="youtubeMusicError" class="text-red-500">
        {{ youtubeMusicError }}
      </div>
      <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        <div
          v-for="playlist in youtubeMusicPlaylists"
          :key="playlist.id"
          class="bg-[#1E1E1E] p-4 rounded-lg hover:bg-[#2A2A2A] transition-colors"
        >
          <h3 class="text-lg font-medium text-white">{{ playlist.name }}</h3>
          <p v-if="playlist.description" class="text-sm text-gray-500 mt-2">
            {{ playlist.description }}
          </p>
        </div>
      </div>
    </div>

    <!-- Playlists Apple Music -->
    <div class="space-y-3">
      <h2 class="text-xl font-semibold text-white">Playlists Apple Music</h2>
      <div v-if="isLoadingApple" class="flex justify-center">
        <div
          class="animate-spin rounded-full h-8 w-8 border-b-2 border-[#00CFE8]"
        ></div>
      </div>
      <div v-else-if="appleError" class="text-red-500">
        {{ appleError }}
      </div>
      <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        <div
          v-for="playlist in applePlaylists"
          :key="playlist.id"
          class="bg-[#1E1E1E] p-4 rounded-lg hover:bg-[#2A2A2A] transition-colors"
        >
          <h3 class="text-lg font-medium text-white">{{ playlist.name }}</h3>
          <p class="text-sm text-gray-400"></p>
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
import { useAppStore } from "@/store/app";
import { useUserStore } from "@/store/user";
import { storeToRefs } from "pinia";
import { PlaylistsResponse, GetPlaylistResponse, adaptPlaylistResponse } from "@/models/playlist";
import { info } from "@tauri-apps/plugin-log";

const appStore = useAppStore();
const userStore = useUserStore();

const {
  isLoadingYouTube: isLoadingYoutubeMusic,
  isLoadingDeezer,
  isLoadingSpotify,
  isLoadingApple,
} = storeToRefs(appStore);

const {
  youtubePlaylists: youtubeMusicPlaylists,
  deezerPlaylists,
  spotifyPlaylists,
  applePlaylists,
} = storeToRefs(userStore);
info(
  "UserStore state on PlaylistList mount: " + JSON.stringify(userStore.$state)
);
info(
  "AppStore state on PlaylistList mount: " + JSON.stringify(appStore.$state)
);
const spotifyError = ref<string | null>(null);
const deezerError = ref<string | null>(null);
const youtubeMusicError = ref<string | null>(null);
const appleError = ref<string | null>(null);

let unlistenYoutubeMusicPlaylists: (() => void) | null = null;
let unlistenApplePlaylists: (() => void) | null = null;

const setupYoutubeMusicPlaylistsListener = async () => {
  unlistenYoutubeMusicPlaylists = await listen<GetPlaylistResponse>(
    "youtubemusic_playlists",
    (event) => {
      userStore.setYoutubePlaylists(adaptPlaylistResponse(event.payload));
    }
  );
};

const setupApplePlaylistsListener = async () => {
  unlistenApplePlaylists = await listen<GetPlaylistResponse>(
    "apple_music_playlists",
    (event) => {
      userStore.setApplePlaylists(adaptPlaylistResponse(event.payload));
    }
  );
};

const fetchYoutubeMusicPlaylists = async () => {
  try {
    appStore.setLoading("youtube", true);
    youtubeMusicError.value = null;
    const response = await invoke<GetPlaylistResponse>(
      "get_playlists_youtubemusic"
    );
    userStore.setYoutubePlaylists(adaptPlaylistResponse(response));
  } catch (error) {
    youtubeMusicError.value = error as string;
    console.error("Error fetching YouTube Music playlists:", error);
  } finally {
    appStore.setLoading("youtube", false);
  }
};

const fetchApplePlaylists = async () => {
  try {
    appStore.setLoading("apple", true);
    appleError.value = null;
    const response = await invoke<GetPlaylistResponse>(
      "get_apple_music_playlists"
    );
    userStore.setApplePlaylists(adaptPlaylistResponse(response));
  } catch (error) {
    appleError.value = error as string;
    console.error("Error fetching Apple Music playlists:", error);
  } finally {
    appStore.setLoading("apple", false);
  }
};

let unlistenSpotifyPlaylists: (() => void) | null = null;
let unlistenDeezerPlaylists: (() => void) | null = null;

const setupSpotifyPlaylistsListener = async () => {
  unlistenSpotifyPlaylists = await listen<GetPlaylistResponse>(
    "spotify_playlists",
    (event) => {
      userStore.setSpotifyPlaylists(adaptPlaylistResponse(event.payload));
    }
  );
};

const setupDeezerPlaylistsListener = async () => {
  unlistenDeezerPlaylists = await listen<GetPlaylistResponse>(
    "deezer_playlists",
    (event) => {
      userStore.setDeezerPlaylists(adaptPlaylistResponse(event.payload));
    }
  );
};

const fetchSpotifyPlaylists = async () => {
  try {
    appStore.setLoading("spotify", true);
    spotifyError.value = null;
    const response = await invoke<GetPlaylistResponse>("get_playlists_spotify");
    userStore.setSpotifyPlaylists(adaptPlaylistResponse(response));
  } catch (error) {
    spotifyError.value = error as string;
    console.error("Error fetching Spotify playlists:", error);
  } finally {
    appStore.setLoading("spotify", false);
  }
};

const fetchDeezerPlaylists = async () => {
  try {
    appStore.setLoading("deezer", true);
    deezerError.value = null;
    const response = await invoke<GetPlaylistResponse>("get_playlists_deezer");
    userStore.setDeezerPlaylists(adaptPlaylistResponse(response));
  } catch (error) {
    deezerError.value = error as string;
    console.error("Error fetching Deezer playlists:", error);
  } finally {
    appStore.setLoading("deezer", false);
  }
};

onMounted(async () => {
  await Promise.all([
    fetchSpotifyPlaylists(),
    fetchDeezerPlaylists(),
    fetchYoutubeMusicPlaylists(),
    fetchApplePlaylists(),
    setupSpotifyPlaylistsListener(),
    setupDeezerPlaylistsListener(),
    setupYoutubeMusicPlaylistsListener(),
    setupApplePlaylistsListener(),
  ]);
});

onUnmounted(() => {
  if (unlistenSpotifyPlaylists) {
    unlistenSpotifyPlaylists();
  }
  if (unlistenDeezerPlaylists) {
    unlistenDeezerPlaylists();
  }
  if (unlistenYoutubeMusicPlaylists) {
    unlistenYoutubeMusicPlaylists();
  }
  // if (unlistenApplePlaylists) {
  //   unlistenApplePlaylists();
  // }
});
</script>
