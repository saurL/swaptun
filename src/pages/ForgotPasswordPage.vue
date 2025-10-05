<template>
  <div class="min-h-screen bg-background flex items-center justify-center p-4">
    <div class="w-full max-w-md">
      <Card variant="white" padding="lg">
        <!-- LOGO -->
        <div class="flex justify-center mb-6">
          <img
            src="/src/assets/images/icon.svg"
            alt="Swaply Logo"
            class="h-20 w-20"
          />
        </div>

        <h1 class="text-3xl font-bold text-center text-text-primary mb-2">
          Forgot password?
        </h1>
        <p class="text-sm text-text-secondary text-center mb-8">
          Enter your email address to receive a reset link
        </p>

        <!-- Messages -->
        <div
          v-if="errorMessage"
          class="mb-4 p-3 rounded-lg bg-error/10 border border-error/20"
        >
          <p class="text-error text-sm text-center">{{ errorMessage }}</p>
        </div>

        <div
          v-if="successMessage"
          class="mb-4 p-3 rounded-lg bg-success/10 border border-success/20"
        >
          <p class="text-success text-sm text-center">{{ successMessage }}</p>
        </div>

        <form @submit.prevent="handleForgotPassword" class="space-y-5">
          <div>
            <label class="block text-sm font-medium text-text-primary mb-2"
              >Email</label
            >
            <input
              type="email"
              v-model="email"
              required
              class="w-full px-4 py-3 rounded-xl bg-background-secondary text-text-primary placeholder-text-secondary border border-secondary focus:outline-none focus:ring-2 focus:ring-primary focus:border-primary focus:shadow-glow transition-all"
              placeholder="melomane@swaptun.app"
            />
          </div>

          <Button
            type="submit"
            variant="primary"
            size="lg"
            :loading="isSubmitting"
            class="w-full"
          >
            Send link
          </Button>
        </form>

        <div class="mt-8 pt-6 border-t border-secondary">
          <p class="text-center text-sm text-text-secondary">
            Remember your password?
            <router-link
              to="/login"
              class="text-primary hover:text-primary-light hover:underline font-medium transition-colors"
            >
              Sign in
            </router-link>
          </p>
        </div>
      </Card>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import Card from "@/components/common/Card.vue";
import Button from "@/components/common/Button.vue";

const email = ref("");
const errorMessage = ref("");
const successMessage = ref("");
const isSubmitting = ref(false);

const handleForgotPassword = async () => {
  if (!email.value) {
    errorMessage.value = "Please enter your email address";
    return;
  }

  try {
    isSubmitting.value = true;
    errorMessage.value = "";
    successMessage.value = "";

    const response = await invoke<boolean>("forgot_password", {
      request: { email: email.value },
    });

    if (response) {
      successMessage.value =
        "A reset link has been sent to your email address.";
      email.value = "";
    } else {
      errorMessage.value = "An error occurred while sending the reset link.";
    }
  } catch (error: any) {
    errorMessage.value = `Error: ${error.message || "An error occurred"}`;
  } finally {
    isSubmitting.value = false;
  }
};
</script>
