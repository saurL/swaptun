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
          Create account
        </h1>
        <p class="text-sm text-text-secondary text-center mb-8">
          Join the Swaply community
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

        <form @submit.prevent="handleSubmit" class="space-y-4">
          <div class="grid grid-cols-2 gap-3">
            <div>
              <label class="block text-sm font-medium text-text-primary mb-2"
                >First name</label
              >
              <input
                type="text"
                v-model="form.firstName"
                required
                class="w-full px-4 py-3 rounded-xl bg-background-secondary text-text-primary placeholder-text-secondary border border-secondary focus:outline-none focus:ring-2 focus:ring-primary focus:border-primary focus:shadow-glow transition-all"
                placeholder="John"
              />
            </div>

            <div>
              <label class="block text-sm font-medium text-text-primary mb-2"
                >Last name</label
              >
              <input
                type="text"
                v-model="form.lastName"
                required
                class="w-full px-4 py-3 rounded-xl bg-background-secondary text-text-primary placeholder-text-secondary border border-secondary focus:outline-none focus:ring-2 focus:ring-primary focus:border-primary focus:shadow-glow transition-all"
                placeholder="Doe"
              />
            </div>
          </div>

          <div>
            <label class="block text-sm font-medium text-text-primary mb-2"
              >Username</label
            >
            <input
              type="text"
              v-model="form.username"
              required
              class="w-full px-4 py-3 rounded-xl bg-background-secondary text-text-primary placeholder-text-secondary border border-secondary focus:outline-none focus:ring-2 focus:ring-primary focus:border-primary focus:shadow-glow transition-all"
              placeholder="johndoe"
            />
          </div>

          <div>
            <label class="block text-sm font-medium text-text-primary mb-2"
              >Email</label
            >
            <input
              type="email"
              v-model="form.email"
              required
              class="w-full px-4 py-3 rounded-xl bg-background-secondary text-text-primary placeholder-text-secondary border border-secondary focus:outline-none focus:ring-2 focus:ring-primary focus:border-primary focus:shadow-glow transition-all"
              placeholder="john@example.com"
            />
          </div>

          <div>
            <label class="block text-sm font-medium text-text-primary mb-2"
              >Password</label
            >
            <input
              type="password"
              v-model="form.password"
              required
              minlength="8"
              class="w-full px-4 py-3 rounded-xl bg-background-secondary text-text-primary placeholder-text-secondary border border-secondary focus:outline-none focus:ring-2 focus:ring-primary focus:border-primary focus:shadow-glow transition-all"
              placeholder="••••••••"
            />
            <p class="text-xs text-text-secondary mt-1">Minimum 8 characters</p>
          </div>

          <Button
            type="submit"
            variant="primary"
            size="lg"
            :loading="isSubmitting"
            class="w-full"
          >
            Sign Up
          </Button>
        </form>

        <div class="mt-8 pt-6 border-t border-secondary">
          <p class="text-center text-sm text-text-secondary">
            Already have an account?
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
import { reactive, ref } from "vue";
import { useRouter } from "vue-router";
import { invoke } from "@tauri-apps/api/core";
import { info } from "@tauri-apps/plugin-log";
import Card from "@/components/common/Card.vue";
import Button from "@/components/common/Button.vue";

const router = useRouter();

const form = reactive({
  firstName: "",
  lastName: "",
  username: "",
  email: "",
  password: "",
});

const errorMessage = ref("");
const successMessage = ref("");
const isSubmitting = ref(false);

async function handleSubmit() {
  try {
    isSubmitting.value = true;
    errorMessage.value = "";
    successMessage.value = "";

    await invoke("register", {
      firstName: form.firstName,
      lastName: form.lastName,
      username: form.username,
      email: form.email,
      password: form.password,
    });

    successMessage.value = "Registration successful! Redirecting...";
    info("User registered successfully");

    setTimeout(() => {
      router.replace("/login");
    }, 2000);
  } catch (error) {
    console.error("Registration error:", error);
    errorMessage.value = "An error occurred during registration";
  } finally {
    isSubmitting.value = false;
  }
}
</script>
