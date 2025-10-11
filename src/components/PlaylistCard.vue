<template>
  <div
    class="bg-[#1E1E1E] p-4 rounded-xl border border-white/10 flex justify-between items-center cursor-pointer hover:border-white/20 transition-all active:scale-95"
    @click="handleCardClick"
  >
    <div>
      <h3 class="text-white font-semibold">{{ playlist.name }}</h3>
      <p class="text-gray-400 text-sm">{{ playlist.tracks }} titres</p>
    </div>
    <button
      class="text-[#00CFE8] hover:text-[#FFC436] transition active:scale-90"
      @click.stop="handleSendClick"
    >
      <i class="lucide lucide-send"></i>
    </button>
  </div>
</template>

<script setup lang="ts">
import { useHaptics } from '@/composables/useHaptics'

interface Playlist {
  name: string
  tracks: number
}

const props = defineProps<{
  playlist: Playlist
}>()

const emit = defineEmits<{
  click: []
  send: []
}>()

const haptics = useHaptics()

const handleCardClick = async () => {
  await haptics.light()
  emit('click')
}

const handleSendClick = async () => {
  await haptics.medium()
  emit('send')
}
</script>
