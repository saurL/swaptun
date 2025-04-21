import { createApp } from "vue";
import App from "./pages/App.vue";
import { createRouter, createWebHistory } from "vue-router";
import HomePage from "./pages/HomePage.vue";
import AboutPage from "./pages/AboutPage.vue";
import "./assets/css/global.css"


const routes = [
    { path: "/", component: HomePage },
    { path: "/about", component: AboutPage },
  ];

  const router = createRouter({
    history: createWebHistory(),
    routes,
  });
  

createApp(App).use(router).mount("#app");
