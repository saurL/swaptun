<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useRoute, useRouter } from "vue-router";
import { invoke } from "@tauri-apps/api/core";

const route = useRoute();
const router = useRouter();

const password = ref("");
const confirmPassword = ref("");
const errorMessage = ref("");
const successMessage = ref("");
const isSubmitting = ref(false);
const token = ref("");

onMounted(() => {
  // Get token from URL query parameter
  token.value = (route.query.token as string) || "";

  // If no token, redirect to forgot password page
  if (token.value == "") {
    router.replace("/");
  }
});

const handleResetPassword = async () => {
  if (!password.value) {
    errorMessage.value = "Veuillez entrer un nouveau mot de passe";
    return;
  }

  if (password.value.length < 6) {
    errorMessage.value = "Le mot de passe doit contenir au moins 6 caractères";
    return;
  }

  if (password.value !== confirmPassword.value) {
    errorMessage.value = "Les mots de passe ne correspondent pas";
    return;
  }

  isSubmitting.value = true;
  errorMessage.value = "";
  successMessage.value = "";

  invoke<boolean>("reset_password", {
    token: token.value,
    request: { password: password.value },
  })
    .then((response: boolean) => {
      isSubmitting.value = false;
      if (response) {
        successMessage.value =
          "Votre mot de passe a été réinitialisé avec succès.";
        password.value = ""; // Clear the input field
        confirmPassword.value = ""; // Clear the confirm field
      } else {
        errorMessage.value =
          "Une erreur est survenue lors de la réinitialisation du mot de passe.";
      }
      goToHome();
    })
    .catch((error: any) => {
      isSubmitting.value = false;
      errorMessage.value = `Erreur: ${typeof error}`;
    });
};

const goToHome = () => {
  router.push("/");
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
      Réinitialiser le mot de passe
    </h1>
    <p class="text-sm text-gray-300 text-center mb-8">
      Entrez votre nouveau mot de passe ci-dessous
    </p>

    <form class="space-y-5">
      <div>
        <label class="block text-sm font-medium text-gray-300"
          >Nouveau mot de passe</label
        >
        <input
          type="password"
          v-model="password"
          class="w-full mt-1 px-4 py-2 rounded-lg bg-[#2A2A2A] text-white placeholder-gray-500 border border-gray-600 focus:outline-none focus:ring-2 focus:ring-[#00CFE8]"
          placeholder="••••••"
        />
      </div>

      <div>
        <label class="block text-sm font-medium text-gray-300"
          >Confirmer le mot de passe</label
        >
        <input
          type="password"
          v-model="confirmPassword"
          class="w-full mt-1 px-4 py-2 rounded-lg bg-[#2A2A2A] text-white placeholder-gray-500 border border-gray-600 focus:outline-none focus:ring-2 focus:ring-[#00CFE8]"
          placeholder="••••••"
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
        @click.prevent="handleResetPassword"
        :disabled="isSubmitting"
      >
        {{
          isSubmitting
            ? "Réinitialisation en cours..."
            : "Réinitialiser le mot de passe"
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
