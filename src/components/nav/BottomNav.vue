<template>
  <nav
    class="fixed bottom-0 left-0 right-0 z-40 bg-white border-t border-secondary shadow-lg pb-safe"
  >
    <div class="w-full">
      <div class="flex items-center justify-around px-6 py-3 max-w-7xl mx-auto">
        <!-- Accueil -->
        <button
          @click="navigateTo('HomeTab')"
          class="relative group transition-all duration-300"
          :class="isActive('HomeTab') ? 'scale-110' : 'scale-100'"
        >
          <div
            class="p-3.5 rounded-2xl transition-all duration-300"
            :class="
              isActive('HomeTab')
                ? 'bg-primary shadow-lg shadow-primary/30'
                : 'bg-transparent group-hover:bg-secondary'
            "
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              class="h-6 w-6 transition-all duration-300"
              :class="
                isActive('HomeTab')
                  ? 'text-white'
                  : 'text-text-secondary group-hover:text-primary group-hover:scale-110'
              "
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
              stroke-width="2"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"
              />
            </svg>
          </div>
        </button>

        <!-- Shared Playlists -->
        <button
          @click="navigateTo('SharedPlaylistsTab')"
          class="relative group transition-all duration-300"
          :class="isActive('SharedPlaylistsTab') ? 'scale-110' : 'scale-100'"
        >
          <!-- Badge for unviewed playlists -->
          <div
            v-if="unviewedCount > 0"
            class="absolute -top-1 -right-1 z-50 bg-[#CB5520] text-white text-xs font-bold rounded-full min-w-[20px] h-5 flex items-center justify-center px-1.5 shadow-lg"
          >
            {{ unviewedCount > 9 ? '9+' : unviewedCount }}
          </div>

          <div
            class="p-3.5 rounded-2xl transition-all duration-300"
            :class="
              isActive('SharedPlaylistsTab')
                ? 'bg-primary-light shadow-lg shadow-primary-light/30'
                : 'bg-transparent group-hover:bg-secondary'
            "
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              class="h-6 w-6 transition-all duration-300"
              :class="
                isActive('SharedPlaylistsTab')
                  ? 'text-white'
                  : 'text-text-secondary group-hover:text-primary-light group-hover:scale-110'
              "
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
              stroke-width="2"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M9 19V6l12-3v13M9 19c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zm12-3c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zM9 10l12-3"
              />
            </svg>
          </div>
        </button>

        <!-- Amis -->
        <button
          @click="navigateTo('FriendsTab')"
          class="relative group transition-all duration-300"
          :class="isActive('FriendsTab') ? 'scale-110' : 'scale-100'"
        >
          <div
            class="p-3.5 rounded-2xl transition-all duration-300"
            :class="
              isActive('FriendsTab')
                ? 'bg-primary-light shadow-lg shadow-primary-light/30'
                : 'bg-transparent group-hover:bg-secondary'
            "
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              class="h-6 w-6 transition-all duration-300"
              :class="
                isActive('FriendsTab')
                  ? 'text-white'
                  : 'text-text-secondary group-hover:text-primary-light group-hover:scale-110'
              "
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
              stroke-width="2"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z"
              />
            </svg>
          </div>
        </button>

        <!-- Profil -->
        <button
          @click="navigateTo('ProfileTab')"
          class="relative group transition-all duration-300"
          :class="isActive('ProfileTab') ? 'scale-110' : 'scale-100'"
        >
          <div
            class="p-3.5 rounded-2xl transition-all duration-300"
            :class="
              isActive('ProfileTab')
                ? 'bg-primary-lighter shadow-lg shadow-primary-lighter/30'
                : 'bg-transparent group-hover:bg-secondary'
            "
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              class="h-6 w-6 transition-all duration-300"
              :class="
                isActive('ProfileTab')
                  ? 'text-white'
                  : 'text-text-secondary group-hover:text-primary-lighter group-hover:scale-110'
              "
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
              stroke-width="2"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"
              />
            </svg>
          </div>
        </button>
      </div>
    </div>
  </nav>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { useRoute, useRouter } from "vue-router";
import { storeToRefs } from "pinia";
import { useSharedPlaylistsStore } from "@/store/sharedPlaylists";

const route = useRoute();
const router = useRouter();
const sharedPlaylistsStore = useSharedPlaylistsStore();

const { unviewedCount } = storeToRefs(sharedPlaylistsStore);

const isActive = (routeName: string) => {
  return computed(() => route.name === routeName).value;
};

const navigateTo = (routeName: string) => {
  // Utilise push pour créer l'historique de navigation
  router.push({ name: routeName });
};
</script>
