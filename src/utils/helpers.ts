/**
 * Formate un nombre de pistes pour l'affichage
 */
export function formatTrackCount(count: number): string {
  if (count === 0) return "Aucun titre";
  if (count === 1) return "1 titre";
  return `${count} titres`;
}

/**
 * Tronque un texte à une longueur maximale
 */
export function truncate(text: string, maxLength: number): string {
  if (text.length <= maxLength) return text;
  return text.slice(0, maxLength) + "...";
}

/**
 * Vérifie si une chaîne est un email valide
 */
export function isValidEmail(email: string): boolean {
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
  return emailRegex.test(email);
}

/**
 * Vérifie si un mot de passe est valide (au moins 8 caractères)
 */
export function isValidPassword(password: string): boolean {
  return password.length >= 8;
}

/**
 * Retourne les initiales d'un nom d'utilisateur
 */
export function getUserInitials(username: string): string {
  if (!username) return "?";
  const words = username.trim().split(" ");
  if (words.length === 1) {
    return words[0].charAt(0).toUpperCase();
  }
  return (words[0].charAt(0) + words[words.length - 1].charAt(0)).toUpperCase();
}

/**
 * Converts UTC datetime string to local Date object
 */
export function utcToLocal(utcDateString: string): Date {
  return new Date(utcDateString);
}

/**
 * Formate une date au format français
 */
export function formatDate(date: Date | string): string {
  const d = typeof date === "string" ? new Date(date) : date;
  return d.toLocaleDateString("en-US", {
    year: "numeric",
    month: "long",
    day: "numeric",
  });
}

/**
 * Format a date as a relative time string (e.g., "2 hours ago")
 */
export function formatRelativeTime(date: Date | string): string {
  const d = typeof date === "string" ? new Date(date) : date;
  const now = new Date();
  const diffMs = now.getTime() - d.getTime();
  const diffMins = Math.floor(diffMs / 60000);
  const diffHours = Math.floor(diffMs / 3600000);
  const diffDays = Math.floor(diffMs / 86400000);

  if (diffMins < 1) {
    return "just now";
  } else if (diffMins < 60) {
    return `${diffMins} minute${diffMins !== 1 ? "s" : ""} ago`;
  } else if (diffHours < 24) {
    return `${diffHours} hour${diffHours !== 1 ? "s" : ""} ago`;
  } else if (diffDays < 7) {
    return `${diffDays} day${diffDays !== 1 ? "s" : ""} ago`;
  } else {
    return d.toLocaleDateString("en-US", {
      year: "numeric",
      month: "short",
      day: "numeric",
    });
  }
}

/**
 * Attend un certain nombre de millisecondes
 */
export function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}
