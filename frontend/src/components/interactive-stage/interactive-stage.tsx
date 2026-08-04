import {
  component$,
  useSignal,
  useTask$,
  useVisibleTask$,
} from "@builder.io/qwik";
import {
  MODULE1_STAGE_SEED,
  activeTranscriptSegment,
  availableMediaLocales,
  buildTutorTranscriptContext,
  extractYouTubeId,
  resolveMedia,
  type ConceptMedia,
  type MediaLocale,
  type TutorTranscriptContext,
} from "../../lib/curriculum-media";
import { TranscriptPanel } from "./transcript-panel";
import { YouTubePlayer } from "./youtube-player";

export type InteractiveStageProps = {
  concept?: ConceptMedia;
};

export const InteractiveStage = component$<InteractiveStageProps>((props) => {
  const concept = props.concept ?? MODULE1_STAGE_SEED;
  const locales = availableMediaLocales(concept);
  const initialLocale: MediaLocale = locales.includes("es") ? "es" : locales[0];

  const locale = useSignal<MediaLocale>(initialLocale);
  const currentTime = useSignal(0);
  const seekRequest = useSignal<number | null>(null);

  const media = resolveMedia(concept, locale.value);
  const transcript = media?.transcript ?? [];
  const videoId = media ? (extractYouTubeId(media.resource_url) ?? "") : "";
  const hasTranscript = transcript.length > 0;

  const tutorContext = useSignal<TutorTranscriptContext>(
    buildTutorTranscriptContext(concept.id, initialLocale, transcript, 0),
  );

  useTask$(({ track }) => {
    const t = track(() => currentTime.value);
    const lang = track(() => locale.value);
    const resolved = resolveMedia(concept, lang);
    tutorContext.value = buildTutorTranscriptContext(
      concept.id,
      lang,
      resolved?.transcript ?? [],
      t,
    );
  });

  // eslint-disable-next-line qwik/no-use-visible-task
  useVisibleTask$(({ track }) => {
    const ctx = track(() => tutorContext.value);
    (
      window as Window & {
        __ppiTutorTranscriptContext?: TutorTranscriptContext;
      }
    ).__ppiTutorTranscriptContext = ctx;
  });

  const active = activeTranscriptSegment(transcript, currentTime.value);

  if (!media || !videoId) {
    return (
      <section class="interactive-stage interactive-stage--empty" role="status">
        <p>Este concepto aún no tiene un recurso de video válido.</p>
      </section>
    );
  }

  return (
    <section
      class="interactive-stage"
      aria-labelledby="interactive-stage-title"
      data-concept-id={concept.id}
      data-media-locale={locale.value}
      data-tutor-context="ready"
    >
      <header class="interactive-stage__header">
        <div class="interactive-stage__header-row">
          <p class="interactive-stage__eyebrow">
            Module 1 · Declarative Foundations
          </p>
          <div
            class="lang-toggle"
            role="group"
            aria-label="Idioma del video y la transcripción"
          >
            {(["es", "en"] as const).map((lang) => {
              const enabled = locales.includes(lang);
              const selected = locale.value === lang;
              return (
                <button
                  key={lang}
                  type="button"
                  class={`lang-toggle__btn${selected ? " lang-toggle__btn--active" : ""}`}
                  disabled={!enabled}
                  aria-pressed={selected}
                  onClick$={() => {
                    if (locale.value === lang) {
                      return;
                    }
                    locale.value = lang;
                    currentTime.value = 0;
                    seekRequest.value = 0;
                  }}
                >
                  {lang.toUpperCase()}
                </button>
              );
            })}
          </div>
        </div>
        <h2 id="interactive-stage-title" class="interactive-stage__title">
          {concept.title}
        </h2>
        <p class="interactive-stage__summary">{concept.summary}</p>
      </header>

      <div class="interactive-stage__layout">
        <div class="interactive-stage__media">
          <YouTubePlayer
            videoId={videoId}
            playerDomId="ppi-m01-yt-player"
            seekRequest={seekRequest.value}
            onTimeUpdate$={(seconds) => {
              currentTime.value = seconds;
              if (seekRequest.value != null) {
                seekRequest.value = null;
              }
            }}
          />
        </div>

        {hasTranscript ? (
          <TranscriptPanel
            segments={transcript}
            activeStartSec={active?.start_sec ?? null}
            onSeek$={(startSec) => {
              seekRequest.value = startSec;
              currentTime.value = startSec;
            }}
          />
        ) : (
          <aside class="transcript-panel transcript-panel--empty" role="status">
            <header class="transcript-panel__head">
              <h3 class="transcript-panel__title">Transcripción</h3>
            </header>
            <p class="transcript-panel__missing">
              No hay una transcripción detallada disponible en{" "}
              <strong>{locale.value.toUpperCase()}</strong> para este concepto.
              Podés cambiar de idioma o continuar con el video.
            </p>
          </aside>
        )}
      </div>

      <p class="interactive-stage__tutor-bridge" aria-live="polite">
        [{locale.value.toUpperCase()}] Segmento activo para tutora:{" "}
        {tutorContext.value.activeSegment?.text ??
          (hasTranscript
            ? "reproducí el video para sincronizar el contexto"
            : "sin transcripción en este idioma")}
      </p>
    </section>
  );
});
