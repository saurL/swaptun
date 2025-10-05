<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useRoute, useRouter } from "vue-router";
import { invoke } from "@tauri-apps/api/core";
import Card from "@/components/common/Card.vue";
import Button from "@/components/common/Button.vue";

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
    errorMessage.value = "Please enter a new password";
    return;
  }

  if (password.value.length < 6) {
    errorMessage.value = "Password must contain at least 6 characters";
    return;
  }

  if (password.value !== confirmPassword.value) {
    errorMessage.value = "Passwords do not match";
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
        successMessage.value = "Your password has been reset successfully.";
        password.value = ""; // Clear the input field
        confirmPassword.value = ""; // Clear the confirm field
      } else {
        errorMessage.value = "An error occurred while resetting the password.";
      }
      goToHome();
    })
    .catch((error: any) => {
      isSubmitting.value = false;
      errorMessage.value = `Error: ${typeof error}`;
    });
};

const goToHome = () => {
  router.push("/");
};
</script>

<template>
  <div class="min-h-screen bg-background flex items-center justify-center p-4">
    <!-- Lien vers la page de connexion -->
    <router-link
      to="/login"
      class="absolute top-4 left-4 text-primary hover:text-primary-light text-sm font-medium transition-all"
    >
      ← Back to sign in
    </router-link>

    <div class="w-full max-w-md">
      <Card variant="white" padding="lg">
        <!-- LOGO -->
        <div class="flex justify-center mb-6">
          <img
            src="/src/assets/images/icon.svg"
            alt="Swaply Logo"
            class="h-16 w-16"
          />
        </div>

        <h1 class="text-2xl font-bold text-center text-text-primary mb-4">
          Reset password
        </h1>
        <p class="text-sm text-text-secondary text-center mb-8">
          Enter your new password below
        </p>

        <form class="space-y-5">
          <div>
            <label class="block text-sm font-medium text-text-primary mb-2"
              >New password</label
            >
            <input
              type="password"
              v-model="password"
              class="w-full px-4 py-3 rounded-xl bg-background-secondary text-text-primary placeholder-text-secondary border border-secondary focus:outline-none focus:ring-2 focus:ring-primary focus:border-primary focus:shadow-glow transition-all"
              placeholder="••••••"
            />
          </div>

          <div>
            <label class="block text-sm font-medium text-text-primary mb-2"
              >Confirm password</label
            >
            <input
              type="password"
              v-model="confirmPassword"
              class="w-full px-4 py-3 rounded-xl bg-background-secondary text-text-primary placeholder-text-secondary border border-secondary focus:outline-none focus:ring-2 focus:ring-primary focus:border-primary focus:shadow-glow transition-all"
              placeholder="••••••"
            />
          </div>

          <div
            v-if="errorMessage"
            class="p-3 rounded-lg bg-error/10 border border-error/20"
          >
            <p class="text-error text-sm text-center">{{ errorMessage }}</p>
          </div>

          <div
            v-if="successMessage"
            class="p-3 rounded-lg bg-success/10 border border-success/20"
          >
            <p class="text-success text-sm text-center">{{ successMessage }}</p>
          </div>

          <Button
            type="submit"
            variant="primary"
            size="lg"
            :loading="isSubmitting"
            class="w-full"
            @click="handleResetPassword"
          >
            Reset password
          </Button>
        </form>

        <p class="text-center text-sm text-text-secondary mt-6">
          Remember your password?
          <router-link
            to="/login"
            class="text-primary hover:text-primary-light hover:underline font-medium"
            >Sign in</router-link
          >
        </p>
      </Card>
    </div>
  </div>
</template>

<style scoped></style>
