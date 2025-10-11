<template>
  <div class="space-y-6">
    <p></p>
    <!-- Informations utilisateur -->
    <div class="flex items-center gap-4 mb-6">
      <div
        class="w-20 h-20 rounded-full bg-gradient-to-br from-primary to-primary-lighter flex items-center justify-center text-white text-3xl font-bold"
      >
        {{ userStore.username?.charAt(0).toUpperCase() }}
      </div>
      <div>
        <h2 class="text-2xl font-bold text-text-primary">
          {{ userStore.username }}
        </h2>
        <p class="text-text-secondary">{{ userStore.email }}</p>
      </div>
    </div>

    <!-- Statistiques -->
    <div class="grid grid-cols-2 gap-4">
      <div
        class="bg-background-secondary rounded-lg p-4 text-center border border-secondary"
      >
        <div class="text-3xl font-bold text-primary">
          {{ totalPlaylists }}
        </div>
        <div class="text-text-secondary text-sm mt-1">Playlists</div>
      </div>
      <div
        class="bg-background-secondary rounded-lg p-4 text-center border border-secondary"
      >
        <div class="text-3xl font-bold text-primary-light">
          {{ userStore.friends.length }}
        </div>
        <div class="text-text-secondary text-sm mt-1">Friends</div>
      </div>
    </div>

    <!-- Platforms -->
    <h3 class="text-xl font-bold text-text-primary mb-4">Platforms</h3>
    <div class="space-y-3">
      <div
        v-for="platform in platforms"
        :key="platform.id"
        class="flex items-center justify-between p-3 bg-background-secondary rounded-lg border transition-all"
        :class="platform.connected ? 'border-success/30' : 'border-secondary'"
      >
        <div class="flex items-center gap-3">
          <img
            v-if="platform.icon"
            :src="platform.icon"
            :alt="platform.name"
            class="w-8 h-8"
          />
          <span class="text-text-primary font-medium">{{ platform.name }}</span>
        </div>

        <!-- Connected state - show disconnect button -->
        <Button
          v-if="platform.connected"
          variant="outline"
          size="sm"
          @click="handleDisconnectPlatform(platform.id)"
        >
          Disconnect
        </Button>

        <!-- Not connected - show connect button -->
        <Button
          v-else
          variant="primary"
          size="sm"
          @click="handleConnectPlatform(platform.id)"
        >
          Connect
        </Button>
      </div>
    </div>

    <!-- Bouton de déconnexion -->
    <div class="flex justify-center">
      <Button variant="outline" @click="handleLogout"> Sign out </Button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { useRouter } from "vue-router";
import { invoke } from "@tauri-apps/api/core";
import { useUserStore } from "@/store/user";
import {
  usePlatformConnect,
  type Platform,
} from "@/composables/usePlatformConnect";
import { PLATFORMS } from "@/utils/constants";
import Card from "@/components/common/Card.vue";
import Button from "@/components/common/Button.vue";

const router = useRouter();
const userStore = useUserStore();
const { connectToPlatform } = usePlatformConnect();

// Compute platforms with connection status
const platforms = computed(() => {
  return PLATFORMS.map((platform) => {
    let connected = false;

    switch (platform.id) {
      case "spotify":
        connected = userStore.spotifyPlaylists.length > 0;
        break;
      case "youtube":
        connected = userStore.youtubePlaylists.length > 0;
        break;
      case "appleMusic":
        connected = userStore.applePlaylists.length > 0;
        break;
    }

    return {
      ...platform,
      connected,
    };
  });
});

const totalPlaylists = computed(() => {
  return (
    userStore.spotifyPlaylists.length +
    userStore.youtubePlaylists.length +
    userStore.applePlaylists.length
  );
});

const handleConnectPlatform = async (platformId: string) => {
  try {
    await connectToPlatform(platformId as Platform);
  } catch (error) {
    console.error(`Error connecting to ${platformId}:`, error);
  }
};

const handleDisconnectPlatform = async (platformId: string) => {
  try {
    // Call Tauri command to disconnect the platform
    const commands: Record<string, string> = {
      spotify: "disconnect_spotify",
      youtube: "disconnect_youtube",
      appleMusic: "disconnect_apple_music",
    };

    const command = commands[platformId];
    if (!command) {
      console.error(`Unknown platform: ${platformId}`);
      return;
    }

    // Call Tauri command
    await invoke(command);

    // Clear the playlists for the disconnected platform locally
    switch (platformId) {
      case "spotify":
        userStore.spotifyPlaylists = [];
        break;
      case "youtube":
        userStore.youtubePlaylists = [];
        break;
      case "appleMusic":
        userStore.applePlaylists = [];
        break;
    }
  } catch (error) {
    console.error(`Error disconnecting from ${platformId}:`, error);
    // Optionally show error to user
  }
};

const handleLogout = () => {
  userStore.reset();
  router.replace("/login");
};
</script>
