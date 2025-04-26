<script lang="ts" setup>
import { reactive } from "vue";
import { invoke } from "@tauri-apps/api/core";
const form = reactive({
  firstName: "",
  lastName: "",
  username: "",
  email: "",
  password: "",
});

async function handleSubmit() {
  try {
    const response = await invoke("register", {
      firstName: form.firstName,
      lastName: form.lastName,
      username: form.username,
      email: form.email,
      password: form.password,
    });
    console.log("Registration successful:", response);
    // Handle success (e.g., redirect or show a success message)
  } catch (error) {
    console.error("Registration failed:", error);
    // Handle error (e.g., show an error message)
  }
}
</script>

<template>
  <div
    class="min-h-screen w-screen bg-[#121212] text-white flex flex-col items-center justify-center"
  >
    <form
      @submit.prevent="handleSubmit"
      class="bg-[#1e1e1e] p-6 rounded-lg shadow-md w-full max-w-md"
    >
      <div class="mb-4">
        <label for="firstName" class="block text-sm font-medium mb-1"
          >Prénom:</label
        >
        <input
          type="text"
          id="firstName"
          v-model="form.firstName"
          required
          class="w-full px-3 py-2 bg-[#2a2a2a] text-white rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
      </div>
      <div class="mb-4">
        <label for="lastName" class="block text-sm font-medium mb-1"
          >Nom:</label
        >
        <input
          type="text"
          id="lastName"
          v-model="form.lastName"
          required
          class="w-full px-3 py-2 bg-[#2a2a2a] text-white rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
      </div>
      <div class="mb-4">
        <label for="username" class="block text-sm font-medium mb-1"
          >Nom d'utilisateur:</label
        >
        <input
          type="text"
          id="username"
          v-model="form.username"
          required
          class="w-full px-3 py-2 bg-[#2a2a2a] text-white rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
      </div>
      <div class="mb-4">
        <label for="email" class="block text-sm font-medium mb-1"
          >Adresse mail:</label
        >
        <input
          type="email"
          id="email"
          v-model="form.email"
          required
          class="w-full px-3 py-2 bg-[#2a2a2a] text-white rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
      </div>
      <div class="mb-6">
        <label for="password" class="block text-sm font-medium mb-1"
          >Mot de passe:</label
        >
        <input
          type="password"
          id="password"
          v-model="form.password"
          required
          class="w-full px-3 py-2 bg-[#2a2a2a] text-white rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
      </div>
      <button
        type="submit"
        class="w-full bg-blue-500 hover:bg-blue-600 text-white font-medium py-2 px-4 rounded-md transition"
      >
        S'inscrire
      </button>
    </form>
  </div>
</template>

<style scoped>
/* Ajoutez des styles spécifiques si nécessaire */
</style>
