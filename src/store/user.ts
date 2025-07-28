import { defineStore } from 'pinia';

// Common interfaces for music and playlists
export interface Music {
  id: string;
  title: string;
  artist: string;
  album: string; 
}

export interface Playlist {
  id: string;
  name: string;
  description?: string;
  artwork?: string;
  musics: Music[];
  source: 'youtube' | 'deezer' | 'spotify';

}

// Interface for the user store state
export interface UserState {
  id: number | null;
  token: string | null;
  information_loaded: boolean;
  username: string | null;
  youtubePlaylists: Playlist[];
  deezerPlaylists: Playlist[];
  spotifyPlaylists: Playlist[];
}

export const useUserStore = defineStore('user', {
  state: (): UserState => ({
    id: null,
    token: null,
    username: null,
    information_loaded: false,
    youtubePlaylists: [],
    deezerPlaylists: [],
    spotifyPlaylists: [],
  }),
  actions: {
    setUserInfo(id: number, username: string) {
      this.id = id;
      this.username = username;
      this.information_loaded = true;
    },
    setToken(token: string) {
      this.token = token;
    },
    setYoutubePlaylists(playlists: Playlist[]) {
      this.youtubePlaylists = playlists;
    },
    setDeezerPlaylists(playlists: Playlist[]) {
      this.deezerPlaylists = playlists;
    },
    setSpotifyPlaylists(playlists: Playlist[]) {
      this.spotifyPlaylists = playlists;
    },
  },
  getters: {
    allPlaylists: (state): Playlist[] => {
      return [...state.youtubePlaylists, ...state.deezerPlaylists, ...state.spotifyPlaylists];
    },
  },tauri:{
    autoStart: true,
    allowSave: true,
  }

});

