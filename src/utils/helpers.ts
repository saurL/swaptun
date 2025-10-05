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
 * Formate une date au format français
 */
export function formatDate(date: Date | string): string {
  const d = typeof date === "string" ? new Date(date) : date;
  return d.toLocaleDateString("fr-FR", {
    year: "numeric",
    month: "long",
    day: "numeric",
  });
}

/**
 * Attend un certain nombre de millisecondes
 */
export function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}
