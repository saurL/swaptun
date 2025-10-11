<template>
  <div class="flex flex-col items-center justify-center flex-1">
    <div class="space-y-4">
      <p class="text-text-primary text-lg mb-4">Select a music platform</p>
      <!-- Backdrop pour fermer le menu -->
      <div
        v-if="isOpen"
        @click="closeMenu"
        class="fixed top-0 left-0 right-0 bottom-0 z-50"
        style="margin: 0; padding: 0"
      ></div>

      <!-- Container pour le bouton + et le menu radial -->
      <div class="relative flex justify-center items-center h-64">
        <!-- Boutons des plateformes en cercle -->
        <div
          class="absolute inset-0 flex justify-center items-center pointer-events-none"
        >
          <button
            v-for="(platform, index) in platforms"
            :key="platform.id"
            :style="getRadialPosition(index, platforms.length, isOpen)"
            class="fixed w-16 h-16 rounded-full bg-white border-2 border-secondary flex items-center justify-center transition-all duration-500 ease-out hover:scale-110 hover:border-primary z-[60] pointer-events-auto shadow-lg tap-highlight-transparent"
            :class="{ 'opacity-0 scale-0': !isOpen }"
            :disabled="platform.loading || !isOpen"
            @click.stop="handlePlatformClick(platform.id)"
          >
            <img
              v-if="platform.icon"
              :src="platform.icon"
              :alt="platform.name"
              class="w-10 h-10 object-contain"
            />
            <div
              v-if="platform.loading"
              class="absolute inset-0 flex items-center justify-center bg-white/90 rounded-full"
            >
              <div
                class="w-6 h-6 border-2 border-primary border-t-transparent rounded-full animate-spin"
              ></div>
            </div>
          </button>
        </div>

        <!-- Bouton + central -->
        <button
          @click.stop="toggleMenu"
          data-tour="platform-button"
          class="relative z-30 w-20 h-20 rounded-full bg-gradient-to-br from-primary to-primary-light hover:from-primary-light hover:to-primary-lighter transition-all duration-300 flex items-center justify-center text-white text-4xl font-light shadow-lg"
          :class="{ 'rotate-45': isOpen }"
        >
          +
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useRouter } from "vue-router";
import { computed } from "vue";

export interface Platform {
  id: string;
  name: string;
  icon: string;
  connected?: boolean;
  loading?: boolean;
}

interface Props {
  platforms: Platform[];
}

defineProps<Props>();

const emit = defineEmits<{
  connect: [platformId: string];
}>();

const router = useRouter();

// Vérifie si le menu est ouvert via la route
const isOpen = computed(
  () => router.currentRoute.value.query.platforms === "open"
);

const toggleMenu = () => {
  if (isOpen.value) {
    router.back();
  } else {
    router.push({ query: { platforms: "open" } });
  }
};

const closeMenu = () => {
  if (isOpen.value) {
    router.back();
  }
};

const handlePlatformClick = (platformId: string) => {
  emit("connect", platformId);
  router.back();
};

const getRadialPosition = (index: number, total: number, open: boolean) => {
  const radius = open ? 100 : 0; // Animation depuis le centre
  const angle = (index * 2 * Math.PI) / total - Math.PI / 2; // Commence en haut
  const x = Math.cos(angle) * radius;
  const y = Math.sin(angle) * radius;

  return {
    transform: `translate(${x}px, ${y}px) scale(${open ? 1 : 0})`,
  };
};
</script>
