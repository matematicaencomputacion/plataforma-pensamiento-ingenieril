import {
  component$,
  useSignal,
  useTask$,
  useVisibleTask$,
} from "@builder.io/qwik";
import {
  MODULE1_STAGE_SEED,
  activeTranscriptSegment,
  buildTutorTranscriptContext,
  extractYouTubeId,
  type ConceptMedia,
  type TutorTranscriptContext,
} from "../../lib/curriculum-media";
import { TranscriptPanel } from "./transcript-panel";
import { YouTubePlayer } from "./youtube-player";

export type InteractiveStageProps = {
  concept?: ConceptMedia;
};

export const InteractiveStage = component$<InteractiveStageProps>((props) => {
  const concept = props.concept ?? MODULE1_STAGE_SEED;
  const currentTime = useSignal(0);
  const seekRequest = useSignal<number | null>(null);
  const tutorContext = useSignal<TutorTranscriptContext>(
    buildTutorTranscriptContext(concept.id, concept.transcript, 0),
  );

  const videoId = extractYouTubeId(concept.resource_url) ?? "";

  useTask$(({ track }) => {
    const t = track(() => currentTime.value);
    tutorContext.value = buildTutorTranscriptContext(
      concept.id,
      concept.transcript,
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

  const active = activeTranscriptSegment(
    concept.transcript,
    currentTime.value,
  );

  if (!videoId) {
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
      data-tutor-context="ready"
    >
      <header class="interactive-stage__header">
        <p class="interactive-stage__eyebrow">
          Module 1 · Declarative Foundations
        </p>
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

        <TranscriptPanel
          segments={concept.transcript}
          activeStartSec={active?.start_sec ?? null}
          onSeek$={(startSec) => {
            seekRequest.value = startSec;
            currentTime.value = startSec;
          }}
        />
      </div>

      <p class="interactive-stage__tutor-bridge" aria-live="polite">
        Segmento activo para tutora:{" "}
        {tutorContext.value.activeSegment?.text ??
          "reproducí el video para sincronizar el contexto"}
      </p>
    </section>
  );
});
