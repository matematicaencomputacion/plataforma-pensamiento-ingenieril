import type { ProfileSynthesis } from "./profile-synthesis";

/** Payload de “migas de pan” hacia el futuro grafo Neo4j / GET|PUT profile. */
export type LearnerProfileSynthesis = ProfileSynthesis & {
  rawNotes: string;
  sourceStepId: string;
  savedAt: string;
};

/**
 * Persistencia local de transición (sin server$).
 * Evita OOM de SSR por server$ en el árbol de /exercise.
 * Se reemplaza por PUT /api/user/profile en el change de persistencia.
 */
export async function saveLearnerProfile(
  profileData: LearnerProfileSynthesis,
): Promise<{ success: true } | { success: false }> {
  await new Promise((resolve) => setTimeout(resolve, 150));
  if (typeof console !== "undefined") {
    console.log(
      "Guardando perfil (cliente/transición):",
      JSON.stringify(profileData, null, 2),
    );
  }
  try {
    if (typeof localStorage !== "undefined") {
      localStorage.setItem(
        "ppi.learner.profile.draft",
        JSON.stringify(profileData),
      );
    }
  } catch {
    /* private mode */
  }
  return { success: true as const };
}
