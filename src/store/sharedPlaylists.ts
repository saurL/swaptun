import { defineStore } from 'pinia';
import { invoke } from '@tauri-apps/api/core';
import { info, error as logError } from '@tauri-apps/plugin-log';
import Playlist, { SharedPlaylist, SharedPlaylistsResponse, SharedBy,  adaptSharedPlaylist } from '@/models/playlist';

export interface SharedPlaylistsState {
  playlists: SharedPlaylist[];
  loading: boolean;
  lastFetch: number | null;
}


export const useSharedPlaylistsStore = defineStore('sharedPlaylists', {
  state: (): SharedPlaylistsState => ({
    playlists: [],
    loading: false,
    lastFetch: null,
  }),

  actions: {
    async fetchSharedPlaylists(force = false) {
      // Don't refetch if we fetched less than 30 seconds ago (unless forced)
      const now = Date.now();
      if (!force && this.lastFetch && now - this.lastFetch < 30000) {
        info('Using cached shared playlists');
        return;
      }

      this.loading = true;

      try {
        const response = await invoke<SharedPlaylistsResponse>('get_shared_playlists');

        // Create a map of existing playlists to preserve local state (viewed status)
        const existingPlaylistsMap = new Map(
          this.playlists.map(p => [p.id, p])
        );

        // Adapt API response to frontend format and merge with local state
        this.playlists = response.shared_playlists.map((apiPlaylist) => {
          const adaptedPlaylist = adaptSharedPlaylist(apiPlaylist);
          const existing = existingPlaylistsMap.get(adaptedPlaylist.id);

          return {
            ...adaptedPlaylist,
            // Preserve local viewed state if it exists
            viewed: existing?.viewed ?? false,
          };
        });

        this.lastFetch = now;
        info(`Fetched ${response.shared_playlists.length} shared playlists`);
      } finally {
        this.loading = false;
      }
    },

    async addSharedPlaylistFromNotification(
      playlistWithMusics: Playlist,
      sharedBy: SharedBy
    ) {
      console.log("addSharedPlaylistFromNotification called with:", playlistWithMusics.playlist.name, sharedBy.username);

      // Convert PlaylistWithMusics to frontend Playlist format
      const playlist: Playlist = {
        playlist: {
        id: playlistWithMusics.playlist.id.toString(),
        user_id: playlistWithMusics.playlist.user_id.toString(),
        name: playlistWithMusics.playlist.name,
        description: playlistWithMusics.playlist.description,
        origin: playlistWithMusics.playlist.origin,
        origin_id: playlistWithMusics.playlist.origin_id,
        created_on: playlistWithMusics.playlist.created_on,
        updated_on: playlistWithMusics.playlist.updated_on,
        },
        musics: playlistWithMusics.musics || [],
      };

      const newSharedPlaylist: SharedPlaylist = {
        id: 0, // Temporary ID until we refetch
        playlist: playlist,
        shared_by: sharedBy,
        shared_at: new Date(),
        viewed: false,
      };

      // Add to beginning of array (most recent first)
      this.playlists.unshift(newSharedPlaylist);
      info(`Added new shared playlist: ${playlist.playlist.name} with ${playlist.musics.length} musics`);

      // Refetch in background to get accurate data with real ID
      this.fetchSharedPlaylists(true).catch((error) => {
        logError(`Failed to refetch after adding: ${error}`);
      });
    },

    // Legacy method for backwards compatibility
    async addSharedPlaylist(
      playlistId: string,
      playlistName: string,
      sharedById: number,
      sharedByUsername: string,
    ) {
      console.log("addSharedPlaylist (legacy) called with:", playlistId, playlistName, sharedById, sharedByUsername);
      // Add a new shared playlist to the list (from notification)
      let playlist: Playlist = {
        playlist: {
        id: playlistId,
        user_id: '', // Unknown
        name: playlistName,
        description: '',
        origin: '',
        origin_id: '',
        created_on: new Date().toISOString(),
        updated_on: new Date().toISOString(),
        },
        musics: [],

      };
      const newSharedPlaylist: SharedPlaylist = {
        id: 0, // Temporary ID until we refetch
        playlist: playlist,
        shared_by: {
          id: sharedById,
          username: sharedByUsername,
        },
        shared_at: new Date(),
        viewed: false,
      };

      // Add to beginning of array (most recent first)
      this.playlists.unshift(newSharedPlaylist);
      info(`Added new shared playlist: ${playlistName}`);

      // Refetch in background to get accurate data
      this.fetchSharedPlaylists(true).catch((error) => {
        logError(`Failed to refetch after adding: ${error}`);
      });
    },

    async markAsViewed(sharedPlaylistId: number) {
      try {
        await invoke('mark_shared_playlist_viewed', { sharedPlaylistId });

        // Update local state
        const playlist = this.playlists.find((p) => p.id === sharedPlaylistId);
        if (playlist) {
          playlist.viewed = true;
          info(`Marked playlist ${sharedPlaylistId} as viewed`);
        }
      } catch (error) {
        logError(`Failed to mark playlist as viewed: ${error}`);
        throw error;
      }
    },

    markAllAsViewed() {
      // Mark all playlists as viewed in local state
      this.playlists.forEach((p) => {
        p.viewed = true;
      });
    },

    reset() {
      this.playlists = [];
      this.loading = false;
      this.lastFetch = null;
    },
  },

  getters: {
    unviewedCount: (state): number => {
      return state.playlists.filter((p) => !p.viewed).length;
    },

    sortedPlaylists: (state): SharedPlaylist[] => {
      return [...state.playlists].sort((a, b) => {
        // Unviewed first
        if (a.viewed !== b.viewed) {
          return a.viewed ? 1 : -1;
        }
        // Then by date (most recent first)
        return new Date(b.shared_at).getTime() - new Date(a.shared_at).getTime();
      });
    },
  },

  tauri: {
    saveOnChange: true,
  },
});
