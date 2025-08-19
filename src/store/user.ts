import { defineStore } from 'pinia';
import Playlist from '@/models/playlist';
import User, { UserState } from '@/models/user';
import { invoke } from '@tauri-apps/api/core';
import { info } from '@tauri-apps/plugin-log';
export const useUserStore = defineStore('user', {
  state: (): UserState => ({
    id: null,
    token: null,
    username: null,
    information_loaded: false,
    youtubePlaylists: [],
    deezerPlaylists: [],
    spotifyPlaylists: [],
    friends: [],
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
    reset() {
      // Reset all state values to their defaults
      this.id = null;
      this.token = null;
      this.username = null;
      this.information_loaded = false;
      this.youtubePlaylists = [];
      this.deezerPlaylists = [];
      this.spotifyPlaylists = [];
      this.friends = [];
    },
    addFriend(user:User){
      invoke("add_friend", { request: {friend_id: user.id} })
        .then(() => {
          info("Friend added successfully");
          this.friends.push(user);
        })
        .catch((error) => {
          info("Error adding friend: " + error);
        });
    },
    removeFriend(user:User){
      invoke("remove_friend", { request: {friend_id: user.id} })
        .then(() => {
          info("Friend removed successfully");
          this.friends = this.friends.filter(friend => friend.id !== user.id);
        })
        .catch((error) => {
          info("Error removing friend: " + error);
        });
    }
  },
  getters: {
    allPlaylists: (state): Playlist[] => {
      return [...state.youtubePlaylists, ...state.deezerPlaylists, ...state.spotifyPlaylists];
    },
  },
  tauri: {
    saveOnChange: true,
  }

});
