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

        <!-- Loading State -->
        <Transition name="fade" mode="out-in">
          <div v-if="isLoading" key="loading" class="py-12 text-center space-y-4">
            <Transition name="scale" mode="out-in">
              <div v-if="!loginSuccess" key="spinner" class="flex justify-center">
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
              <div v-if="!loginSuccess" key="connecting" class="space-y-2">
                <p class="text-lg font-medium text-text-primary">Connecting...</p>
              </div>
              <div v-else key="success-msg" class="space-y-2">
                <p class="text-lg font-medium text-success">Connected successfully!</p>
                <p class="text-sm text-text-secondary">You will be redirected shortly</p>
              </div>
            </Transition>
          </div>

          <!-- Login Form -->
          <div v-else key="form">
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
        </div>
        </Transition>

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
import { useHaptics } from "@/composables/useHaptics";
import { useTour } from "@/composables/useTour";
import Card from "@/components/common/Card.vue";
import Button from "@/components/common/Button.vue";
import type LoginResponse from "@/models/dto";

const router = useRouter();
const store = useUserStore();
const haptics = useHaptics();
const { startTour, hasSeenTour } = useTour();

const email = ref("");
const password = ref("");
const errorMessage = ref("");
const isLoading = ref(false);
const loginSuccess = ref(false);

const handleLogin = async () => {
  if (!email.value || !password.value) {
    errorMessage.value = "Please fill in all fields";
    await haptics.warning();
    return;
  }

  try {
    isLoading.value = true;
    loginSuccess.value = false;
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

    // Show success state
    loginSuccess.value = true;
    await haptics.success();

    // Wait a moment before redirecting to show the success message
    setTimeout(async () => {
      await router.replace("/");

      // Start tour if user hasn't seen it yet
      if (!hasSeenTour.value) {
        setTimeout(() => {
          startTour();
        }, 500); // Small delay to let the home page load
      }
    }, 1500);
  } catch (err) {
    logError("Login error: " + err);
    errorMessage.value = "Incorrect email or password";
    await haptics.error();
    isLoading.value = false;
    loginSuccess.value = false;
  }
};
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
