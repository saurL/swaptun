import { defineStore } from 'pinia';
export interface AppState {
  isAppReady: boolean;
  isLoadingYouTube: boolean;
  isLoadingDeezer: boolean;
  isLoadingSpotify: boolean;
  isLoadingApple: boolean;
}

export const useAppStore = defineStore('app', {
  state: (): AppState => ({
    isAppReady: false,
    isLoadingYouTube: false,
    isLoadingDeezer: false,
    isLoadingSpotify: false,
    isLoadingApple: false,
  }),
  actions: {
    setAppReady(isReady: boolean) {
      this.isAppReady = isReady;
    },
    setLoading(service: 'youtube' | 'deezer' | 'spotify' | 'apple', isLoading: boolean) {
      if (service === 'youtube') this.isLoadingYouTube = isLoading;
      if (service === 'deezer') this.isLoadingDeezer = isLoading;
      if (service === 'spotify') this.isLoadingSpotify = isLoading;
      if (service === 'apple') this.isLoadingApple = isLoading;
    },
  },
});