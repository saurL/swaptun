<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const greetMsg = ref('');
const name = ref('');

/**
 * Invoke the `greet` command with the user-provided name and update `greetMsg` with the response.
 */
async function greet() {
  greetMsg.value = await invoke('greet', { name: name.value });
}
</script>

<template>
  <section class="flex flex-col items-center gap-6 py-10">
    <form class="flex flex-wrap gap-3" @submit.prevent="greet">
      <input
        id="greet-input"
        v-model="name"
        placeholder="Enter a name..."
        aria-label="Enter your name"
        class="px-4 py-2 border border-gray-300 bg-red-50 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-400"
      />
      <button 
        type="submit"
        class="px-4 py-2 border border-blue-500 shadow-sm rounded-md !bg-blue-500 text-white hover:bg-blue-700 transition"
      >
        Greet
      </button>
    </form>
    <p class="text-lg font-semibold">{{ greetMsg }}</p>
  </section>
</template>


