<template>
  <div class="flex flex-col items-center justify-center">
    <header class="text-center">
      <h1 class="text-4xl font-bold text-blue-600">Welcome to Swaply</h1>
      <p class="mt-2 text-gray-600">Your Nuxt.js project is ready!</p>
    </header>
    <nuxt-link
      to="/about"
      class="mt-6 px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700"
    >
      Go to About Page
    </nuxt-link>
    <p>fonctionne stp</p>
    <p>et la ?</p>
    <p>aaaaa</p>
    <p>iiiii</p>
    <p>fgsfqsfdqsfqsfdsq</p>
    <p>test</p>
    <p>et la ça fonctionne</p>
    <p>un autre test</p>
    <p>et si j'ajoute ça ?</p>
    <p>tt</p>
  </div>
</template>

<script setup>
import { info, error } from "@tauri-apps/plugin-log";
import { fetch } from "@tauri-apps/plugin-http";

// Récupérer l'adresse de développement de l'application Tauri

// <p>tauri host ip : {{ process.env.TAURI_DEV_HOST }}</p>
const devHost = import.meta.env.TAURI_DEV_HOST;
const couchdbUrl = `http://${devHost}:5984`; // Formuler l'URL de CouchDB

info("URL de CouchDB " + couchdbUrl);

// Exemple de requête pour lister les bases de données dans CouchDB
async function fetchDatabases() {
  try {
    info("Fetching databases from CouchDB...");
    const response = await fetch(`${couchdbUrl}`, {
      method: "GET",
    });
    const data = await response.json();
    info("Bases de données :" + JSON.stringify(data));
  } catch (error) {
    error("Erreur lors de la récupération des bases de données : " + error);
  }
}

fetchDatabases();
definePageMeta({
  layout: "default",
  title: "Home",
});
</script>

<style scoped>
/* Add any custom styles here if needed */
</style>
