// Common interfaces for music and playlists
export default interface Music {
  id: string;
  title: string;
  artist: string;
  album: string;
  release_date: string;
  genre?: string | null;
}
