import {
  $,
  component$,
  useSignal,
  useVisibleTask$,
  type QRL,
} from "@builder.io/qwik";
import {
  composeSpeechNotes,
  getSpeechRecognitionConstructor,
  voiceErrorMessage,
  type SpeechRecognitionLike,
} from "./speech-recognition";

export type CoachingInteractionState = "drafting" | "reviewing" | "saved";

export type CoachingInterfaceProps = {
  prompts: string[];
  notes: string;
  interactionState: CoachingInteractionState;
  isAnalyzing?: boolean;
  analyzeError?: string;
  onNotesChange$: QRL<(value: string) => void>;
  onAnalyze$: QRL<() => void>;
  onRedesign$: QRL<() => void>;
};

const TEXTAREA_MIN_PX = 96;
const MIN_NOTES_FOR_ANALYZE = 12;

function resizeTextarea(el: HTMLTextAreaElement | undefined | null): void {
  if (!el) {
    return;
  }
  el.style.height = "auto";
  el.style.height = `${Math.max(el.scrollHeight, TEXTAREA_MIN_PX)}px`;
}

/**
 * Entrevista Rogeriana (onboarding): drafting → reviewing → saved.
 * SpeechRecognition solo en cliente (useVisibleTask$) para no romper SSR.
 */
export const CoachingInterface = component$((props: CoachingInterfaceProps) => {
  const isListening = useSignal(false);
  const voiceSupported = useSignal(false);
  const voiceError = useSignal("");
  const recognitionRef = useSignal<SpeechRecognitionLike | null>(null);
  const sessionBase = useSignal("");
  const finalTranscript = useSignal("");
  const textareaRef = useSignal<HTMLTextAreaElement>();

  const isDrafting = props.interactionState === "drafting";
  const isAnalyzing = Boolean(props.isAnalyzing);
  const canAnalyze =
    isDrafting &&
    !isAnalyzing &&
    props.notes.trim().length >= MIN_NOTES_FOR_ANALYZE;

  // eslint-disable-next-line qwik/no-use-visible-task -- scrollHeight requiere DOM
  useVisibleTask$(({ track }) => {
    track(() => props.notes);
    track(() => props.interactionState);
    resizeTextarea(textareaRef.value);
  });

  // eslint-disable-next-line qwik/no-use-visible-task -- Web Speech API es browser-only
  useVisibleTask$(({ cleanup }) => {
    const Ctor = getSpeechRecognitionConstructor();
    if (!Ctor) {
      voiceSupported.value = false;
      return;
    }

    const recognition = new Ctor();
    recognition.continuous = true;
    recognition.interimResults = true;
    recognition.lang = "es-AR";

    recognition.onresult = (event) => {
      let interim = "";
      for (let i = event.resultIndex; i < event.results.length; i++) {
        const result = event.results[i];
        const piece = result[0]?.transcript ?? "";
        if (result.isFinal) {
          finalTranscript.value += piece;
        } else {
          interim += piece;
        }
      }
      props
        .onNotesChange$(
          composeSpeechNotes(
            sessionBase.value,
            finalTranscript.value,
            interim,
          ),
        )
        .catch(() => undefined);
    };

    recognition.onerror = (event) => {
      isListening.value = false;
      const msg = voiceErrorMessage(event.error);
      if (msg) {
        voiceError.value = msg;
      }
    };

    recognition.onend = () => {
      isListening.value = false;
    };

    recognitionRef.value = recognition;
    voiceSupported.value = true;

    cleanup(() => {
      try {
        recognition.onresult = null;
        recognition.onerror = null;
        recognition.onend = null;
        recognition.abort();
      } catch {
        /* already stopped */
      }
      recognitionRef.value = null;
      isListening.value = false;
    });
  });

  const stopListening$ = $(() => {
    const recognition = recognitionRef.value;
    if (!recognition || !isListening.value) {
      return;
    }
    try {
      recognition.stop();
    } catch {
      /* ignore */
    }
    isListening.value = false;
  });

  const toggleListening$ = $(async () => {
    if (!isDrafting) {
      return;
    }
    const recognition = recognitionRef.value;
    if (!recognition) {
      return;
    }
    voiceError.value = "";

    if (isListening.value) {
      await stopListening$();
      return;
    }

    sessionBase.value = textareaRef.value?.value ?? "";
    finalTranscript.value = "";
    try {
      recognition.start();
      isListening.value = true;
    } catch {
      isListening.value = false;
      voiceError.value =
        "No se pudo iniciar el micrófono. Esperá un segundo e intentá de nuevo.";
    }
  });

  const prompts =
    props.prompts.length > 0
      ? props.prompts
      : [
          "¿Qué te trae a aprender Python ahora?",
          "¿Con qué urgencia necesitás estos resultados?",
          "¿Qué visión tenés a 5 años?",
          "¿Qué entornos conocés? (Jupyter, Cursor, Positron…)",
        ];

  return (
    <section class="coaching" aria-label="Entrevista de propósito">
      <p class="coaching__eyebrow">Paso 1 · Coaching</p>
      <h2 class="coaching__title">Hola, ¿cómo estás?</h2>
      <p class="coaching__lead">
        Antes de escribir una sola línea de código, quiero conocerte. Contame
        con tus palabras: qué te motiva, qué tan urgente es, hacia dónde vas y
        con qué herramientas ya te sentís a gusto.
      </p>

      <ol class="coaching__prompts">
        {prompts.map((q, i) => (
          <li key={`q-${i}`} class="coaching__prompt">
            {q}
          </li>
        ))}
      </ol>

      <div class="coaching__label-row">
        <label class="coaching__label" for="coaching-notes">
          Tu respuesta
        </label>
        {isDrafting && voiceSupported.value && (
          <span class="coaching__voice-status" aria-live="polite">
            {isListening.value ? "Escuchando…" : "Podés escribir o dictar"}
          </span>
        )}
        {!isDrafting && (
          <span class="coaching__voice-status" aria-live="polite">
            {props.interactionState === "saved"
              ? "Respuesta guardada"
              : "En revisión"}
          </span>
        )}
      </div>

      <div
        class={`coaching__composer${isListening.value && isDrafting ? " coaching__composer--listening" : ""}${!isDrafting ? " coaching__composer--readonly" : ""}`}
      >
        <textarea
          ref={textareaRef}
          id="coaching-notes"
          class="coaching__textarea"
          rows={3}
          readOnly={!isDrafting}
          placeholder="Escribí o dictá libremente… Por ejemplo: quiero automatizar reportes en el trabajo; a 5 años me veo liderando análisis de datos."
          value={props.notes}
          onInput$={(_, el) => {
            if (!isDrafting) {
              return;
            }
            const value = el.value;
            props.onNotesChange$(value);
            resizeTextarea(el);
            if (isListening.value) {
              sessionBase.value = value;
              finalTranscript.value = "";
            }
          }}
        />
        {isDrafting && voiceSupported.value && (
          <button
            type="button"
            class={`coaching__mic${isListening.value ? " coaching__mic--listening" : ""}`}
            aria-pressed={isListening.value}
            aria-label={
              isListening.value
                ? "Detener dictado por voz"
                : "Dictar respuesta por voz"
            }
            title={
              isListening.value ? "Detener escucha" : "Dictar con el micrófono"
            }
            onClick$={toggleListening$}
          >
            <svg
              class="coaching__mic-icon"
              viewBox="0 0 24 24"
              width="20"
              height="20"
              aria-hidden="true"
              focusable="false"
            >
              <path
                fill="currentColor"
                d="M12 14a3 3 0 0 0 3-3V6a3 3 0 1 0-6 0v5a3 3 0 0 0 3 3zm5-3a5 5 0 0 1-10 0H5a7 7 0 0 0 6 6.92V21h2v-3.08A7 7 0 0 0 19 11h-2z"
              />
            </svg>
            {isListening.value && <span class="coaching__mic-pulse" />}
          </button>
        )}
      </div>

      {voiceError.value && isDrafting && (
        <p class="coaching__voice-error" role="status">
          {voiceError.value}
        </p>
      )}

      <div class="coaching__actions">
        {isDrafting ? (
          <>
            <button
              type="button"
              class="exercise-ws__btn exercise-ws__btn--primary"
              disabled={!canAnalyze}
              title={
                isAnalyzing
                  ? "Analizando con Gemini…"
                  : canAnalyze
                    ? "Analizar tu respuesta"
                    : "Escribí un poco más para enviar"
              }
              onClick$={async () => {
                if (!canAnalyze) {
                  return;
                }
                await stopListening$();
                await props.onAnalyze$();
              }}
            >
              {isAnalyzing ? "Analizando…" : "Enviar para análisis"}
            </button>
            {props.analyzeError && (
              <p class="coaching__voice-error" role="alert">
                {props.analyzeError}
              </p>
            )}
            {!canAnalyze && !isAnalyzing && !props.analyzeError && (
              <p class="coaching__continue-hint" role="status">
                Contanos un poco más (unas líneas) para analizar tu perfil.
              </p>
            )}
          </>
        ) : (
          <button
            type="button"
            class="exercise-ws__btn"
            onClick$={async () => {
              await props.onRedesign$();
            }}
          >
            Rediseñar / Mejorar mi respuesta
          </button>
        )}
      </div>
    </section>
  );
});
