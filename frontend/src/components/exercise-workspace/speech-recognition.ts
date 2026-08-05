/** Tipos mínimos de Web Speech API (no todos los browsers tipifican webkit*). */

export type SpeechRecognitionResultLike = {
  readonly isFinal: boolean;
  readonly 0: { transcript: string };
};

export type SpeechRecognitionEventLike = {
  readonly resultIndex: number;
  readonly results: ArrayLike<SpeechRecognitionResultLike>;
};

export type SpeechRecognitionErrorLike = {
  readonly error: string;
  readonly message?: string;
};

export type SpeechRecognitionLike = {
  continuous: boolean;
  interimResults: boolean;
  lang: string;
  start: () => void;
  stop: () => void;
  abort: () => void;
  onresult: ((event: SpeechRecognitionEventLike) => void) | null;
  onerror: ((event: SpeechRecognitionErrorLike) => void) | null;
  onend: (() => void) | null;
};

export type SpeechRecognitionConstructor = new () => SpeechRecognitionLike;

export function getSpeechRecognitionConstructor():
  | SpeechRecognitionConstructor
  | null {
  if (typeof window === "undefined") {
    return null;
  }
  const w = window as Window & {
    SpeechRecognition?: SpeechRecognitionConstructor;
    webkitSpeechRecognition?: SpeechRecognitionConstructor;
  };
  return w.SpeechRecognition ?? w.webkitSpeechRecognition ?? null;
}

/** Une base tipada + finales + interim sin duplicar espacios raros. */
export function composeSpeechNotes(
  base: string,
  finalAcc: string,
  interim: string,
): string {
  const spoken = `${finalAcc}${interim}`;
  if (!spoken) {
    return base;
  }
  if (!base) {
    return spoken.replace(/^\s+/, "");
  }
  if (/\s$/.test(base) || /^\s/.test(spoken)) {
    return base + spoken;
  }
  return `${base} ${spoken.replace(/^\s+/, "")}`;
}

export function voiceErrorMessage(code: string): string {
  switch (code) {
    case "not-allowed":
    case "service-not-allowed":
      return "No pudimos acceder al micrófono. Revisá los permisos del navegador.";
    case "no-speech":
      return "No detectamos habla. Tocá el micrófono e intentá de nuevo.";
    case "audio-capture":
      return "No encontramos un micrófono disponible.";
    case "network":
      return "Falló la conexión del reconocimiento de voz.";
    case "aborted":
      return "";
    default:
      return "Hubo un problema con el reconocimiento de voz. Probá de nuevo.";
  }
}
