import { createRouter, createWebHistory } from "vue-router";
import LoginPage from "./pages/LoginPage.vue";
import HomePage from "./pages/HomePage.vue";
import HomeTab from "./pages/HomeTab.vue";
import FriendsTab from "./pages/FriendsTab.vue";
import ProfileTab from "./pages/ProfileTab.vue";
import RegisterPage from "./pages/RegisterPage.vue";
import ForgotPasswordPage from "./pages/ForgotPasswordPage.vue";
import ResetPasswordPage from "./pages/ResetPasswordPage.vue";
import SharedPlaylistsPage from "./pages/SharedPlaylistsPage.vue";
import SharePlaylistModal from "./components/playlists/SharePlaylistModal.vue";
import SendPlaylistModal from "./components/playlists/SendPlaylistModal.vue";
import { useUserStore } from "./store/user";


const routes = [
  { path: "/login", component: LoginPage, name: "Login" },
  { path: "/", redirect: "/home/accueil" },
  {
    path: "/home",
    component: HomePage,
    name: "Home",
    meta: { requiresAuth: true },
    redirect: "/home/accueil",
    children: [
      {
        path: "accueil",
        component: HomeTab,
        name: "HomeTab",
        children: [
          {
            path: "share/:playlistId",
            component: SharePlaylistModal,
            name: "SharePlaylist",
          },
        ],
      },
      { path: "amis", component: FriendsTab, name: "FriendsTab" },
      {
        path: "shared",
        component: SharedPlaylistsPage,
        name: "SharedPlaylistsTab",
        children: [
          {
            path: "send/:playlistId",
            component: SendPlaylistModal,
            name: "send-playlist",
          },
        ],
      },
      { path: "profil", component: ProfileTab, name: "ProfileTab" },
    ],
  },
  { path: "/register", component: RegisterPage, name: "Register" },
  {
    path: "/forgot-password",
    component: ForgotPasswordPage,
    name: "ForgotPassword",
  },
  {
    path: "/reset-password",
    component: ResetPasswordPage,
    name: "ResetPassword",
    meta: { requiresQuery: true },
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

router.beforeEach((to, from, next) => {
  const userStore = useUserStore();
  console.log("Going from", from.fullPath, "to", to.fullPath);

  if (to.meta.requiresQuery && Object.keys(to.query).length === 0) {
    console.log("Route requires query parameters, redirecting to home");
    next({ path: "/" });
  } else if (to.meta.requiresAuth && userStore.token == null) {
    console.log("User is not authenticated, redirecting to login");
    next({ name: "Login" });
  } else {
    console.log(
      "User is authenticated or route does not require auth, proceeding"
    );
    next();
  }
});

export default router;
