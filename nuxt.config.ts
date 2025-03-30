import { defineNuxtConfig } from "nuxt/config";

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  app: {
    rootAttrs: {
    class: "h-full"
    },   },
  srcDir: 'src',

  // (optional) Enable the Nuxt devtools
  devtools: { enabled: true },
  modules: ['@nuxt/ui'],
  css: ['~/assets/css/main.css'],
  // Enable SSG
  ssr: false,

  // Enables the development server to be discoverable by other devices when running on iOS physical devices
  devServer: { host: process.env.TAURI_DEV_HOST || 'localhost' ,
    port:3000
  },
  
  hooks: {
    'vite:extend': function ({ config }) {
      
      if (config.server && config.server.hmr && config.server.hmr !== true) {
        config.server.hmr.protocol = 'ws'
        config.server.hmr.host = process.env.TAURI_DEV_HOST || 'localhost'

        config.server.hmr.port = 3000
      }

    },
    
  },
  vite: {
    
    // Better support for Tauri CLI output
    clearScreen: false,
    
    // Enable environment variables
    // Additional environment variables can be found at
    // https://v2.tauri.app/reference/environment-variables/
    envPrefix: ['VITE_', 'TAURI_'],
    server: {
      host: '0.0.0.0',

      strictPort: true,
      watch: {
        ignored: ["**/src-tauri/**"],
        usePolling: true
    },

      // Tauri requires a consistent port
      
      hmr: {
        host: '0.0.0.0',
        protocol: 'ws',
        port: 3000,
      },
    },
  },

  compatibilityDate: '2025-03-24',
});