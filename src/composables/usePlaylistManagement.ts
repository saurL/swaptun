import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { useUserStore } from "@/store/user";
import { useAppStore } from "@/store/app";
import type Playlist from "@/models/playlist";

interface PlaylistsResponse {
  vec: Playlist[];
  total: number;
}

export function usePlaylistManagement() {
  const userStore = useUserStore();
  const appStore = useAppStore();

  const spotifyError = ref<string | null>(null);
  const deezerError = ref<string | null>(null);
  const youtubeError = ref<string | null>(null);
  const appleError = ref<string | null>(null);

  let unlisteners: Array<() => void> = [];

  // Fetch functions
  const fetchSpotifyPlaylists = async () => {
    try {
      appStore.setLoading("spotify", true);
      spotifyError.value = null;
      const response = await invoke<PlaylistsResponse>("get_playlists_spotify");
      userStore.setSpotifyPlaylists(response.vec);
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
      const response = await invoke<PlaylistsResponse>("get_playlists_deezer");
      userStore.setDeezerPlaylists(response.vec);
    } catch (error) {
      deezerError.value = error as string;
      console.error("Error fetching Deezer playlists:", error);
    } finally {
      appStore.setLoading("deezer", false);
    }
  };

  const fetchYoutubePlaylists = async () => {
    try {
      appStore.setLoading("youtube", true);
      youtubeError.value = null;
      const response = await invoke<PlaylistsResponse>("get_playlists_youtubemusic");
      userStore.setYoutubePlaylists(response.vec);
    } catch (error) {
      youtubeError.value = error as string;
      console.error("Error fetching YouTube Music playlists:", error);
    } finally {
      appStore.setLoading("youtube", false);
    }
  };

  const fetchApplePlaylists = async () => {
    try {
      appStore.setLoading("apple", true);
      appleError.value = null;
      const response = await invoke<PlaylistsResponse>("get_apple_music_playlists");
      userStore.setApplePlaylists(response.vec);
    } catch (error) {
      appleError.value = error as string;
      console.error("Error fetching Apple Music playlists:", error);
    } finally {
      appStore.setLoading("apple", false);
    }
  };

  // Event listeners setup
  const setupPlaylistListeners = async () => {
    const spotifyUnlisten = await listen<PlaylistsResponse>("spotify_playlists", (event) => {
      userStore.setSpotifyPlaylists(event.payload.vec);
    });

    const deezerUnlisten = await listen<PlaylistsResponse>("deezer_playlists", (event) => {
      userStore.setDeezerPlaylists(event.payload.vec);
    });

    const youtubeUnlisten = await listen<PlaylistsResponse>("youtubemusic_playlists", (event) => {
      userStore.setYoutubePlaylists(event.payload.vec);
    });

    const appleUnlisten = await listen<PlaylistsResponse>("apple_music_playlists", (event) => {
      userStore.setApplePlaylists(event.payload.vec);
    });

    unlisteners = [spotifyUnlisten, deezerUnlisten, youtubeUnlisten, appleUnlisten];
  };

  const fetchAllPlaylists = async () => {
    await Promise.all([
      fetchSpotifyPlaylists(),
      fetchDeezerPlaylists(),
      fetchYoutubePlaylists(),
      fetchApplePlaylists(),
    ]);
  };

  const sendPlaylist = async (playlistId: number, destination: string) => {
    try {
      await invoke("send_playlist", {
        playlistId,
        req: { destination },
      });
    } catch (error) {
      console.error("Error sending playlist:", error);
      throw error;
    }
  };

  const cleanup = () => {
    unlisteners.forEach((unlisten) => unlisten());
    unlisteners = [];
  };

  return {
    // State
    spotifyError,
    deezerError,
    youtubeError,
    appleError,
    // Methods
    fetchSpotifyPlaylists,
    fetchDeezerPlaylists,
    fetchYoutubePlaylists,
    fetchApplePlaylists,
    fetchAllPlaylists,
    setupPlaylistListeners,
    sendPlaylist,
    cleanup,
  };
}
