import Music from "@/models/music"

// Frontend Playlist structure (simplified)
export default interface Playlist {
  playlist: {
  id: string;
  user_id: string;
  name: string;
  description: string | null;
  origin: string;
  origin_id: string;
  created_on: string;
  updated_on: string;
  }

  musics: Music[];
}



export interface GetPlaylistResponse {
  playlists: Playlist[];
}

// Legacy structure (for backwards compatibility)
export interface PlaylistsResponse {
  playlists: Playlist[];
}

// Adapter function to convert new API format to frontend format
export function adaptPlaylistResponse(response: GetPlaylistResponse): Playlist[] {
  return response.playlists.map((item) => ({
    playlist: {
    id: item.playlist.id.toString(),
    user_id: item.playlist.user_id.toString(),
    name: item.playlist.name,
    description: item.playlist.description,
    origin: item.playlist.origin,
    origin_id: item.playlist.origin_id,
    created_on: item.playlist.created_on,
    updated_on: item.playlist.updated_on,
    },
    musics: item.musics || [],
  }));
}

export interface SharedBy {
  id: number;
  username: string;
}

// Backend API structure for shared playlists
export interface SharedPlaylistAPI {
  id: number;
  playlist: Playlist;  // Contains playlist + optional musics
  shared_by: SharedBy;
  shared_at: string; // ISO 8601 UTC datetime
}

// Frontend structure for shared playlists (with adapted Playlist)
export interface SharedPlaylist {
  id: number;
  playlist: Playlist;
  shared_by: SharedBy;
  shared_at: Date; // ISO 8601 UTC datetime
  viewed?: boolean; // Local state, not from backend
}

export interface SharedPlaylistsResponse {
  shared_playlists: SharedPlaylistAPI[];
}

// Legacy
export interface SharedPlaylistResponse {
  shared_playlists: SharedPlaylist[];
}

// Adapter function to convert shared playlist API format to frontend format
export function adaptSharedPlaylist(apiPlaylist: SharedPlaylistAPI): SharedPlaylist {
  return {
    id: apiPlaylist.id,
    playlist: {
      playlist: {
        id: apiPlaylist.id.toString(),
      user_id: apiPlaylist.playlist.playlist.user_id.toString(),
      name: apiPlaylist.playlist.playlist.name,
      description: apiPlaylist.playlist.playlist.description,
      origin: apiPlaylist.playlist.playlist.origin,
      origin_id: apiPlaylist.playlist.playlist.origin_id,
      created_on: apiPlaylist.playlist.playlist.created_on,
      updated_on: apiPlaylist.playlist.playlist.updated_on,},
      musics: apiPlaylist.playlist.musics || [],
    },
    shared_by: apiPlaylist.shared_by,
    shared_at: new Date(apiPlaylist.shared_at),
    viewed: false,
  };
}
