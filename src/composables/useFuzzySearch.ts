import { computed, ref, Ref } from "vue";

// Calcul de la distance de Levenshtein pour la similarité
const levenshteinDistance = (str1: string, str2: string): number => {
  const len1 = str1.length;
  const len2 = str2.length;
  const matrix: number[][] = [];

  for (let i = 0; i <= len1; i++) {
    matrix[i] = [i];
  }
  for (let j = 0; j <= len2; j++) {
    matrix[0][j] = j;
  }

  for (let i = 1; i <= len1; i++) {
    for (let j = 1; j <= len2; j++) {
      const cost = str1[i - 1] === str2[j - 1] ? 0 : 1;
      matrix[i][j] = Math.min(
        matrix[i - 1][j] + 1,
        matrix[i][j - 1] + 1,
        matrix[i - 1][j - 1] + cost
      );
    }
  }

  return matrix[len1][len2];
};

// Calcul du score de similarité (0 = meilleur match, plus grand = moins similaire)
const calculateSimilarity = (query: string, text: string): number => {
  const queryLower = query.toLowerCase();
  const textLower = text.toLowerCase();

  // Score prioritaire : correspondance exacte dans le texte
  if (textLower === queryLower) return 0;

  // Score très bon : le texte commence par la requête
  if (textLower.startsWith(queryLower)) return 0.5;

  // Score bon : le texte contient la requête
  if (textLower.includes(queryLower)) return 1;

  // Calcul de la distance minimum entre la requête et tous les mots du texte
  const words = textLower.split(/\s+/);
  let minDistance = Infinity;
  let bestWordMatch = false;

  for (const word of words) {
    // Priorité aux mots qui commencent par la requête
    if (word.startsWith(queryLower)) {
      return 2; // Bon score pour préfixe de mot
    }

    // Vérifier si le mot contient la requête
    if (word.includes(queryLower)) {
      bestWordMatch = true;
      minDistance = Math.min(minDistance, 3);
    } else {
      const distance = levenshteinDistance(queryLower, word);
      minDistance = Math.min(minDistance, distance + 4); // Pénalité supplémentaire
    }
  }

  return minDistance;
};

export function useFuzzySearch<T>(
  items: Ref<T[]>,
  searchQuery: Ref<string>,
  getSearchableText: (item: T) => string
) {
  const filteredItems = computed(() => {
    if (!searchQuery.value.trim()) return items.value;

    const query = searchQuery.value.trim();
    const queryLength = query.length;

    // Seuil de tolérance plus strict basé sur la longueur de la requête
    // 1-2 chars: tolérance de 1 erreur (score <= 3)
    // 3-4 chars: tolérance de 2 erreurs (score <= 4)
    // 5-6 chars: tolérance de 2 erreurs (score <= 5)
    // 7+ chars: tolérance de 3 erreurs max (score <= 6)
    const maxDistance = queryLength <= 2 ? 3 :
                       queryLength <= 4 ? 4 :
                       queryLength <= 6 ? 5 : 6;

    return items.value
      .map((item) => ({
        item,
        score: calculateSimilarity(query, getSearchableText(item)),
      }))
      .filter(({ score }) => score <= maxDistance)
      .sort((a, b) => a.score - b.score) // Tri par score croissant (meilleurs matchs en premier)
      .map(({ item }) => item);
  });

  return {
    filteredItems,
  };
}
