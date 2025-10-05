import { invoke } from "@tauri-apps/api/core";
import { info } from "@tauri-apps/plugin-log";

export type Platform = "spotify" | "youtube" | "deezer" | "appleMusic";

export function usePlatformConnect() {
  const connectToSpotify = async () => {
    try {
      info("Connecting to Spotify...");
      await invoke("get_autorization_url_spotify");
    } catch (error) {
      info("Error connecting to Spotify: " + error);
      throw error;
    }
  };

  const connectToYoutube = async () => {
    try {
      info("Connecting to YouTube Music...");
      await invoke("connect_youtube");
    } catch (error) {
      info("Error connecting to YouTube: " + error);
      throw error;
    }
  };

  const connectToDeezer = async () => {
    try {
      info("Connecting to Deezer...");
      // TODO: Implement Deezer connection when available
      throw new Error("Deezer connection not yet implemented");
    } catch (error) {
      info("Error connecting to Deezer: " + error);
      throw error;
    }
  };

  const connectToAppleMusic = async () => {
    try {
      info("Connecting to Apple Music...");
      const response = await invoke("connect_apple_music");
      info("Apple Music authorization response: " + JSON.stringify(response));
      return response;
    } catch (error) {
      info("Error connecting to Apple Music: " + error);
      throw error;
    }
  };

  const connectToPlatform = async (platform: Platform) => {
    switch (platform) {
      case "spotify":
        return await connectToSpotify();
      case "youtube":
        return await connectToYoutube();
      case "deezer":
        return await connectToDeezer();
      case "appleMusic":
        return await connectToAppleMusic();
      default:
        throw new Error(`Unknown platform: ${platform}`);
    }
  };

  return {
    connectToSpotify,
    connectToYoutube,
    connectToDeezer,
    connectToAppleMusic,
    connectToPlatform,
  };
}
