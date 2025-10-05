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
          Welcome to Swaply
        </h1>
        <p class="text-sm text-text-secondary text-center mb-8">
          Share your vibe with your friends.
        </p>

        <!-- Messages d'erreur -->
        <div
          v-if="errorMessage"
          class="mb-4 p-3 rounded-lg bg-error/10 border border-error/20"
        >
          <p class="text-error text-sm text-center">{{ errorMessage }}</p>
        </div>

        <form @submit.prevent="handleLogin" class="space-y-5">
          <div>
            <label class="block text-sm font-medium text-text-primary mb-2"
              >Email</label
            >
            <input
              type="email"
              v-model="email"
              required
              class="w-full px-4 py-3 rounded-xl bg-background-secondary text-text-primary placeholder-text-secondary border border-secondary focus:outline-none focus:ring-2 focus:ring-primary focus:border-primary focus:shadow-glow transition-all"
              placeholder="your@email.com"
            />
          </div>

          <div>
            <label class="block text-sm font-medium text-text-primary mb-2"
              >Password</label
            >
            <input
              type="password"
              v-model="password"
              required
              class="w-full px-4 py-3 rounded-xl bg-background-secondary text-text-primary placeholder-text-secondary border border-secondary focus:outline-none focus:ring-2 focus:ring-primary focus:border-primary focus:shadow-glow transition-all"
              placeholder="••••••••"
            />
          </div>

          <Button
            type="submit"
            variant="primary"
            size="lg"
            :loading="isLoading"
            class="w-full"
          >
            Sign In
          </Button>

          <div class="text-center">
            <router-link
              to="/forgot-password"
              class="text-sm text-primary hover:text-primary-light hover:underline transition-colors"
            >
              Forgot password?
            </router-link>
          </div>
        </form>

        <div class="mt-8 pt-6 border-t border-secondary">
          <p class="text-center text-sm text-text-secondary">
            Don't have an account yet?
            <router-link
              to="/register"
              class="text-primary hover:text-primary-light hover:underline font-medium transition-colors"
            >
              Create account
            </router-link>
          </p>
        </div>
      </Card>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { useRouter } from "vue-router";
import { invoke } from "@tauri-apps/api/core";
import { info, error as logError } from "@tauri-apps/plugin-log";
import { useUserStore } from "@/store/user";
import Card from "@/components/common/Card.vue";
import Button from "@/components/common/Button.vue";
import type LoginResponse from "@/models/dto";

const router = useRouter();
const store = useUserStore();

const email = ref("");
const password = ref("");
const errorMessage = ref("");
const isLoading = ref(false);

const handleLogin = async () => {
  if (!email.value || !password.value) {
    errorMessage.value = "Please fill in all fields";
    return;
  }

  try {
    isLoading.value = true;
    errorMessage.value = "";

    const response = await invoke<LoginResponse>("login_email", {
      request: {
        email: email.value,
        password: password.value,
      },
    });

    store.setToken(response.token);
    store.setUserInfo(response.user.id, response.user.username);

    info("User logged in successfully");
    router.replace("/");
  } catch (err) {
    logError("Login error: " + err);
    errorMessage.value = "Incorrect email or password";
  } finally {
    isLoading.value = false;
  }
};
</script>
