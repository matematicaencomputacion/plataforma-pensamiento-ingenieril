/** Síntesis de perfil (mock hasta LLM real). */

export type ProfileSynthesis = {
  purpose: string;
  urgency: string;
  vision: string;
  stack: string;
};

export const EMPTY_PROFILE_SYNTHESIS: ProfileSynthesis = {
  purpose: "",
  urgency: "",
  vision: "",
  stack: "",
};

export const PROFILE_WAITING_COPY = "Esperando para escucharte...";

const DEBOUNCE_MS = 800;

export const PROFILE_SYNTHESIS_DEBOUNCE_MS = DEBOUNCE_MS;

function normalizeForMatch(text: string): string {
  return text
    .toLowerCase()
    .normalize("NFD")
    .replace(/\p{M}/gu, "");
}

/**
 * Mock inteligente por keywords. Sustituible por llamada LLM sin cambiar el contrato UI.
 */
export function simulateAISynthesis(text: string): ProfileSynthesis {
  const raw = text.trim();
  if (!raw) {
    return { ...EMPTY_PROFILE_SYNTHESIS };
  }

  const t = normalizeForMatch(raw);
  const out: ProfileSynthesis = { ...EMPTY_PROFILE_SYNTHESIS };

  if (/\bestudiante\b/.test(t) || /\bpadres\b/.test(t)) {
    out.purpose = "Ayudar a su familia y ganar autonomía económica.";
  }

  if (/\brapido\b/.test(t) || /\burgencia\b/.test(t)) {
    out.urgency = "Extrema - Necesita resultados inmediatos.";
  }

  if (/\bno se\b/.test(t)) {
    out.vision = "Exploratoria. Buscando definir un camino sólido.";
  }

  const hasCoursera = /\bcoursera\b/.test(t);
  const deniesNotebookEnv =
    /no (conozco|se|use|usei|probado|prob[eé]).{0,48}(jupyter|colab)/.test(t) ||
    /(jupyter|colab).{0,48}no (conozco|se|use|usei|probado)/.test(t) ||
    /nunca.{0,24}(jupyter|colab)/.test(t) ||
    /sin (conocer|saber).{0,24}(jupyter|colab)/.test(t);

  if (hasCoursera || deniesNotebookEnv) {
    out.stack =
      "Coursera. Primer contacto con entornos como Jupyter/Colab.";
  }

  return out;
}

export function applyProfileSynthesis(
  target: ProfileSynthesis,
  next: ProfileSynthesis,
): void {
  target.purpose = next.purpose;
  target.urgency = next.urgency;
  target.vision = next.vision;
  target.stack = next.stack;
}
