<template>
  <div>
    <form @submit.prevent="submitForm">
      <div>
        <label for="firstName">Prénom:</label>
        <input type="text" id="firstName" v-model="form.firstName" required />
      </div>
      <div>
        <label for="lastName">Nom:</label>
        <input type="text" id="lastName" v-model="form.lastName" required />
      </div>
      <div>
        <label for="username">Nom d'utilisateur:</label>
        <input type="text" id="username" v-model="form.username" required />
      </div>
      <div>
        <label for="email">Email:</label>
        <input type="email" id="email" v-model="form.email" required />
      </div>
      <button type="submit">Créer un utilisateur</button>
    </form>
  </div>
</template>

<script lang="ts" setup>
import { reactive } from "vue";

const form = reactive({
  firstName: "",
  lastName: "",
  username: "",
  email: "",
});

const submitForm = () => {
  console.log("Formulaire soumis:", form);
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
