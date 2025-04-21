import { createApp } from "vue";
import App from "./App.vue";
import { createRouter, createWebHistory } from "vue-router";
import "./assets/css/global.css"
import LoginPage from "./pages/LoginPage.vue"
import HomePage from "./pages/HomePage.vue";
import ProfilePage from "./pages/ProfilePage.vue";


const routes = [
  { path: "/login", component: LoginPage }, // page de connexion
  { path: "/", component: HomePage },  // page d’accueil
  { path: "/profile", 
    component: ProfilePage, 
    props: { 
      userName: 'John Doe', 
      userAge: 25, 
      userCity: 'Paris', 
      userBio: 'Passionné de musique et de technologie.'
    } 
  }, // page du profil
];

  const router = createRouter({
    history: createWebHistory(),
    routes,
  });
  

createApp(App).use(router).mount("#app");
