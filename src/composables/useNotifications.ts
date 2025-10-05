import { onMounted, onUnmounted, ref } from "vue";
import { invoke, PluginListener } from "@tauri-apps/api/core";
import { info } from "@tauri-apps/plugin-log";
import {
  getFcmToken,
  requestPushPermission,
  onNewFcmToken,
} from "tauri-plugin-push-notifications";

export function useNotifications() {
  const hasPermission = ref(false);
  const fcmToken = ref<string | null>(null);
  let unlistenToken: PluginListener | null = null;

  const setupNotifications = async () => {
    try {
      // Request permission
      const permission = await requestPushPermission();
      info("Push permission: " + permission);
      hasPermission.value = permission;

      // Get initial token
      const token = await getFcmToken();
      info("Initial FCM token: " + token);
      fcmToken.value = token;

      // Send token to backend
      await sendTokenToBackend(token!);

      // Listen for new tokens
      unlistenToken = await onNewFcmToken((newToken) => {
        info("New FCM token received: " + newToken);
        fcmToken.value = newToken;
        sendTokenToBackend(newToken);
      });
    } catch (error) {
      info("Error setting up notifications: " + error);
      console.error("Error setting up notifications:", error);
    }
  };

  const sendTokenToBackend = async (token: string) => {
    try {
      await invoke("set_fcm_token", { token });
      info("FCM token set successfully");
    } catch (error) {
      info("Error setting FCM token: " + error);
      throw error;
    }
  };

  const sendTestNotification = async (userId: number, title?: string, body?: string) => {
    try {
      await invoke("send_test_notification", {
        title: title || "Test Notification",
        body: body || "This is a test notification from Swaptun.",
        userId,
      });
      info("Test notification sent successfully");
    } catch (error) {
      info("Error sending test notification: " + error);
      throw error;
    }
  };

  const cleanup = () => {
    if (unlistenToken) {
      unlistenToken.unregister();
      unlistenToken = null;
    }
  };

  return {
    hasPermission,
    fcmToken,
    setupNotifications,
    sendTestNotification,
    cleanup,
  };
}
