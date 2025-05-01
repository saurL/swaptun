import { createApp } from "vue";
import App from "./App.vue";
import { createRouter, createWebHistory } from "vue-router";
import "./assets/css/global.css"
import LoginPage from "./pages/LoginPage.vue"
import HomePage from "./pages/HomePage.vue";
import ProfilePage from "./pages/ProfilePage.vue";
import RegisterPage from "./pages/RegisterPage.vue";
import SpotifyAuthPage from "./pages/SpotifyAuthPage.vue";
import { createPinia } from 'pinia';
import { createPlugin } from '@tauri-store/pinia';


const routes = [
  { path: "/", component: LoginPage }, // page de connexion
  { path: "/homepage", component: HomePage },  // page d’accueil
  { path: "/profile", 
    component: ProfilePage, 
    props: { 
      userName: 'John Doe', 
      userAge: 25, 
      userCity: 'Paris', 
      userBio: 'Passionné de musique et de technologie.'
    } 
  }, // page du profil
   { path: "/profile", 
    component: ProfilePage, 
    props: { 
      userName: 'John Doe', 
      userAge: 25, 
      userCity: 'Paris', 
      userBio: 'Passionné de musique et de technologie.'
    } 
  },
  { path: "/register", component: RegisterPage },  // page d’accueil
  { path: "/spotify-auth", component: SpotifyAuthPage },  // page d’accueil
];

  const router = createRouter({
    history: createWebHistory(),
    routes,
  });
  
  const pinia = createPinia();
  pinia.use(createPlugin());
  
createApp(App).use(router).use(pinia).mount("#app");
