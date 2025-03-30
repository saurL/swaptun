<template>
  <div>
    <UForm :state="state" class="space-y-4" @submit="onSubmit">
      <UFormField label="Email" name="email">
        <UInput v-model="state.email" />
      </UFormField>

      <UFormField label="Password" name="password">
        <UInput v-model="state.password" type="password" />
      </UFormField>

      <UButton type="submit"> Submit </UButton>
    </UForm>
  </div>
</template>

<script lang="ts" setup>
import { reactive } from "vue";
import type { FormSubmitEvent } from "@nuxt/ui";
const state = reactive({
  email: "",
  password: "",
});

function submitForm() {
  console.log("Form submitted:", state);
}

async function onSubmit(event: FormSubmitEvent<typeof state>) {
  console.log(event.data);
}
</script>

<style></style>
