import { defineStore } from 'pinia';
import { invoke } from '@tauri-apps/api/core';
import { info, error as logError } from '@tauri-apps/plugin-log';
import { SharedPlaylist, SharedPlaylistResponse } from '@/models/playlist';
import { utcToLocal } from '@/utils/helpers';

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

      try {
        this.loading = true;
        const response = await invoke<SharedPlaylistResponse>('get_shared_playlists');
        this.playlists = response.vec.map((sharedPlaylist) => ({
          ...sharedPlaylist,
          viewed: sharedPlaylist.viewed ?? false,
        }));
        this.lastFetch = now;
        info(`Fetched ${response.vec.length} shared playlists`);
      } catch (error) {
        logError(`Failed to fetch shared playlists: ${error}`);
        throw error;
      } finally {
        this.loading = false;
      }
    },

    async addSharedPlaylist(
      playlistId: string,
      playlistName: string,
      sharedById: number,
      sharedByUsername: string,
      sharedByName: string
    ) {
      // Add a new shared playlist to the list (from notification)
      const [firstName, ...lastNameParts] = sharedByName.split(' ');
      const newSharedPlaylist: SharedPlaylist = {
        id: 0, // Temporary ID until we refetch
        playlist_id: playlistId,
        playlist_name: playlistName,
        shared_by: {
          id: sharedById,
          username: sharedByUsername,
          first_name: firstName,
          last_name: lastNameParts.join(' '),
        },
        shared_at: new Date().toISOString(),
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
