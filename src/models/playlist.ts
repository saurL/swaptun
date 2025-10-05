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
  first_name: string;
  last_name: string;
}

export interface SharedPlaylist {
  id: number;
  playlist_id: string;
  playlist_name: string;
  shared_by: SharedBy;
  shared_at: string; // ISO 8601 UTC datetime
  viewed?: boolean; // Local state, not from backend
}

export interface SharedPlaylistsResponse {
  vec: SharedPlaylist[];
  total: number;
}
