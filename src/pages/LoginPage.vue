<script setup lang="ts">
import { useStore } from "@/store/token";
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { info } from "@tauri-apps/plugin-log";
import { save } from "@tauri-store/pinia";

//import { useRouter } from "vue-router";
const store = useStore();
//const router = useRouter();
const email = ref("");
const password = ref("");
const errorMessage = ref("");
interface LoginResponse {
  token: string;
}

const handleLogin = async () => {
  invoke("login_email", {
    email: email.value,
    password: password.value,
  })
    .then(async (response: any) => {
      const loginResponse = response as LoginResponse;
      info(`Login successful, token: ${loginResponse.token}`);
      store.setIdentificationToken(loginResponse.token);
      await save(store);
      info("Store saved");
    })
    .catch((error) => {
      errorMessage.value = error.message;
    });
};
//router.push("/spotify-auth");
</script>

<template>
  <!-- Lien vers la homepage -->
  <a
    href="/homepage"
    class="absolute top-4 left-4 text-[#00CFE8] hover:text-[#FFC436] text-sm font-medium transition-all"
  >
    Lien vers l'accueil
  </a>
  <div
    class="bg-white/10 backdrop-blur-md shadow-lg rounded-2xl w-full max-w-md p-8 border border-white/10"
  >
    <!-- LOGO -->
    <div class="flex justify-center mb-6">
      <img
        src="/src/assets/images/icon.ico"
        alt="Swaply Logo"
        class="h-16 w-16"
      />
    </div>

    <h1 class="text-2xl font-bold text-center text-white mb-4">
      Bienvenue sur Swaply
    </h1>
    <p class="text-sm text-gray-300 text-center mb-8">
      Partage ta vibe avec tes amis.
    </p>

    <form class="space-y-5">
      <div>
        <label class="block text-sm font-medium text-gray-300">Email</label>
        <input
          type="email"
          v-model="email"
          class="w-full mt-1 px-4 py-2 rounded-lg bg-[#2A2A2A] text-white placeholder-gray-500 border border-gray-600 focus:outline-none focus:ring-2 focus:ring-[#00CFE8]"
          placeholder="melomane@swaptun.app"
        />
      </div>
      <div>
        <label class="block text-sm font-medium text-gray-300"
          >Mot de passe</label
        >
        <input
          type="password"
          v-model="password"
          class="w-full mt-1 px-4 py-2 rounded-lg bg-[#2A2A2A] text-white placeholder-gray-500 border border-gray-600 focus:outline-none focus:ring-2 focus:ring-[#00CFE8]"
          placeholder="••••••••"
        />
      </div>
      <button
        type="submit"
        class="w-full bg-[#00CFE8] hover:bg-[#FFC436] text-[#1E1E1E] font-semibold py-2 px-4 rounded-lg transition"
        @click.prevent="handleLogin"
      >
        Se connecter
      </button>
    </form>

    <p class="text-center text-sm text-gray-400 mt-6">
      Pas encore de compte ?
      <a
        href="/register"
        class="text-[#00CFE8] hover:text-[#FFC436] hover:underline"
        >Créer un compte</a
      >
    </p>
  </div>
</template>

<style scoped></style>
