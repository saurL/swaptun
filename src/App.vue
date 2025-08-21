<script setup lang="ts">

/*
import loadingAppAnimation from "./components/LoadingAppAnimation.vue";
import { onMounted } from "vue";
import { useAppStore } from "./store/app";
import { storeToRefs } from "pinia";

const appStore = useAppStore();

const { isAppReady } = storeToRefs(appStore);

onMounted(async () => {
  
  isAppReady.value = true;
});
*/

</script>
<template>

  <div
    class=" h-screen bg-[#121212] text-white flex flex-col max-h-screen w-screen max-w-full overflow-y-scroll overflow-x-hidden items-center justify-center"
  >

    <!--<loadingAppAnimation v-if="!isAppReady" />-->
    <RouterView />


  </div>

</template>

<style lang="css">
html,
body {
  margin: 0;
  padding: 0;
  height: 100%;
  width: 100%;
  overflow: hidden; /* Prevent scrolling */
  overscroll-behavior: none; /* Disable overscroll effects */
  touch-action: none; /* Disable all touch scrolling and zooming */
  -ms-overflow-style: none; /* IE/Edge */
  scrollbar-width: none; /* Firefox */
  position: fixed; /* Fix the body to prevent scrolling */
  top: 0;
  left: 0;
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
  user-select: none;
  -ms-overflow-style: none;
  /* IE and Edge */
  scrollbar-width: none;
  /* Firefox */
  touch-action: pan-x, pan-y;
}
</style>
