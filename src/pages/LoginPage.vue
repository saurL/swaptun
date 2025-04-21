<template>
    <div class="min-h-screen flex items-center justify-center bg-[#1E1E1E] px-4">
      <div class="bg-white/10 backdrop-blur-md shadow-lg rounded-2xl w-full max-w-md p-8 border border-white/10">
        <!-- LOGO -->
        <div class="flex justify-center mb-6">
          <img src="/src/assets/images/icon.ico" alt="Swaply Logo" class="h-16 w-16" />
        </div>
  
        <h1 class="text-2xl font-bold text-center text-white mb-4">Bienvenue sur Swaply</h1>
        <p class="text-sm text-gray-300 text-center mb-8">Partage ta vibe avec tes amis.</p>
  
        <form class="space-y-5">
          <div>
            <label class="block text-sm font-medium text-gray-300">Email</label>
            <input
              type="email"
              class="w-full mt-1 px-4 py-2 rounded-lg bg-[#2A2A2A] text-white placeholder-gray-500 border border-gray-600 focus:outline-none focus:ring-2 focus:ring-[#00CFE8]"
              placeholder="melomane@swaply.app"
            />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-300">Mot de passe</label>
            <input
              type="password"
              class="w-full mt-1 px-4 py-2 rounded-lg bg-[#2A2A2A] text-white placeholder-gray-500 border border-gray-600 focus:outline-none focus:ring-2 focus:ring-[#00CFE8]"
              placeholder="••••••••"
            />
          </div>
          <button
            type="submit"
            class="w-full bg-[#00CFE8] hover:bg-[#FFC436] text-[#1E1E1E] font-semibold py-2 px-4 rounded-lg transition"
          >
            Se connecter
          </button>
        </form>
  
        <p class="text-center text-sm text-gray-400 mt-6">
          Pas encore de compte ?
          <a href="#" class="text-[#00CFE8] hover:text-[#FFC436] hover:underline">Créer un compte</a>
        </p>
      </div>

      <!-- Lien vers la homepage -->
    <a href="/" class="absolute top-4 left-4 text-[#00CFE8] hover:text-[#FFC436] text-sm font-medium transition-all">
      Lien vers l'accueil
    </a>

    </div>
  </template>
  
  <style scoped>
  </style>
  