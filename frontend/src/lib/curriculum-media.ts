/** Metadatos multimedia opcionales alineados al contrato Go (Concept). */

export type MediaLocale = "es" | "en";

export type TranscriptSegment = {
  start_sec: number;
  end_sec: number;
  text: string;
};

export type MediaResource = {
  resource_url: string;
  transcript: TranscriptSegment[];
};

export type ConceptMedia = {
  id: string;
  title: string;
  summary: string;
  track?: string;
  /** Legado: un solo video/transcript (se interpreta como "es"). */
  resource_url?: string;
  transcript?: TranscriptSegment[];
  /** Preferido: mapa idioma → recurso. */
  resources?: Record<string, MediaResource>;
};

/** Contexto listo para que la tutora IA consuma el segmento activo (fase posterior). */
export type TutorTranscriptContext = {
  conceptId: string;
  locale: string;
  activeSegment: TranscriptSegment | null;
  currentTimeSec: number;
};

export function extractYouTubeId(resourceUrl: string): string | null {
  try {
    const url = new URL(resourceUrl);
    if (url.hostname.includes("youtu.be")) {
      return url.pathname.replace("/", "") || null;
    }
    const id = url.searchParams.get("v");
    return id;
  } catch {
    return null;
  }
}

export function activeTranscriptSegment(
  transcript: TranscriptSegment[],
  atSec: number,
): TranscriptSegment | null {
  if (!transcript.length) {
    return null;
  }
  for (const seg of transcript) {
    if (atSec >= seg.start_sec && atSec < seg.end_sec) {
      return seg;
    }
  }
  const last = transcript[transcript.length - 1];
  if (atSec >= last.end_sec) {
    return last;
  }
  return null;
}

export function availableMediaLocales(concept: ConceptMedia): MediaLocale[] {
  const out: MediaLocale[] = [];
  const seen = new Set<string>();
  const add = (lang: string) => {
    const key = lang.toLowerCase();
    if (key !== "es" && key !== "en") {
      return;
    }
    if (seen.has(key)) {
      return;
    }
    seen.add(key);
    out.push(key);
  };

  for (const [lang, media] of Object.entries(concept.resources ?? {})) {
    if (media?.resource_url?.trim()) {
      add(lang);
    }
  }
  if (concept.resource_url?.trim()) {
    add("es");
  }
  return out.length ? out : ["es"];
}

/** Resuelve media por idioma con fallback legado y entre locales. */
export function resolveMedia(
  concept: ConceptMedia,
  lang: string,
): MediaResource | null {
  const locale = (lang || "es").toLowerCase();
  const fromMap = concept.resources?.[locale];
  if (fromMap?.resource_url?.trim()) {
    return {
      resource_url: fromMap.resource_url,
      transcript: fromMap.transcript ?? [],
    };
  }

  if (locale === "es" && concept.resource_url?.trim()) {
    return {
      resource_url: concept.resource_url,
      transcript: concept.transcript ?? [],
    };
  }

  for (const fallback of ["es", "en"] as const) {
    if (fallback === locale) {
      continue;
    }
    const alt = concept.resources?.[fallback];
    if (alt?.resource_url?.trim()) {
      return {
        resource_url: alt.resource_url,
        transcript: alt.transcript ?? [],
      };
    }
    if (fallback === "es" && concept.resource_url?.trim()) {
      return {
        resource_url: concept.resource_url,
        transcript: concept.transcript ?? [],
      };
    }
  }

  return null;
}

export function buildTutorTranscriptContext(
  conceptId: string,
  locale: string,
  transcript: TranscriptSegment[],
  currentTimeSec: number,
): TutorTranscriptContext {
  return {
    conceptId,
    locale,
    activeSegment: activeTranscriptSegment(transcript, currentTimeSec),
    currentTimeSec,
  };
}

/** Seed Module 1 bilingüe (espejo de curriculum.json). */
export const MODULE1_STAGE_SEED: ConceptMedia = {
  id: "concept:string-literals",
  title: "Literales de texto (strings)",
  summary:
    "Cómo se escriben textos en código: comillas, escapes y errores típicos de sintaxis.",
  track: "python",
  resource_url: "https://www.youtube.com/watch?v=QJeGxd8biVA",
  transcript: [
    {
      start_sec: 0,
      end_sec: 15,
      text: "Bienvenida al Módulo 1 — Declarative Foundations: vamos a leer y escribir literales de texto en Python.",
    },
    {
      start_sec: 15,
      end_sec: 35,
      text: 'Un string se declara entre comillas simples o dobles. Ejemplo: mensaje = "hola".',
    },
    {
      start_sec: 35,
      end_sec: 55,
      text: "Los escapes como \\n permiten saltos de línea; un error típico es olvidar cerrar las comillas.",
    },
    {
      start_sec: 55,
      end_sec: 80,
      text: "Práctica: declara un literal, asígnalo a una variable y muéstralo con print().",
    },
  ],
  resources: {
    es: {
      resource_url: "https://www.youtube.com/watch?v=QJeGxd8biVA",
      transcript: [
        {
          start_sec: 0,
          end_sec: 15,
          text: "Bienvenida al Módulo 1 — Declarative Foundations: vamos a leer y escribir literales de texto en Python.",
        },
        {
          start_sec: 15,
          end_sec: 35,
          text: 'Un string se declara entre comillas simples o dobles. Ejemplo: mensaje = "hola".',
        },
        {
          start_sec: 35,
          end_sec: 55,
          text: "Los escapes como \\n permiten saltos de línea; un error típico es olvidar cerrar las comillas.",
        },
        {
          start_sec: 55,
          end_sec: 80,
          text: "Práctica: declara un literal, asígnalo a una variable y muéstralo con print().",
        },
      ],
    },
    en: {
      resource_url: "https://www.youtube.com/watch?v=kqtD5dpn9C8",
      transcript: [
        {
          start_sec: 0,
          end_sec: 15,
          text: "Welcome to Module 1 — Declarative Foundations: we will read and write text literals in Python.",
        },
        {
          start_sec: 15,
          end_sec: 35,
          text: 'A string is declared with single or double quotes. Example: message = "hello".',
        },
        {
          start_sec: 35,
          end_sec: 55,
          text: "Escapes like \\n create new lines; a common mistake is forgetting to close the quotes.",
        },
        {
          start_sec: 55,
          end_sec: 80,
          text: "Practice: declare a literal, bind it to a variable, and show it with print().",
        },
      ],
    },
  },
};

export function formatTimestamp(totalSec: number): string {
  const sec = Math.max(0, Math.floor(totalSec));
  const m = Math.floor(sec / 60);
  const s = sec % 60;
  return `${m}:${s.toString().padStart(2, "0")}`;
}
