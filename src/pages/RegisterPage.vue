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

        <!-- Loading State -->
        <Transition name="fade" mode="out-in">
          <div v-if="isSubmitting" key="loading" class="py-12 text-center space-y-4">
            <Transition name="scale" mode="out-in">
              <div v-if="!registrationSuccess" key="spinner" class="flex justify-center">
                <div class="animate-spin rounded-full h-16 w-16 border-4 border-primary border-t-transparent"></div>
              </div>
              <div v-else key="success" class="flex justify-center">
                <div class="flex items-center justify-center h-16 w-16 rounded-full bg-success animate-bounce-once">
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-10 w-10 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="3">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7" />
                  </svg>
                </div>
              </div>
            </Transition>
            <Transition name="slide-up" mode="out-in">
              <div v-if="!registrationSuccess" key="creating" class="space-y-2">
                <p class="text-lg font-medium text-text-primary">Creating your account...</p>
              </div>
              <div v-else key="success-msg" class="space-y-2">
                <p class="text-lg font-medium text-success">Account created successfully!</p>
                <p class="text-sm text-text-secondary">Redirecting to login...</p>
              </div>
            </Transition>
          </div>

          <!-- Registration Form -->
          <div v-else key="form">
          <!-- Messages d'erreur -->
          <div
            v-if="errorMessage"
            class="mb-4 p-3 rounded-lg bg-error/10 border border-error/20"
          >
            <p class="text-error text-sm text-center">{{ errorMessage }}</p>
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
              class="w-full"
            >
              Sign Up
            </Button>
          </form>
        </div>
        </Transition>

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
import { useHaptics } from "@/composables/useHaptics";
import Card from "@/components/common/Card.vue";
import Button from "@/components/common/Button.vue";

const router = useRouter();
const haptics = useHaptics();

const form = reactive({
  firstName: "",
  lastName: "",
  username: "",
  email: "",
  password: "",
});

const errorMessage = ref("");
const isSubmitting = ref(false);
const registrationSuccess = ref(false);

async function handleSubmit() {
  try {
    isSubmitting.value = true;
    registrationSuccess.value = false;
    errorMessage.value = "";

    await invoke("register", {
      firstName: form.firstName,
      lastName: form.lastName,
      username: form.username,
      email: form.email,
      password: form.password,
    });

    info("User registered successfully");

    // Show success state
    registrationSuccess.value = true;
    await haptics.success();

    // Wait a moment before redirecting to show the success message
    setTimeout(() => {
      router.replace("/login");
    }, 1500);
  } catch (error) {
    console.error("Registration error:", error);
    errorMessage.value = "An error occurred during registration";
    await haptics.error();
    isSubmitting.value = false;
    registrationSuccess.value = false;
  }
}
</script>

<style scoped>
/* Fade transition */
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

/* Scale transition */
.scale-enter-active,
.scale-leave-active {
  transition: all 0.3s ease;
}

.scale-enter-from {
  opacity: 0;
  transform: scale(0.8);
}

.scale-leave-to {
  opacity: 0;
  transform: scale(1.2);
}

/* Slide up transition */
.slide-up-enter-active,
.slide-up-leave-active {
  transition: all 0.3s ease;
}

.slide-up-enter-from {
  opacity: 0;
  transform: translateY(10px);
}

.slide-up-leave-to {
  opacity: 0;
  transform: translateY(-10px);
}

/* Bounce once animation */
@keyframes bounce-once {
  0%, 100% {
    transform: scale(1);
  }
  50% {
    transform: scale(1.2);
  }
}

.animate-bounce-once {
  animation: bounce-once 0.5s ease;
}
</style>
