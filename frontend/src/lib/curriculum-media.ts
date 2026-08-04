/** Metadatos multimedia opcionales alineados al contrato Go (Concept). */

export type TranscriptSegment = {
  start_sec: number;
  end_sec: number;
  text: string;
};

export type ConceptMedia = {
  id: string;
  title: string;
  summary: string;
  track?: string;
  resource_url: string;
  transcript: TranscriptSegment[];
};

/** Contexto listo para que la tutora IA consuma el segmento activo (fase posterior). */
export type TutorTranscriptContext = {
  conceptId: string;
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

export function buildTutorTranscriptContext(
  conceptId: string,
  transcript: TranscriptSegment[],
  currentTimeSec: number,
): TutorTranscriptContext {
  return {
    conceptId,
    activeSegment: activeTranscriptSegment(transcript, currentTimeSec),
    currentTimeSec,
  };
}

/** Seed Module 1 (espejo de curriculum.json) para InteractiveStage sin endpoint aún. */
export const MODULE1_STAGE_SEED: ConceptMedia = {
  id: "concept:string-literals",
  title: "Literales de texto (strings)",
  summary:
    "Cómo se escriben textos en código: comillas, escapes y errores típicos de sintaxis.",
  track: "python",
  resource_url: "https://www.youtube.com/watch?v=kqtD5dpn9C8",
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
};

export function formatTimestamp(totalSec: number): string {
  const sec = Math.max(0, Math.floor(totalSec));
  const m = Math.floor(sec / 60);
  const s = sec % 60;
  return `${m}:${s.toString().padStart(2, "0")}`;
}
