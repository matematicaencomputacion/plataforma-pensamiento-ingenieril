/** Metadatos multimedia opcionales alineados al contrato Go (Concept). */

export type MediaLocale = "es" | "en";

export type TranscriptSegment = {
  start_sec: number;
  end_sec: number;
  text: string;
};

/** Capítulos del arnés (JSON snake_case alineado al dominio Go). */
export type MediaChapter = {
  id: string;
  title: string;
  start_sec: number;
  end_sec: number;
  transcript?: TranscriptSegment[];
  /** Vincula el bloque con el ejercicio/concepto práctico del editor. */
  exercise_ref?: string;
};

export type MediaResource = {
  resource_url: string;
  transcript?: TranscriptSegment[];
  chapters?: MediaChapter[];
  /** Alias opcional del backend. */
  topics?: MediaChapter[];
};

export type ConceptMedia = {
  id: string;
  title: string;
  summary: string;
  track?: string;
  resource_url?: string;
  transcript?: TranscriptSegment[];
  resources?: Record<string, MediaResource>;
};

export type TutorTranscriptContext = {
  conceptId: string;
  locale: string;
  activeSegment: TranscriptSegment | null;
  currentTimeSec: number;
  activeChapterId: string | null;
  activeChapterTitle: string | null;
  exerciseRef: string | null;
};

export type PedTopicContext = {
  conceptId: string;
  locale: string;
  chapterId: string | null;
  chapterTitle: string | null;
  exerciseRef: string | null;
  currentTimeSec: number;
};

export { extractYouTubeVideoId as extractYouTubeId } from "./youtube-utils";

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

export function normalizeChapters(media: MediaResource | null): MediaChapter[] {
  if (!media) {
    return [];
  }
  return media.chapters?.length ? media.chapters : (media.topics ?? []);
}

export function chapterAt(
  chapters: MediaChapter[],
  atSec: number,
): MediaChapter | null {
  if (!chapters.length) {
    return null;
  }
  for (const ch of chapters) {
    if (atSec >= ch.start_sec && atSec < ch.end_sec) {
      return ch;
    }
  }
  const last = chapters[chapters.length - 1];
  if (atSec >= last.end_sec) {
    return last;
  }
  return null;
}

export function transcriptForChapter(
  media: MediaResource,
  chapter: MediaChapter,
): TranscriptSegment[] {
  if (chapter.transcript?.length) {
    return chapter.transcript;
  }
  const all = media.transcript ?? [];
  return all.filter(
    (seg) => seg.start_sec >= chapter.start_sec && seg.start_sec < chapter.end_sec,
  );
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
      chapters: normalizeChapters(fromMap),
    };
  }

  if (locale === "es" && concept.resource_url?.trim()) {
    return {
      resource_url: concept.resource_url,
      transcript: concept.transcript ?? [],
      chapters: [],
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
        chapters: normalizeChapters(alt),
      };
    }
    if (fallback === "es" && concept.resource_url?.trim()) {
      return {
        resource_url: concept.resource_url,
        transcript: concept.transcript ?? [],
        chapters: [],
      };
    }
  }

  return null;
}

export function buildTutorTranscriptContext(input: {
  conceptId: string;
  locale: string;
  transcript: TranscriptSegment[];
  currentTimeSec: number;
  chapter: MediaChapter | null;
}): TutorTranscriptContext {
  return {
    conceptId: input.conceptId,
    locale: input.locale,
    activeSegment: activeTranscriptSegment(
      input.transcript,
      input.currentTimeSec,
    ),
    currentTimeSec: input.currentTimeSec,
    activeChapterId: input.chapter?.id ?? null,
    activeChapterTitle: input.chapter?.title ?? null,
    exerciseRef: input.chapter?.exercise_ref ?? input.conceptId,
  };
}

const MOURE_CHAPTERS: MediaChapter[] = [
  {
    id: "ch-01-introduccion",
    title: "Capítulo 1: Introducción",
    start_sec: 0,
    end_sec: 244,
    exercise_ref: "concept:string-literals",
    transcript: [
      {
        start_sec: 0.0,
        end_sec: 81.0,
        text: "Inicio de Capítulo 1: Introducción.",
      },
      {
        start_sec: 81.0,
        end_sec: 244.0,
        text: "Cierre y práctica del bloque: Capítulo 1: Introducción.",
      },
    ],
  },
  {
    id: "ch-02-contexto",
    title: "Capítulo 2: Contexto",
    start_sec: 244,
    end_sec: 850,
    exercise_ref: "concept:string-literals",
    transcript: [
      {
        start_sec: 244.0,
        end_sec: 446.0,
        text: "Inicio de Capítulo 2: Contexto.",
      },
      {
        start_sec: 446.0,
        end_sec: 850.0,
        text: "Cierre y práctica del bloque: Capítulo 2: Contexto.",
      },
    ],
  },
  {
    id: "ch-03-configuracion",
    title: "Capítulo 3: 01 - Configuración",
    start_sec: 850,
    end_sec: 1518,
    exercise_ref: "concept:string-literals",
    transcript: [
      {
        start_sec: 850.0,
        end_sec: 1072.0,
        text: "Inicio de Capítulo 3: 01 - Configuración.",
      },
      {
        start_sec: 1072.0,
        end_sec: 1518.0,
        text: "Cierre y práctica del bloque: Capítulo 3: 01 - Configuración.",
      },
    ],
  },
  {
    id: "ch-04-hola-mundo",
    title: "Capítulo 4: 02 - Hola Mundo",
    start_sec: 1518,
    end_sec: 2938,
    exercise_ref: "concept:string-literals",
    transcript: [
      {
        start_sec: 1518.0,
        end_sec: 1991.0,
        text: "Inicio de Capítulo 4: 02 - Hola Mundo.",
      },
      {
        start_sec: 1991.0,
        end_sec: 2938.0,
        text: "Cierre y práctica del bloque: Capítulo 4: 02 - Hola Mundo.",
      },
    ],
  },
  {
    id: "ch-05-variables",
    title: "Capítulo 5: 03 - Variables",
    start_sec: 2938,
    end_sec: 5665,
    exercise_ref: "concept:variables-scope",
    transcript: [
      {
        start_sec: 2938.0,
        end_sec: 3847.0,
        text: "Inicio de Capítulo 5: 03 - Variables.",
      },
      {
        start_sec: 3847.0,
        end_sec: 5665.0,
        text: "Cierre y práctica del bloque: Capítulo 5: 03 - Variables.",
      },
    ],
  },
  {
    id: "ch-06-operadores",
    title: "Capítulo 6: 04 - Operadores",
    start_sec: 5665,
    end_sec: 8645,
    exercise_ref: "concept:variables-scope",
    transcript: [
      {
        start_sec: 5665.0,
        end_sec: 6658.0,
        text: "Inicio de Capítulo 6: 04 - Operadores.",
      },
      {
        start_sec: 6658.0,
        end_sec: 8645.0,
        text: "Cierre y práctica del bloque: Capítulo 6: 04 - Operadores.",
      },
    ],
  },
  {
    id: "ch-07-strings",
    title: "Capítulo 7: 05 - Strings",
    start_sec: 8645,
    end_sec: 10875,
    exercise_ref: "concept:string-literals",
    transcript: [
      {
        start_sec: 8645.0,
        end_sec: 9388.0,
        text: "Inicio de Capítulo 7: 05 - Strings.",
      },
      {
        start_sec: 9388.0,
        end_sec: 10875.0,
        text: "Cierre y práctica del bloque: Capítulo 7: 05 - Strings.",
      },
    ],
  },
  {
    id: "ch-08-listas",
    title: "Capítulo 8: 06 - Listas",
    start_sec: 10875,
    end_sec: 14711,
    exercise_ref: "concept:variables-scope",
    transcript: [
      {
        start_sec: 10875.0,
        end_sec: 12153.0,
        text: "Inicio de Capítulo 8: 06 - Listas.",
      },
      {
        start_sec: 12153.0,
        end_sec: 14711.0,
        text: "Cierre y práctica del bloque: Capítulo 8: 06 - Listas.",
      },
    ],
  },
  {
    id: "ch-09-tuplas",
    title: "Capítulo 9: 07 - Tuplas",
    start_sec: 14711,
    end_sec: 16335,
    exercise_ref: "concept:variables-scope",
    transcript: [
      {
        start_sec: 14711.0,
        end_sec: 15252.0,
        text: "Inicio de Capítulo 9: 07 - Tuplas.",
      },
      {
        start_sec: 15252.0,
        end_sec: 16335.0,
        text: "Cierre y práctica del bloque: Capítulo 9: 07 - Tuplas.",
      },
    ],
  },
  {
    id: "ch-10-sets",
    title: "Capítulo 10: 08 - Sets",
    start_sec: 16335,
    end_sec: 18507,
    exercise_ref: "concept:variables-scope",
    transcript: [
      {
        start_sec: 16335.0,
        end_sec: 17059.0,
        text: "Inicio de Capítulo 10: 08 - Sets.",
      },
      {
        start_sec: 17059.0,
        end_sec: 18507.0,
        text: "Cierre y práctica del bloque: Capítulo 10: 08 - Sets.",
      },
    ],
  },
  {
    id: "ch-11-diccionarios",
    title: "Capítulo 11: 09 - Diccionarios",
    start_sec: 18507,
    end_sec: 21442,
    exercise_ref: "concept:variables-scope",
    transcript: [
      {
        start_sec: 18507.0,
        end_sec: 19485.0,
        text: "Inicio de Capítulo 11: 09 - Diccionarios.",
      },
      {
        start_sec: 19485.0,
        end_sec: 21442.0,
        text: "Cierre y práctica del bloque: Capítulo 11: 09 - Diccionarios.",
      },
    ],
  },
  {
    id: "ch-12-condicionales",
    title: "Capítulo 12: 10 - Condicionales",
    start_sec: 21442,
    end_sec: 23822,
    exercise_ref: "concept:variables-scope",
    transcript: [
      {
        start_sec: 21442.0,
        end_sec: 22235.0,
        text: "Inicio de Capítulo 12: 10 - Condicionales.",
      },
      {
        start_sec: 22235.0,
        end_sec: 23822.0,
        text: "Cierre y práctica del bloque: Capítulo 12: 10 - Condicionales.",
      },
    ],
  },
  {
    id: "ch-13-bucles",
    title: "Capítulo 13: 11 - Bucles/Loops/Ciclos",
    start_sec: 23822,
    end_sec: 26619,
    exercise_ref: "concept:variables-scope",
    transcript: [
      {
        start_sec: 23822.0,
        end_sec: 24754.0,
        text: "Inicio de Capítulo 13: 11 - Bucles/Loops/Ciclos.",
      },
      {
        start_sec: 24754.0,
        end_sec: 26619.0,
        text: "Cierre y práctica del bloque: Capítulo 13: 11 - Bucles/Loops/Ciclos.",
      },
    ],
  },
  {
    id: "ch-14-funciones",
    title: "Capítulo 14: 12 - Funciones",
    start_sec: 26619,
    end_sec: 29327,
    exercise_ref: "concept:function-parameters",
    transcript: [
      {
        start_sec: 26619.0,
        end_sec: 27521.0,
        text: "Inicio de Capítulo 14: 12 - Funciones.",
      },
      {
        start_sec: 27521.0,
        end_sec: 29327.0,
        text: "Cierre y práctica del bloque: Capítulo 14: 12 - Funciones.",
      },
    ],
  },
  {
    id: "ch-15-clases",
    title: "Capítulo 15: 13 - Clases",
    start_sec: 29327,
    end_sec: 32236,
    exercise_ref: "concept:function-parameters",
    transcript: [
      {
        start_sec: 29327.0,
        end_sec: 30296.0,
        text: "Inicio de Capítulo 15: 13 - Clases.",
      },
      {
        start_sec: 30296.0,
        end_sec: 32236.0,
        text: "Cierre y práctica del bloque: Capítulo 15: 13 - Clases.",
      },
    ],
  },
  {
    id: "ch-16-excepciones",
    title: "Capítulo 16: 14 - Excepciones",
    start_sec: 32236,
    end_sec: 33983,
    exercise_ref: "concept:debug-variables",
    transcript: [
      {
        start_sec: 32236.0,
        end_sec: 32818.0,
        text: "Inicio de Capítulo 16: 14 - Excepciones.",
      },
      {
        start_sec: 32818.0,
        end_sec: 33983.0,
        text: "Cierre y práctica del bloque: Capítulo 16: 14 - Excepciones.",
      },
    ],
  },
  {
    id: "ch-17-modulos",
    title: "Capítulo 17: 15 - Módulos",
    start_sec: 33983,
    end_sec: 36391,
    exercise_ref: "concept:function-parameters",
    transcript: [
      {
        start_sec: 33983.0,
        end_sec: 34785.0,
        text: "Inicio de Capítulo 17: 15 - Módulos.",
      },
      {
        start_sec: 34785.0,
        end_sec: 36391.0,
        text: "Cierre y práctica del bloque: Capítulo 17: 15 - Módulos.",
      },
    ],
  },
  {
    id: "ch-18-proximos-pasos",
    title: "Capítulo 18: Próximos pasos",
    start_sec: 36391,
    end_sec: 36454,
    exercise_ref: "concept:string-literals",
    transcript: [
      {
        start_sec: 36391.0,
        end_sec: 36421.0,
        text: "Inicio de Capítulo 18: Próximos pasos.",
      },
      {
        start_sec: 36421.0,
        end_sec: 36454.0,
        text: "Cierre y práctica del bloque: Capítulo 18: Próximos pasos.",
      },
    ],
  },
];

/** Seed Module 1 con arnés de temas (ES/MoureDev) + EN corto. */
export const MODULE1_STAGE_SEED: ConceptMedia = {
  id: "concept:string-literals",
  title: "Literales de texto (strings)",
  summary:
    "Cómo se escriben textos en código: comillas, escapes y errores típicos de sintaxis.",
  track: "python",
  resource_url: "https://www.youtube.com/watch?v=Kp4Mvapo5kc",
  transcript: MOURE_CHAPTERS.flatMap((ch) => ch.transcript ?? []),
  resources: {
    es: {
      resource_url: "https://www.youtube.com/watch?v=Kp4Mvapo5kc",
      transcript: MOURE_CHAPTERS.flatMap((ch) => ch.transcript ?? []),
      chapters: MOURE_CHAPTERS,
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
