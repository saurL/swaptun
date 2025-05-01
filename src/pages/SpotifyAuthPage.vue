<template>
  <div>
    <form @submit.prevent="authenticateSpotify">
      <div>
        <label for="clientId">Client ID:</label>
        <input type="text" id="clientId" v-model="form.clientId" required />
      </div>
      <div>
        <label for="clientSecret">Client Secret:</label>
        <input
          type="text"
          id="clientSecret"
          v-model="form.clientSecret"
          required
        />
      </div>
      <button type="submit">S'authentifier avec Spotify</button>
    </form>
  </div>
</template>

<script lang="ts" setup>
import { reactive } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { info, error } from "@tauri-apps/plugin-log";

const form = reactive({
  clientId: "",
  clientSecret: "",
});

const authenticateSpotify = async () => {
  try {
    await invoke("authenticate_spotify", {
      clientId: form.clientId,
      clientSecret: form.clientSecret,
    });
    info("Authentification réussie !");
  } catch (err: any) {
    error("Erreur lors de l'authentification :", err);
  }
};
</script>

<style>
form {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

label {
  font-weight: bold;
}

input {
  padding: 0.5rem;
  border: 1px solid #ccc;
  border-radius: 4px;
}

button {
  padding: 0.5rem 1rem;
  background-color: #007bff;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

button:hover {
  background-color: #0056b3;
}
</style>
