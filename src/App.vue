<template>
  <div>
    <form @submit.prevent="handleSubmit">
      <div>
        <label for="Username">Username:</label>
        <input type="text" id="firstName" v-model="form.userName" required />
      </div>
      <div>
        <label for="firstName">Prénom:</label>
        <input type="text" id="firstName" v-model="form.firstName" required />
      </div>
      <div>
        <label for="lastName">Nom:</label>
        <input type="text" id="lastName" v-model="form.lastName" required />
      </div>
      <div>
        <label for="email">Adresse mail:</label>
        <input type="email" id="email" v-model="form.email" required />
      </div>
      <div>
        <label for="password">Mot de passe:</label>
        <input type="password" id="password" v-model="form.password" required />
      </div>
      <button type="submit">S'inscrire</button>
    </form>

    <form @submit.prevent="handleLogin">
      <div>
        <label for="loginUsername">Username:</label>
        <input
          type="text"
          id="loginUsername"
          v-model="loginForm.userName"
          required
        />
      </div>
      <div>
        <label for="loginPassword">Mot de passe:</label>
        <input
          type="password"
          id="loginPassword"
          v-model="loginForm.password"
          required
        />
      </div>
      <button type="submit">Se connecter</button>
    </form>
  </div>
</template>

<script lang="ts" setup>
import { invoke } from "@tauri-apps/api/core";
import { info, error } from "@tauri-apps/plugin-log";
import { reactive } from "vue";

const form = reactive({
  firstName: "",
  lastName: "",
  email: "",
  password: "",
  userName: "",
});

function handleSubmit() {
  invoke("register", {
    username: form.userName,
    password: form.password,
    firstName: form.firstName,
    lastName: form.lastName,
    email: form.email,
  }).then((message) => info(message));
}

const loginForm = reactive({
  userName: "",
  password: "",
});

function handleLogin() {
  invoke("login", {
    username: loginForm.userName,
    password: loginForm.password,
  })
    .then((response) => info(response))
    .catch((err) => error(err));
}
</script>

<style></style>
