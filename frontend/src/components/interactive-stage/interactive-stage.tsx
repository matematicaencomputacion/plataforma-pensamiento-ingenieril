import {
  component$,
  useSignal,
  useTask$,
  useVisibleTask$,
  type QRL,
} from "@builder.io/qwik";
import {
  MODULE1_STAGE_SEED,
  activeTranscriptSegment,
  availableMediaLocales,
  buildTutorTranscriptContext,
  chapterAt,
  normalizeChapters,
  resolveMedia,
  transcriptForChapter,
  type ConceptMedia,
  type MediaChapter,
  type MediaLocale,
  type PedTopicContext,
  type TutorTranscriptContext,
} from "../../lib/curriculum-media";
import { isValidYouTubeResource } from "../../lib/youtube-utils";
import { ResourceLinks } from "./resource-links";
import { TopicNavigator } from "./topic-navigator";
import { TranscriptPanel } from "./transcript-panel";
import { YouTubePlayer } from "./youtube-player";

export type InteractiveStageProps = {
  concept?: ConceptMedia;
  onTopicChange$?: QRL<(ctx: PedTopicContext) => void>;
};

export const InteractiveStage = component$<InteractiveStageProps>((props) => {
  const concept = props.concept ?? MODULE1_STAGE_SEED;
  const locales = availableMediaLocales(concept);
  const initialLocale: MediaLocale = locales.includes("es") ? "es" : locales[0];

  const locale = useSignal<MediaLocale>(initialLocale);
  const currentTime = useSignal(0);
  const seekRequest = useSignal<number | null>(null);
  const pinnedChapterId = useSignal<string | null>(null);

  const media = resolveMedia(concept, locale.value);
  const chapters = normalizeChapters(media);
  const resourceUrl = media?.resource_url ?? "";
  const hasValidVideo = isValidYouTubeResource(resourceUrl);

  const autoChapter = chapterAt(chapters, currentTime.value);
  const activeChapter: MediaChapter | null = (() => {
    if (!pinnedChapterId.value) {
      return autoChapter;
    }
    return chapters.find((ch) => ch.id === pinnedChapterId.value) ?? autoChapter;
  })();

  const visibleTranscript =
    media && activeChapter
      ? transcriptForChapter(media, activeChapter)
      : (media?.transcript ?? []);
  const hasTranscript = visibleTranscript.length > 0;
  const activeCue = activeTranscriptSegment(
    visibleTranscript,
    currentTime.value,
  );

  const tutorContext = useSignal<TutorTranscriptContext>(
    buildTutorTranscriptContext({
      conceptId: concept.id,
      locale: initialLocale,
      transcript: [],
      currentTimeSec: 0,
      chapter: null,
    }),
  );

  useTask$(({ track }) => {
    const t = track(() => currentTime.value);
    const lang = track(() => locale.value);
    const pinned = track(() => pinnedChapterId.value);
    const resolved = resolveMedia(concept, lang);
    const chs = normalizeChapters(resolved);
    const automatic = chapterAt(chs, t);
    const chapter = pinned
      ? (chs.find((c) => c.id === pinned) ?? automatic)
      : automatic;
    const transcript =
      resolved && chapter
        ? transcriptForChapter(resolved, chapter)
        : (resolved?.transcript ?? []);

    const next = buildTutorTranscriptContext({
      conceptId: concept.id,
      locale: lang,
      transcript,
      currentTimeSec: t,
      chapter,
    });
    tutorContext.value = next;

    void props.onTopicChange$?.({
      conceptId: concept.id,
      locale: lang,
      chapterId: chapter?.id ?? null,
      chapterTitle: chapter?.title ?? null,
      exerciseRef: chapter?.exercise_ref ?? concept.id,
      currentTimeSec: t,
    });
  });

  // eslint-disable-next-line qwik/no-use-visible-task
  useVisibleTask$(({ track }) => {
    const ctx = track(() => tutorContext.value);
    const win = window as Window & {
      __ppiTutorTranscriptContext?: TutorTranscriptContext;
      __ppiActiveTopic?: PedTopicContext;
    };
    win.__ppiTutorTranscriptContext = ctx;
    win.__ppiActiveTopic = {
      conceptId: ctx.conceptId,
      locale: ctx.locale,
      chapterId: ctx.activeChapterId,
      chapterTitle: ctx.activeChapterTitle,
      exerciseRef: ctx.exerciseRef,
      currentTimeSec: ctx.currentTimeSec,
    };
  });

  if (!media || !hasValidVideo) {
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
      data-active-chapter={activeChapter?.id ?? ""}
      data-exercise-ref={activeChapter?.exercise_ref ?? concept.id}
      data-tutor-context="ready"
    >
      <header class="interactive-stage__header">
        <div class="interactive-stage__header-row">
          <p class="interactive-stage__eyebrow">{concept.title}</p>
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
                    pinnedChapterId.value = null;
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

      <ResourceLinks />

      {chapters.length > 0 && (
        <TopicNavigator
          chapters={chapters}
          activeChapterId={activeChapter?.id ?? null}
          onSelect$={(chapterId) => {
            const selected = chapters.find((ch) => ch.id === chapterId);
            if (!selected) {
              return;
            }
            pinnedChapterId.value = selected.id;
            seekRequest.value = selected.start_sec;
            currentTime.value = selected.start_sec;
          }}
        />
      )}

      <div class="interactive-stage__layout">
        <div class="interactive-stage__media">
          <YouTubePlayer
            resourceUrl={resourceUrl}
            playerDomId="ppi-m01-yt-player"
            seekRequest={seekRequest.value}
            onTimeUpdate$={(seconds) => {
              currentTime.value = seconds;
              if (seekRequest.value != null) {
                seekRequest.value = null;
              }
              const pinned = pinnedChapterId.value;
              if (pinned) {
                const ch = chapters.find((c) => c.id === pinned);
                if (ch && (seconds < ch.start_sec || seconds >= ch.end_sec)) {
                  pinnedChapterId.value = null;
                }
              }
            }}
          />
        </div>

        {hasTranscript ? (
          <TranscriptPanel
            segments={visibleTranscript}
            activeStartSec={activeCue?.start_sec ?? null}
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
              No hay una transcripción detallada disponible
              {activeChapter
                ? ` para el tema “${activeChapter.title}”`
                : ` en ${locale.value.toUpperCase()}`}
              .
            </p>
          </aside>
        )}
      </div>

      <p class="interactive-stage__tutor-bridge" aria-live="polite">
        [{locale.value.toUpperCase()}]
        {activeChapter
          ? ` Tema: ${activeChapter.title} · ejercicio ${activeChapter.exercise_ref ?? concept.id}`
          : " Tema: (video completo)"}
        {" — "}
        {tutorContext.value.activeSegment?.text ??
          (hasTranscript
            ? "reproducí el video para sincronizar el contexto"
            : "sin transcripción en este bloque")}
      </p>
    </section>
  );
});
