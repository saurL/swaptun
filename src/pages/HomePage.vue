<template>
  <MainLayout>
    <GreetingCard userName="Léna" />
    <PlaylistList />
  </MainLayout>
</template>

<script setup>
import MainLayout from '../layouts/MainLayout.vue'
import GreetingCard from '../components/GreetingCard.vue'
import PlaylistList from '../components/PlaylistList.vue'
</script>
