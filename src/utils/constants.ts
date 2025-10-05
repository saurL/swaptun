export const PLATFORMS = [
  {
    id: "spotify",
    name: "Spotify",
    icon: "/src/assets/images/spotify.svg",
  },
  {
    id: "appleMusic",
    name: "Apple Music",
    icon: "/src/assets/images/Apple_Music_icon.svg.png",
  },
  {
    id: "youtube",
    name: "YouTube Music",
    icon: "/src/assets/images/Youtube_Music_icon.svg.png",
  },
] as const;

export const ROUTES = {
  HOME: "/",
  HOMEPAGE: "/Homepage",
  LOGIN: "/login",
  REGISTER: "/register",
  FORGOT_PASSWORD: "/forgot-password",
  RESET_PASSWORD: "/reset-password",
} as const;

export const COLORS = {
  primary: "#00CFE8",
  secondary: "#FFC436",
  background: "#121212",
  surface: "#1E1E1E",
  surfaceLight: "#2A2A2A",
} as const;
