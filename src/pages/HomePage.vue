<template>
  <MainLayout>
    <GreetingCard :userName="userStore.username" />

    <div class="relative flex items-center justify-center h-screen">
      <!-- Bouton central -->
      <button
        @click="toggleMenu"
        class="z-10 w-16 h-16 rounded-full bg-blue-500 text-white flex items-center justify-center"
      >
        🎵
      </button>

      <!-- Icônes circulaires -->
      <transition-group name="fade" tag="div">
        <div
          v-for="(item, i) in platforms"
          :key="item.name"
          class="absolute w-12 h-12 rounded-full bg-white shadow-lg flex items-center justify-center cursor-pointer"
          :style="circlePosition(i, platforms.length)"
          @click="goToPlatform(item)"
        >
          <img :src="item.icon" alt="" class="w-6 h-6" />
        </div>
      </transition-group>
    </div>

    <button @click="connectToSpotify" class="connect-button">
      Connect to Spotify
    </button>

    <button @click="connectToYoutube" class="connect-button">
      Connect to youtube
    </button>

    <button @click="testNotification" class="connect-button">
      Tester les notifications
    </button>
    <button @click="testSendPlaylist" class="connect-button">
      Tester l'envoi de playlist
    </button>

    <button @click="connectToAppleMusic" class="connect-button">
      Connect to Apple Music
    </button>

    <div class="p-4 max-w-md mx-auto">
      <!-- Input de recherche -->
      <input
        :value="query"
        @input="
          // @ts-ignore
          query = $event.target.value
        "
        type="text"
        placeholder="Rechercher un utilisateur..."
        class="w-full p-2 border border-gray-300 rounded mb-4 focus:outline-none focus:ring-2 focus:ring-blue-500"
      />

      <!-- Liste des utilisateurs -->
      <ul>
        <li
          v-for="user in users"
          :key="user.id"
          class="p-2 border-b border-gray-200"
        >
          {{ user.username }}

          <button
            @click="userStore.addFriend(user)"
            class="ml-2 text-blue-500 hover:underline"
          >
            Add Friend
          </button>
        </li>
      </ul>

      <!-- Message si aucun résultat -->
      <p v-if="users.length === 0 && query" class="text-gray-500">
        Aucun utilisateur trouvé.
      </p>
    </div>
    <button @click="logout" class="connect-button">Logout</button>
    <div class="flex flex-col h-20">
      <div class="overflow-y-auto">
        <PlaylistList />
      </div>
    </div>
  </MainLayout>
</template>

<script setup lang="ts">
import MainLayout from "@/layouts/MainLayout.vue";
import GreetingCard from "@/components/GreetingCard.vue";
import PlaylistList from "@/components/PlaylistList.vue";
import { useUserStore } from "@/store/user";
import { invoke } from "@tauri-apps/api/core";
import { onMounted, ref, watch } from "vue";
import { info } from "@tauri-apps/plugin-log";
import { useRouter } from "vue-router";
import {
  getFcmToken,
  requestPushPermission,
  onNewFcmToken,
} from "tauri-plugin-push-notifications";

// @ts-ignore
import { debounce } from "lodash";
import User from "@/models/user";
import { onBeforeRouteLeave } from "vue-router";

const isOpen = ref(false);

const platforms = [
  { name: "Spotify", icon: "/icons/spotify.svg" },
  { name: "Deezer", icon: "/icons/deezer.svg" },
  { name: "Apple Music", icon: "/icons/applemusic.svg" },
  { name: "YouTube Music", icon: "/icons/youtubemusic.svg" },
];

// toggle menu
function toggleMenu() {
  isOpen.value = !isOpen.value;
}

// calcul de la position en cercle
function circlePosition(index: number, total: number) {
  if (!isOpen.value) return { display: "none" };

  const angle = (index / total) * (2 * Math.PI);
  const radius = 100; // distance du bouton central

  return {
    transform: `translate(${Math.cos(angle) * radius}px, ${
      Math.sin(angle) * radius
    }px)`,
    transition: "transform 0.3s ease",
  };
}

// action quand on clique sur une icône
function goToPlatform(item: any) {
  console.log("Ouvrir", item.name);
}

// si tu veux que ça disparaisse quand on change de route
onBeforeRouteLeave(() => {
  isOpen.value = false;
});
// Input de recherche
const query = ref("");
// Fonction qui simule une recherche d'utilisateur

async function searchUsers(term: string) {
  // Remplace par ton appel à l'API ou SeaORM
  // Exemple fictif
  invoke<User[]>("search_users", { search: term }).then((response) => {
    users.value = response;
  });
}

// Debounce pour limiter les appels à searchUsers
const debouncedSearch = debounce(searchUsers, 300);
const users = ref<User[]>([]);
// Watch sur l'input
watch(query, (newValue) => {
  console.log("Input changed:", newValue);
  console.log("query value:", query.value);
  if (newValue.trim() === "") {
    users.value = [];
  } else {
    console.log("searching for:", newValue);
    debouncedSearch(newValue);
  }
});
// Liste des utilisateurs trouvés
const userStore = useUserStore();
const router = useRouter();
const connectToSpotify = async () => {
  info("actual url:" + window.location.href);
  await invoke("get_autorization_url_spotify");
  // window.location.href = url;
};

const connectToYoutube = async () => {
  info("actual url:" + window.location.href);
  await invoke("connect_youtube");
  // window.location.href = url;
};

const connectToAppleMusic = async () => {
  info("actual url:" + window.location.href);
  invoke("connect_apple_music")
    .then((response) => {
      info("Apple Music authorization response: " + JSON.stringify(response));
    })
    .catch((error) => {
      info("Error connecting to Apple Music: " + error);
    });
  // window.location.href = url;
};

const logout = () => {
  userStore.reset();
  router.replace("/login");
};

const testNotification = async () => {
  const title = "Test Notification";
  const body = "This is a test notification from Swaptun.";
  const userId = userStore.id; // Assuming userId is available in the user store
  invoke("send_test_notification", { title, body, userId })
    .then((response) => {
      info("Notification sent successfully: " + response);
    })
    .catch((error) => {
      info("Error sending notification: " + error);
    });
};

const testSendPlaylist = async () => {
  // For testing purposes, we'll use a dummy playlist ID and request
  // In a real implementation, you would use actual playlist data
  const playlistId = 675; // Replace with an actual playlist ID
  const sendPlaylistRequest = {
    // This is a placeholder structure - the actual structure depends on the backend
    destination: "YoutubeMusic",
  };

  try {
    const response = await invoke("send_playlist", {
      playlistId: playlistId,
      req: sendPlaylistRequest,
    });
    info("Playlist sent successfully: " + JSON.stringify(response));
  } catch (error) {
    info("Error sending playlist: " + error);
    console.error("Error sending playlist:", error);
  }
};

onMounted(async () => {
  let onnewtoken = await onNewFcmToken((token) => {
    info("New FCM token received: " + token);
    invoke("set_fcm_token", { token: token })
      .then((response) => {
        info("FCM token set successfully: " + response);
      })
      .catch((error) => {
        info("Error setting FCM token: " + error);
      });
  });
  console.log("HomePage mounted");
  const permission = await requestPushPermission();
  info("Push permission: " + permission);
  getFcmToken()
    .then((token) => {
      info("Initial FCM token: " + token);
    })
    .catch((error) => {
      info("Error getting initial FCM token: " + error);
    });

  info("Requesting FCM token...");
  try {
    const token = await getFcmToken();
    info("push token : " + token);
    invoke("set_fcm_token", { token: token })
      .then((response) => {
        info("FCM token set successfully: " + response);
      })
      .catch((error) => {
        info("Error setting FCM token: ");
      });
  } catch (error) {
    info("Error getting FCM token: " + error);
    info("Error object:" + JSON.stringify(error, null, 2));
  }
});
</script>

<style scoped>
.connect-button {
  padding: 10px 20px;
  background-color: #1db954;
  color: white;
  border: none;
  border-radius: 20px;
  cursor: pointer;
  font-weight: bold;
  margin: 20px 0;
}

.connect-button:hover {
  background-color: #1ed760;
}
</style>
