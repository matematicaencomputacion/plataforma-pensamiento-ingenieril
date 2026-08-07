import { API_BASE_URL } from "../../lib/api";
import { getStoredToken } from "../../lib/auth";
import type { ProfileSynthesis } from "./profile-synthesis";

/** Payload de “migas de pan” hacia el futuro grafo Neo4j / GET|PUT profile. */
export type LearnerProfileSynthesis = ProfileSynthesis & {
  rawNotes: string;
  sourceStepId: string;
  savedAt: string;
};

/** Campos que acepta PUT /api/user/profile. */
export type UserProfilePayload = {
  lifePurpose: string;
  urgency: string;
  vision5Years: string;
  techStack: string;
};

export function synthesisToUserProfile(
  synthesis: ProfileSynthesis,
): UserProfilePayload {
  return {
    lifePurpose: synthesis.purpose.trim(),
    urgency: synthesis.urgency.trim(),
    vision5Years: synthesis.vision.trim(),
    techStack: synthesis.stack.trim(),
  };
}

/**
 * Confirmación local de la síntesis (borrador).
 * La persistencia real ocurre al avanzar (PUT /api/user/profile).
 */
export async function saveLearnerProfile(
  profileData: LearnerProfileSynthesis,
): Promise<{ success: true } | { success: false }> {
  await new Promise((resolve) => setTimeout(resolve, 150));
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

export type PutUserProfileResult =
  | { ok: true; profile: UserProfilePayload }
  | { ok: false; status: number; message: string };

/**
 * Persiste el perfil de coaching del alumno autenticado.
 * Solo el caller debe avanzar de paso si `ok &&` implícito status 200.
 */
export async function putUserProfile(
  payload: UserProfilePayload,
): Promise<PutUserProfileResult> {
  const token = getStoredToken();
  if (!token) {
    return {
      ok: false,
      status: 401,
      message: "Sesión expirada. Volvé a iniciar sesión para guardar tu perfil.",
    };
  }

  let res: Response;
  try {
    res = await fetch(`${API_BASE_URL}/api/user/profile`, {
      method: "PUT",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${token}`,
      },
      body: JSON.stringify(payload),
    });
  } catch {
    return {
      ok: false,
      status: 0,
      message: "No pudimos conectar con el servidor. Revisá la conexión.",
    };
  }

  if (res.status !== 200) {
    let message = `No pudimos guardar el perfil (HTTP ${res.status}).`;
    try {
      const body = (await res.json()) as { error?: string };
      if (body.error) {
        message = body.error;
      }
    } catch {
      /* ignore */
    }
    return { ok: false, status: res.status, message };
  }

  const profile = (await res.json()) as UserProfilePayload;
  try {
    if (typeof localStorage !== "undefined") {
      localStorage.setItem("ppi.learner.profile", JSON.stringify(profile));
      localStorage.removeItem("ppi.learner.profile.draft");
    }
  } catch {
    /* private mode */
  }
  return { ok: true, profile };
}
