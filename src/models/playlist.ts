import Music from "@/models/music"

export default interface Playlist {
  id: string;
  name: string;
  description?: string;
  artwork?: string;
  musics: Music[];
  source?: 'youtube' | 'deezer' | 'spotify' | "appleMusic" | null;

}

export interface PlaylistsResponse {
  vec: Playlist[];
  total: number;
}

export interface SharedBy {
  id: number;
  username: string;
}

export interface SharedPlaylist {
  id: number;
  playlist: Playlist;
  shared_by: SharedBy;
  shared_at: Date; // ISO 8601 UTC datetime
  viewed?: boolean; // Local state, not from backend
}

export interface SharedPlaylistResponse {
  shared_playlists: SharedPlaylist[];
}
