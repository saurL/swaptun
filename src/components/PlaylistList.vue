<template>
  <div class="space-y-3 flex flex-1 flex-col overflow-y-auto min-h-0">
    <PlaylistCard v-for="p in playlists" :key="p.id" :playlist="p" />
  </div>
</template>

<script setup>
import PlaylistCard from "./PlaylistCard.vue";

// Temporaire — test
const playlists = [
  { id: 1, name: "Vibes du moment", tracks: 24 },
  { id: 2, name: "Chill & Study", tracks: 37 },
  { id: 3, name: "Run & Boost", tracks: 19 },
  { id: 4, name: "Chill & Study", tracks: 37 },
  { id: 5, name: "Run & Boost", tracks: 19 },
  { id: 6, name: "Chill & Study", tracks: 37 },
  { id: 7, name: "Run & Boost", tracks: 19 },
];
</script>
