<template>
  <div class="bg-[#1E1E1E] p-4 rounded-xl border border-white/10 flex justify-between items-center">
    <div>
      <h3 class="text-white font-semibold">{{ playlist.name }}</h3>
      <p class="text-gray-400 text-sm">{{ playlist.tracks }} titres</p>
    </div>
    <button class="text-[#00CFE8] hover:text-[#FFC436] transition">
      <i class="lucide lucide-send"></i>
    </button>
  </div>
</template>

<script setup>
const props = defineProps({
  playlist: Object
})
</script>
