

import { defineStore } from "pinia";
import { ref } from "vue";

export const useStore = defineStore("identification_token", () => {
  const identification_token = ref("");

  const setIdentificationToken = (token: string) => {
    identification_token.value = token;
  };

  return {
    identification_token,
    setIdentificationToken,
  };
}, {tauri: {
    saveOnChange: true,
}});