<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
const email = ref("");
const errorMessage = ref("");
const successMessage = ref("");
const isSubmitting = ref(false);

const handleForgotPassword = async () => {
  if (!email.value) {
    errorMessage.value = "Veuillez entrer votre adresse email";
    return;
  }

  isSubmitting.value = true;
  errorMessage.value = "";
  successMessage.value = "";
  invoke<boolean>("forgot_password", { request: { email: email.value } })
    .then((response: boolean) => {
      isSubmitting.value = false;
      if (response) {
        successMessage.value =
          "Un lien de réinitialisation a été envoyé à votre adresse email.";
        email.value = ""; // Clear the input field
      } else {
        errorMessage.value =
          "Une erreur est survenue lors de l'envoi du lien de réinitialisation.";
      }
    })
    .catch((error: any) => {
      isSubmitting.value = false;
      errorMessage.value = `Erreur: ${error.message}`;
    });
};
</script>

<template>
  <!-- Lien vers la page de connexion -->
  <a
    href="/login"
    class="absolute top-4 left-4 text-[#00CFE8] hover:text-[#FFC436] text-sm font-medium transition-all"
  >
    ← Retour à la connexion
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
      Mot de passe oublié ?
    </h1>
    <p class="text-sm text-gray-300 text-center mb-8">
      Entrez votre adresse email pour recevoir un lien de réinitialisation
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

      <div v-if="errorMessage" class="text-red-400 text-sm text-center">
        {{ errorMessage }}
      </div>

      <div v-if="successMessage" class="text-green-400 text-sm text-center">
        {{ successMessage }}
      </div>

      <button
        type="submit"
        class="w-full bg-[#00CFE8] hover:bg-[#FFC436] text-[#1E1E1E] font-semibold py-2 px-4 rounded-lg transition"
        @click.prevent="handleForgotPassword"
        :disabled="isSubmitting"
      >
        {{
          isSubmitting
            ? "Envoi en cours..."
            : "Envoyer le lien de réinitialisation"
        }}
      </button>
    </form>

    <p class="text-center text-sm text-gray-400 mt-6">
      Vous vous souvenez de votre mot de passe ?
      <a
        href="/login"
        class="text-[#00CFE8] hover:text-[#FFC436] hover:underline"
        >Se connecter</a
      >
    </p>
  </div>
</template>

<style scoped></style>
