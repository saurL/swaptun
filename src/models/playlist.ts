import Music from "@/models/music"

export default interface Playlist {
  id: string;
  name: string;
  description?: string;
  artwork?: string;
  musics: Music[];
  source: 'youtube' | 'deezer' | 'spotify';

}

