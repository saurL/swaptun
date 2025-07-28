import { createApp } from "vue";
import App from "./App.vue";
import { createRouter, createWebHistory } from "vue-router";
import "./assets/css/global.css"
import LoginPage from "./pages/LoginPage.vue"
import HomePage from "./pages/HomePage.vue";
import RegisterPage from "./pages/RegisterPage.vue";
import { createPinia } from 'pinia';
import { createPlugin } from '@tauri-store/pinia';


const routes = [
  { path: "/", component: LoginPage }, // page de connexion
  { path: "/homepage", component: HomePage },  // page d’accueil
  { path: "/register", component: RegisterPage },  // page d’accueil
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});
  
const pinia = createPinia();
pinia.use(createPlugin());
  
createApp(App).use(router).use(pinia).mount("#app");
