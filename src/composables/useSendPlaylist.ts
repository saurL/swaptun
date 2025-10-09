import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useUserStore } from "@/store/user";

export type Platform = "Spotify" | "YoutubeMusic" | "AppleMusic" | "Deezer";

export interface ConnectedPlatform {
  name: Platform;
  label: string;
  icon: string;
}

export function useSendPlaylist() {
  const userStore = useUserStore();
  const sending = ref(false);
  const error = ref<string | null>(null);

  // Detect connected platforms based on whether user has playlists from that platform
  const connectedPlatforms = computed<ConnectedPlatform[]>(() => {
    const platforms: ConnectedPlatform[] = [];

    if (userStore.spotifyPlaylists.length > 0) {
      platforms.push({
        name: "Spotify",
        label: "Spotify",
        icon: "/src/assets/images/spotify.svg",
      });
    }

    if (userStore.youtubePlaylists.length > 0) {
      platforms.push({
        name: "YoutubeMusic",
        label: "YouTube Music",
        icon: "/src/assets/images/Youtube_Music_icon.svg.png",
      });
    }

    if (userStore.applePlaylists.length > 0) {
      platforms.push({
        name: "AppleMusic",
        label: "Apple Music",
        icon: "/src/assets/images/Apple_Music_icon.svg.png",
      });
    }

    // Deezer is not yet implemented for sending
    // if (userStore.deezerPlaylists.length > 0) {
    //   platforms.push({
    //     name: "Deezer",
    //     label: "Deezer",
    //     icon: "/src/assets/images/deezer.svg",
    //   });
    // }

    return platforms;
  });

  const hasConnectedPlatforms = computed(() => connectedPlatforms.value.length > 0);
  const hasSinglePlatform = computed(() => connectedPlatforms.value.length === 1);
  const hasMultiplePlatforms = computed(() => connectedPlatforms.value.length > 1);

  const sendPlaylistToPlatform = async (playlistId: number, destination: Platform): Promise<boolean> => {
    try {
      sending.value = true;
      error.value = null;

      await invoke("send_playlist", {
        playlistId,
        req: { destination },
      });

      // Keep loading state for a moment to show success
      await new Promise(resolve => setTimeout(resolve, 500));

      return true;
    } catch (err) {
      error.value = err as string;
      console.error("Error sending playlist:", err);
      return false;
    } finally {
      sending.value = false;
    }
  };

  const sendToDefaultPlatform = async (playlistId: number): Promise<boolean> => {
    if (!hasSinglePlatform.value) {
      error.value = "Cannot send to default platform - multiple platforms connected";
      return false;
    }

    const platform = connectedPlatforms.value[0];
    return sendPlaylistToPlatform(playlistId, platform.name);
  };

  return {
    // State
    sending,
    error,
    connectedPlatforms,
    hasConnectedPlatforms,
    hasSinglePlatform,
    hasMultiplePlatforms,
    // Methods
    sendPlaylistToPlatform,
    sendToDefaultPlatform,
  };
}
