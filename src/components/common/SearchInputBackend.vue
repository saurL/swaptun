<template>
  <div class="space-y-4">
    <SearchInput
      v-model="searchQuery"
      :placeholder="placeholder"
    />

    <slot
      :results="results"
      :loading="loading"
      :search-query="searchQuery"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from "vue";
import { debounce } from "lodash";
import SearchInput from "./SearchInput.vue";

interface Props {
  placeholder?: string;
  searchFunction: (query: string) => Promise<any[]>;
  debounceMs?: number;
}

const props = withDefaults(defineProps<Props>(), {
  placeholder: "Rechercher...",
  debounceMs: 300,
});

const searchQuery = ref("");
const results = ref<any[]>([]);
const loading = ref(false);

const performSearch = async (term: string) => {
  if (!term.trim()) {
    results.value = [];
    return;
  }

  try {
    loading.value = true;
    results.value = await props.searchFunction(term);
  } catch (error) {
    console.error("Error during search:", error);
    results.value = [];
  } finally {
    loading.value = false;
  }
};

const debouncedSearch = debounce(performSearch, props.debounceMs);

watch(searchQuery, (newValue) => {
  if (!newValue.trim()) {
    results.value = [];
    loading.value = false;
  } else {
    loading.value = true;
    debouncedSearch(newValue);
  }
});
</script>
