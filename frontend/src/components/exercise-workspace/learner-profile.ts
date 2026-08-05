import { server$ } from "@builder.io/qwik-city";
import type { ProfileSynthesis } from "./profile-synthesis";

/** Payload de “migas de pan” hacia el futuro grafo Neo4j. */
export type LearnerProfileSynthesis = ProfileSynthesis & {
  rawNotes: string;
  sourceStepId: string;
  savedAt: string;
};

/**
 * Persistencia simulada del perfil de aprendiz.
 * Sustituible por escritura de nodos/relaciones en Neo4j sin cambiar el contrato UI.
 */
export const saveLearnerProfile = server$(
  async (profileData: LearnerProfileSynthesis) => {
    // Latencia artificial: prepara el UX para un round-trip real.
    await new Promise((resolve) => setTimeout(resolve, 350));
    console.log(
      "Guardando nodo en Grafo (Neo4j):",
      JSON.stringify(profileData, null, 2),
    );
    return { success: true as const };
  },
);
