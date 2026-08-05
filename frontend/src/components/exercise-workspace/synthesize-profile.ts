import { API_BASE_URL } from "../../lib/api";
import type { ProfileSynthesis } from "./profile-synthesis";

export type SynthesizeLearnerProfileInput = {
  rawNotes: string;
  sourceStepId: string;
};

export type SynthesizeLearnerProfileResult =
  | { ok: true; synthesis: ProfileSynthesis }
  | { ok: false; status: number; message: string };

/**
 * Llama al backend Go (Vertex/Gemini) para sintetizar el perfil de onboarding.
 */
export async function synthesizeLearnerProfile(
  input: SynthesizeLearnerProfileInput,
): Promise<SynthesizeLearnerProfileResult> {
  let response: Response;
  try {
    response = await fetch(`${API_BASE_URL}/api/learner/profile/synthesize`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        raw_notes: input.rawNotes,
        source_step_id: input.sourceStepId,
      }),
    });
  } catch {
    return {
      ok: false,
      status: 0,
      message:
        "No se pudo conectar con el backend en http://localhost:8080. ¿Está corriendo `make run`?",
    };
  }

  if (!response.ok) {
    let message =
      response.status === 400
        ? "El relato es demasiado corto para analizar."
        : "No pudimos analizar tu perfil. Probá de nuevo en unos segundos.";
    const raw = (await response.text()).trim();
    if (raw) {
      try {
        const payload = JSON.parse(raw) as { error?: string };
        if (payload.error) {
          message = payload.error;
        } else {
          message = raw;
        }
      } catch {
        message = raw;
      }
    }
    return {
      ok: false,
      status: response.status,
      message,
    };
  }

  const data = (await response.json()) as {
    purpose?: string;
    urgency?: string;
    vision?: string;
    stack?: string;
  };

  return {
    ok: true,
    synthesis: {
      purpose: data.purpose ?? "",
      urgency: data.urgency ?? "",
      vision: data.vision ?? "",
      stack: data.stack ?? "",
    },
  };
}
