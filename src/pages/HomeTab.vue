<template>
  <div>
    <!-- Share Playlist Modal (nested route) -->
    <router-view />

    <!-- Global Loading State (when first loading all playlists) -->
    <div v-if="isInitialLoading" class="flex justify-center items-center py-20">
      <LoadingSpinner size="xl" />
    </div>

    <!-- Affichage conditionnel : bouton + OU playlists + autres sections -->
    <div v-else-if="hasPlaylists">
      <!-- Barre de recherche -->
      <div class="mb-4">
        <SearchInput
          v-model="searchQuery"
          placeholder="Search for a playlist..."
        />
      </div>

      <!-- Filter Chips (only show if multiple platforms available) -->
      <div
        v-if="availableFilters.length > 1"
        data-tour="playlists-navigation"
        class="flex gap-2 mb-6 flex-wrap"
      >
        <button
          v-for="filter in availableFilters"
          :key="filter.id"
          @click="toggleFilter(filter.id)"
          :class="[
            'flex items-center gap-2 px-4 py-2 rounded-full text-sm font-medium transition-all',
            activeFilters.includes(filter.id)
              ? 'bg-gradient-primary text-white shadow-button'
              : 'bg-background-secondary text-text-primary border border-secondary hover:border-primary',
          ]"
        >
          <img
            v-if="filter.icon"
            :src="filter.icon"
            :alt="filter.name"
            class="w-4 h-4"
          />
          <span>{{ filter.name }}</span>
          <span class="text-xs opacity-75">({{ filter.count }})</span>
        </button>
      </div>

      <!-- Toutes les playlists groupées -->
      <div data-tour="playlists-section" class="space-y-6">
        <PlaylistSection
          v-if="
            shouldShowPlatform('spotify') &&
            (filteredSpotifyPlaylists.length > 0 || appStore.isLoadingSpotify)
          "
          title="Spotify"
          :playlists="filteredSpotifyPlaylists"
          :loading="appStore.isLoadingSpotify"
          :error="spotifyError"
          :count="filteredSpotifyPlaylists.length"
          icon="/src/assets/images/spotify.svg"
          empty-message="No Spotify playlists. Connect to synchronize."
          @share="handleSharePlaylist"
        />

        <PlaylistSection
          v-if="
            shouldShowPlatform('appleMusic') &&
            (filteredApplePlaylists.length > 0 || appStore.isLoadingApple)
          "
          title="Apple Music"
          :playlists="filteredApplePlaylists"
          :loading="appStore.isLoadingApple"
          :error="appleError"
          :count="filteredApplePlaylists.length"
          icon="/src/assets/images/Apple_Music_icon.svg.png"
          empty-message="No Apple Music playlists."
          @share="handleSharePlaylist"
        />

        <PlaylistSection
          v-if="
            shouldShowPlatform('youtube') &&
            (filteredYoutubePlaylists.length > 0 || appStore.isLoadingYouTube)
          "
          title="YouTube Music"
          :playlists="filteredYoutubePlaylists"
          :loading="appStore.isLoadingYouTube"
          :error="youtubeError"
          :count="filteredYoutubePlaylists.length"
          icon="/src/assets/images/Youtube_Music_icon.svg.png"
          empty-message="No YouTube Music playlists."
          @share="handleSharePlaylist"
        />
      </div>
    </div>

    <!-- Vue vide : seulement le bouton + -->
    <div v-else>
      <PlatformSelector
        :platforms="platformsList"
        @connect="handlePlatformConnect"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from "vue";
import { useRouter } from "vue-router";
import { useUserStore } from "@/store/user";
import { useAppStore } from "@/store/app";
import { usePlatformConnect } from "@/composables/usePlatformConnect";
import { usePlaylistManagement } from "@/composables/usePlaylistManagement";
import { useFuzzySearch } from "@/composables/useFuzzySearch";
import SearchInput from "@/components/common/SearchInput.vue";
import LoadingSpinner from "@/components/common/LoadingSpinner.vue";
import PlatformSelector from "@/components/platforms/PlatformSelector.vue";
import PlaylistSection from "@/components/playlists/PlaylistSection.vue";
import type Playlist from "@/models/playlist";
import { PLATFORMS } from "@/utils/constants";
import type { Platform } from "@/composables/usePlatformConnect";

const router = useRouter();

const userStore = useUserStore();
const appStore = useAppStore();

const { connectToPlatform } = usePlatformConnect();
const { spotifyError, youtubeError, appleError } = usePlaylistManagement();

const searchQuery = ref("");
const activeFilters = ref<string[]>([]);

// Utilisation de la recherche floue pour chaque plateforme
const spotifyPlaylists = computed(() => userStore.spotifyPlaylists);
const youtubePlaylists = computed(() => userStore.youtubePlaylists);
const applePlaylists = computed(() => userStore.applePlaylists);

// Vérifie si l'utilisateur a des playlists
const hasPlaylists = computed(() => {
  console.log("Checking for playlists...");
  console.log("Spotify Playlists:", userStore.spotifyPlaylists);
  console.log("YouTube Playlists:", userStore.youtubePlaylists);
  console.log("Apple Music Playlists:", userStore.applePlaylists);
  return (
    (userStore.spotifyPlaylists?.length ?? 0) > 0 ||
    (userStore.youtubePlaylists?.length ?? 0) > 0 ||
    (userStore.applePlaylists?.length ?? 0) > 0
  );
});

// Track if we're doing the initial load (all platforms loading at once)
const isInitialLoading = computed(() => {
  const loadingCount = [
    appStore.isLoadingSpotify,
    appStore.isLoadingApple,
    appStore.isLoadingYouTube,
  ].filter(Boolean).length;

  const hasNoPlaylists = !hasPlaylists.value;

  // Show global loading if we're loading multiple platforms and have no playlists yet
  return loadingCount >= 2 && hasNoPlaylists;
});

// Available filters based on playlists that exist
const availableFilters = computed(() => {
  const filters = [];

  if ((spotifyPlaylists.value?.length ?? 0) > 0) {
    filters.push({
      id: "spotify",
      name: "Spotify",
      icon: "/src/assets/images/spotify.svg",
      count: spotifyPlaylists.value.length,
    });
  }

  if ((applePlaylists.value?.length ?? 0) > 0) {
    filters.push({
      id: "appleMusic",
      name: "Apple Music",
      icon: "/src/assets/images/Apple_Music_icon.svg.png",
      count: applePlaylists.value.length,
    });
  }

  if ((youtubePlaylists.value?.length ?? 0) > 0) {
    filters.push({
      id: "youtube",
      name: "YouTube Music",
      icon: "/src/assets/images/Youtube_Music_icon.svg.png",
      count: youtubePlaylists.value.length,
    });
  }

  return filters;
});

const toggleFilter = (filterId: string) => {
  const index = activeFilters.value.indexOf(filterId);
  if (index > -1) {
    activeFilters.value.splice(index, 1);
  } else {
    activeFilters.value.push(filterId);
  }
};

// Determine if a platform should be shown based on filters
const shouldShowPlatform = (platformId: string) => {
  // If no filters active, show all
  if (activeFilters.value.length === 0) return true;
  // If filters active, only show if platform is in active filters
  return activeFilters.value.includes(platformId);
};

const { filteredItems: filteredSpotifyPlaylists } = useFuzzySearch(
  spotifyPlaylists,
  searchQuery,
  (playlist) => playlist.playlist.name
);
const { filteredItems: filteredYoutubePlaylists } = useFuzzySearch(
  youtubePlaylists,
  searchQuery,
  (playlist) => playlist.playlist.name
);
const { filteredItems: filteredApplePlaylists } = useFuzzySearch(
  applePlaylists,
  searchQuery,
  (playlist) => playlist.playlist.name
);

// Liste des plateformes avec leur état de connexion
const platformsList = ref(
  PLATFORMS.map((p) => ({
    ...p,
    connected: false,
    loading: false,
  }))
);

const handlePlatformConnect = async (platformId: string) => {
  const platform = platformsList.value.find((p) => p.id === platformId);
  if (!platform) return;

  try {
    platform.loading = true;
    await connectToPlatform(platformId as Platform);
    platform.connected = true;
  } catch (error) {
    console.error(`Error connecting to ${platformId}:`, error);
  } finally {
    platform.loading = false;
  }
};

const handleSharePlaylist = (playlist: Playlist) => {
  router.push({
    name: "SharePlaylist",
    params: { playlistId: playlist.playlist.id },
  });
};
</script>
