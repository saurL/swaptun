<template>
  <div>
    <form @submit.prevent="handleSubmit">
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
  </div>
</template>

<script lang="ts" setup>
import { reactive } from "vue";

const form = reactive({
  firstName: "",
  lastName: "",
  email: "",
  password: "",
});

function handleSubmit() {
  console.log("Form submitted:", form);
  // Add your form submission logic here
}
</script>

<style></style>
