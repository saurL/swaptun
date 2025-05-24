<template>
  <nav
    class="w-full bg-[#1E1E1E] border-t border-white/10 flex justify-around py-4 z-50"
  >
    <!-- Accueil Button -->
    <router-link
      to="/"
      class="text-white flex flex-col items-center transition-transform transform hover:scale-110"
    >
      <i class="lucide lucide-home text-2xl"></i>
      <span class="text-xs mt-1">Accueil</span>
    </router-link>

    <!-- Partager Button -->
    <button
      class="text-[#00CFE8] flex flex-col items-center transition-transform transform hover:scale-110"
    >
      <i class="lucide lucide-share-2 text-2xl"></i>
      <span class="text-xs mt-1">Partager</span>
    </button>

    <!-- Profil Button -->
    <router-link
      to="/profile"
      class="text-white flex flex-col items-center transition-transform transform hover:scale-110"
    >
      <i class="lucide lucide-user text-2xl"></i>
      <span class="text-xs mt-1">Profil</span>
    </router-link>
  </nav>
</template>

<script setup></script>

<style scoped>
/* Animation de l'icône au survol */
button:hover i {
  transform: scale(1.2);
  transition: transform 0.2s ease-in-out;
}

/* Style des icônes */
button i {
  transition: color 0.3s ease-in-out;
}

button:hover i {
  color: #ffc436;
}

/* Change the text color on hover for buttons */
button:hover span {
  color: #ffc436;
}
</style>
