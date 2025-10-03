import Playlist from "@/models/playlist";

export default interface User {
  id: number;
  username: string;
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
  applePlaylists: Playlist[];
  friends: User[];
}
