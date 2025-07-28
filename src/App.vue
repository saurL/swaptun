<script setup lang="ts">
import { useUserStore } from "./store/user";
import { watchEffect } from "vue";
import { useRouter } from "vue-router";
import loadingAppAnimation from "./components/LoadingAppAnimation.vue";
import { useAppStore } from "./store/app";
import { storeToRefs } from "pinia";
const router = useRouter();
const userStore = useUserStore();
const appStore = useAppStore();
const {isAppReady} = storeToRefs(appStore)
const userRefStore = storeToRefs(userStore);

watchEffect(() => {
  if (userRefStore.token.value != null) {
    // If the user is authenticated and the app is ready, navigate to the homepage
    router.replace("/homepage");
  }
  isAppReady.value = true;
});

</script>
<template>
  <div
    class="h-screen w-screen bg-[#121212] text-white flex flex-col max-h-screen w-screen max-w-full overflow-y-scroll overflow-x-hidden
 items-center justify-center">
    <loadingAppAnimation v-if="!isAppReady" />
    <RouterView v-else />
  </div>
</template>

<style lang="css">
html,
body {
  overscroll-behavior-x: none;
  /* désactive le scroll horizontal naturel */
  touch-action: pan-x pan-y;
  /* désactive le zoom par pincement */
  -ms-overflow-style: none;
  overflow: hidden;
}

div,
span {
  -ms-overflow-style: auto;
}

/* Hide scrollbar for Chrome, Safari and Opera */
html::-webkit-scrollbar {
  display: none;
}

/* Hide scrollbar for IE, Edge and Firefox */
* {
  -ms-overflow-style: none;
  /* IE and Edge */
  scrollbar-width: none;
  /* Firefox */
  touch-action: pan-x, pan-y;
}
</style>
