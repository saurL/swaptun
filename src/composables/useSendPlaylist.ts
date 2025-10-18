import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useUserStore } from "@/store/user";
import { useAppStore } from "@/store/app";

export type Platform = "Spotify" | "YoutubeMusic" | "AppleMusic" | "Deezer";

export interface ConnectedPlatform {
  name: Platform;
  label: string;
  icon: string;
}

export interface SendPlaylistResponse {
  platform: Platform;
  playlist_id: string;
}

export function useSendPlaylist() {
  const userStore = useUserStore();
  const appStore = useAppStore();
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

  const sendPlaylistToPlatform = async (
    playlistId: number,
    destination: Platform,
    platformInfo?: ConnectedPlatform
  ): Promise<SendPlaylistResponse | null> => {
    try {
      sending.value = true;
      error.value = null;
      appStore.setSendingPlaylist(true);

      const response = await invoke<SendPlaylistResponse>("send_playlist", {
        playlistId,
        req: { destination },
      });

      // Keep loading state for a moment to show success
      await new Promise(resolve => setTimeout(resolve, 500));

      // Set success state if platform info is provided
      if (platformInfo) {
        appStore.setPlaylistSendSuccess({
          platformLabel: platformInfo.label,
          platformIcon: platformInfo.icon,
          platformName: platformInfo.name,
          playlistId: response.playlist_id,
        });
      }

      return response;
    } catch (err) {
      error.value = err as string;
      console.error("Error sending playlist:", err);
      return null;
    } finally {
      sending.value = false;
      appStore.setSendingPlaylist(false);
    }
  };

  const sendToDefaultPlatform = async (playlistId: number): Promise<SendPlaylistResponse | null> => {
    if (!hasSinglePlatform.value) {
      error.value = "Cannot send to default platform - multiple platforms connected";
      return null;
    }

    const platform = connectedPlatforms.value[0];
    return sendPlaylistToPlatform(playlistId, platform.name, platform);
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
