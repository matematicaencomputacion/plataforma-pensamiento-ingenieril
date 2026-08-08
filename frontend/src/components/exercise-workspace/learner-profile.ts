import { API_BASE_URL } from "../../lib/api";
import { getStoredToken } from "../../lib/auth";
import type { ProfileSynthesis } from "./profile-synthesis";

/** Payload de “migas de pan” hacia el futuro grafo Neo4j / GET|PUT profile. */
export type LearnerProfileSynthesis = ProfileSynthesis & {
  rawNotes: string;
  sourceStepId: string;
  savedAt: string;
};

/** Campos que acepta GET|PUT /api/user/profile (contrato Go `LearnerProfile`). */
export type UserProfilePayload = {
  lifePurpose: string;
  urgency: string;
  vision5Years: string;
  techStack: string;
};

export const EMPTY_USER_PROFILE: UserProfilePayload = {
  lifePurpose: "",
  urgency: "",
  vision5Years: "",
  techStack: "",
};

/** Normaliza null/undefined y espacios para comparación y wire JSON. */
export function normalizeUserProfile(
  input: Partial<UserProfilePayload> | null | undefined,
): UserProfilePayload {
  return {
    lifePurpose: String(input?.lifePurpose ?? "").trim(),
    urgency: String(input?.urgency ?? "").trim(),
    vision5Years: String(input?.vision5Years ?? "").trim(),
    techStack: String(input?.techStack ?? "").trim(),
  };
}

export function snapshotUserProfile(profile: UserProfilePayload): string {
  return JSON.stringify(normalizeUserProfile(profile));
}

export function profilesEqual(
  a: UserProfilePayload,
  b: UserProfilePayload,
): boolean {
  return snapshotUserProfile(a) === snapshotUserProfile(b);
}

export function synthesisToUserProfile(
  synthesis: ProfileSynthesis,
): UserProfilePayload {
  return normalizeUserProfile({
    lifePurpose: synthesis.purpose,
    urgency: synthesis.urgency,
    vision5Years: synthesis.vision,
    techStack: synthesis.stack,
  });
}

/**
 * Confirmación local de la síntesis (borrador).
 * La persistencia real ocurre al avanzar (PUT /api/user/profile) si está dirty.
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

export type FetchUserProfileResult =
  | { ok: true; profile: UserProfilePayload; empty: boolean }
  | { ok: false; status: number; message: string };

export function userProfileToSynthesis(
  profile: UserProfilePayload,
): ProfileSynthesis {
  const n = normalizeUserProfile(profile);
  return {
    purpose: n.lifePurpose,
    urgency: n.urgency,
    vision: n.vision5Years,
    stack: n.techStack,
  };
}

export function isUserProfileEmpty(profile: UserProfilePayload): boolean {
  const n = normalizeUserProfile(profile);
  return (
    !n.lifePurpose && !n.urgency && !n.vision5Years && !n.techStack
  );
}

async function readApiError(res: Response, fallback: string): Promise<string> {
  let text = "";
  try {
    text = (await res.text()).trim();
  } catch {
    return `${fallback} (HTTP ${res.status})`;
  }
  if (!text) {
    return `${fallback} (HTTP ${res.status})`;
  }
  try {
    const body = JSON.parse(text) as { error?: string; message?: string };
    if (body.error && body.error.trim()) {
      return body.error.trim();
    }
    if (body.message && body.message.trim()) {
      return body.message.trim();
    }
  } catch {
    /* texto plano */
  }
  return text.slice(0, 240);
}

function parseProfileBody(raw: unknown): UserProfilePayload {
  if (!raw || typeof raw !== "object") {
    return { ...EMPTY_USER_PROFILE };
  }
  const obj = raw as Record<string, unknown>;
  // Contrato canónico + aliases por si el wire llega con keys de síntesis.
  return normalizeUserProfile({
    lifePurpose: String(obj.lifePurpose ?? obj.purpose ?? ""),
    urgency: String(obj.urgency ?? ""),
    vision5Years: String(obj.vision5Years ?? obj.vision ?? ""),
    techStack: String(obj.techStack ?? obj.stack ?? ""),
  });
}

/**
 * Rehidrata el perfil de coaching del alumno autenticado (GET).
 */
export async function fetchUserProfile(): Promise<FetchUserProfileResult> {
  const token = getStoredToken();
  if (!token) {
    return {
      ok: false,
      status: 401,
      message: "Sesión no iniciada.",
    };
  }

  let res: Response;
  try {
    res = await fetch(`${API_BASE_URL}/api/user/profile`, {
      method: "GET",
      headers: {
        Accept: "application/json",
        Authorization: `Bearer ${token}`,
      },
    });
  } catch {
    return {
      ok: false,
      status: 0,
      message: "No pudimos conectar con el servidor. Revisá la conexión.",
    };
  }

  if (!res.ok) {
    return {
      ok: false,
      status: res.status,
      message: await readApiError(res, "No pudimos cargar el perfil"),
    };
  }

  let profile: UserProfilePayload;
  try {
    profile = parseProfileBody(await res.json());
  } catch {
    return {
      ok: false,
      status: res.status,
      message: "El servidor devolvió un perfil con formato inválido.",
    };
  }

  try {
    if (typeof localStorage !== "undefined" && !isUserProfileEmpty(profile)) {
      localStorage.setItem("ppi.learner.profile", JSON.stringify(profile));
    }
  } catch {
    /* private mode */
  }
  return {
    ok: true,
    profile,
    empty: isUserProfileEmpty(profile),
  };
}

/**
 * Persiste el perfil de coaching del alumno autenticado.
 * Nunca lanza: errores tipados vía `{ ok: false }`.
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

  const body = normalizeUserProfile(payload);
  if (isUserProfileEmpty(body)) {
    return {
      ok: false,
      status: 400,
      message:
        "El perfil está vacío. Completá al menos un campo antes de guardar.",
    };
  }

  let res: Response;
  try {
    res = await fetch(`${API_BASE_URL}/api/user/profile`, {
      method: "PUT",
      headers: {
        Accept: "application/json",
        "Content-Type": "application/json",
        Authorization: `Bearer ${token}`,
      },
      body: JSON.stringify(body),
    });
  } catch {
    return {
      ok: false,
      status: 0,
      message: "No pudimos conectar con el servidor. Revisá la conexión.",
    };
  }

  if (res.status !== 200) {
    return {
      ok: false,
      status: res.status,
      message: await readApiError(res, "No pudimos guardar el perfil"),
    };
  }

  let profile: UserProfilePayload;
  try {
    const text = await res.text();
    profile = text.trim()
      ? parseProfileBody(JSON.parse(text) as unknown)
      : body;
  } catch {
    // Persistió (200) pero el body no parsea: tratamos el request como fuente de verdad.
    profile = body;
  }

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
