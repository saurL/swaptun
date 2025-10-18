import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

export interface Music {
  title: string;
  artist: string;
  album: string;
  release_date: string;
  genre: string | null;
}

export interface GetPlaylistMusicsResponse {
  playlist_id: number;
  musics: Music[];
}

export function usePlaylistMusics() {
  const loading = ref(false);
  const error = ref<string | null>(null);
  const musicsCache = ref<Record<number, Music[]>>({});

  const getPlaylistMusics = async (playlistId: number): Promise<Music[]> => {
    // Check cache first
    if (musicsCache.value[playlistId]) {
      return musicsCache.value[playlistId];
    }

    try {
      loading.value = true;
      error.value = null;

      const response = await invoke<GetPlaylistMusicsResponse>(
        "get_playlist_musics",
        { playlistId }
      );

      // Store in cache
      musicsCache.value[playlistId] = response.musics;

      return response.musics;
    } catch (err) {
      error.value = err as string;
      console.error("Error fetching playlist musics:", err);
      return [];
    } finally {
      loading.value = false;
    }
  };

  const clearCache = (playlistId?: number) => {
    if (playlistId !== undefined) {
      delete musicsCache.value[playlistId];
    } else {
      musicsCache.value = {};
    }
  };

  return {
    loading,
    error,
    musicsCache,
    getPlaylistMusics,
    clearCache,
  };
}
