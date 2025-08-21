import { createApp } from "vue";
import App from "./App.vue";
import {  createRouter, createWebHistory } from "vue-router";
import "./assets/css/global.css"
import LoginPage from "./pages/LoginPage.vue"
import HomePage from "./pages/HomePage.vue";
import RegisterPage from "./pages/RegisterPage.vue";
import ForgotPasswordPage from "./pages/ForgotPasswordPage.vue";
import ResetPasswordPage from "./pages/ResetPasswordPage.vue";
import { createPinia } from 'pinia';
import { createPlugin } from '@tauri-store/pinia';
import { useUserStore } from "./store/user";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import "tauri-plugin-safe-area-insets-css-api"
(window as any).canGoBack = () => {
  return window.location.pathname !== "/"; // ou ta page d’accueil réelle
};

const routes = [
  { path: "/login", component: LoginPage , name: "Login" }, // page de connexion
  { path: "/", redirect: "/Homepage" }, // redirection vers la page d’accueil
  { path: "/Homepage", component: HomePage, name: "Home", meta: { requiresAuth: true } },  // page d’accueil
  { path: "/register", component: RegisterPage, name: "Register" },  // page d’inscription
  { path: "/forgot-password", component: ForgotPasswordPage, name: "ForgotPassword" },  // page de réinitialisation de mot de passe
  { path: "/reset-password", component: ResetPasswordPage, name: "ResetPassword", meta: { requiresQuery: true } },  // page de réinitialisation de mot de passe
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

const pinia = createPinia();
pinia.use(createPlugin());
let app = createApp(App).use(router).use(pinia);
const userStore = useUserStore();
await userStore.$tauri.start();

listen<string>("routing", (event) => {
  const route = event.payload ;
  console.log("Received route event with query:", route);
  router.push(route);
});
router.beforeEach((to, from, next) => {
    const userStore = useUserStore();
    console.log("Going from", from.fullPath, "to", to.fullPath);
  if (to.meta.requiresQuery && Object.keys(to.query).length === 0) {
    console.log("Route requires query parameters, redirecting to home");
    next({ path: "/" });
  } else if (to.meta.requiresAuth && userStore.token == null) {
    console.log("User is not authenticated, redirecting to login");
    next({ name: 'Login' });
  } else {
    // Sinon on laisse passer
    console.log("User is authenticated or route does not require auth, proceeding");
    next();
  }
});

await router.isReady();
/*
await invoke("check_opening_notification");
  */
if (userStore.token == null) {
  router.replace({ name: 'Login' });
}

await invoke("check_opening_url");
console.log("avant mount");
// @ts-ignore
if (window.setSafeAreaInsets) {
// @ts-ignore
    window.setSafeAreaInsets();
    console.log("Safe area insets set");
}
else {
    console.log("Safe area insets function not found");
}


app.mount("#app");

console.log("après mount");

