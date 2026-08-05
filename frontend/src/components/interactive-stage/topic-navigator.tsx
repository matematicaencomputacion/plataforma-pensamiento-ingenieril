import { component$, type QRL } from "@builder.io/qwik";
import {
  formatTimestamp,
  type MediaChapter,
} from "../../lib/curriculum-media";

export type TopicNavigatorProps = {
  chapters: MediaChapter[];
  activeChapterId: string | null;
  onSelect$: QRL<(chapterId: string) => void>;
};

export const TopicNavigator = component$<TopicNavigatorProps>((props) => {
  if (!props.chapters.length) {
    return null;
  }

  return (
    <nav class="topic-nav" aria-label="Índice de temas del video">
      <header class="topic-nav__head">
        <h3 class="topic-nav__title">Temas del curso</h3>
        <p class="topic-nav__hint">Saltar a un bloque conceptual</p>
      </header>
      <ol class="topic-nav__list">
        {props.chapters.map((ch, index) => {
          const active = props.activeChapterId === ch.id;
          return (
            <li key={ch.id}>
              <button
                type="button"
                class={`topic-nav__item${active ? " topic-nav__item--active" : ""}`}
                aria-current={active ? "true" : undefined}
                onClick$={() => props.onSelect$(ch.id)}
              >
                <span class="topic-nav__index">{index + 1}</span>
                <span class="topic-nav__body">
                  <span class="topic-nav__name">{ch.title}</span>
                  <span class="topic-nav__meta">
                    {formatTimestamp(ch.start_sec)} –{" "}
                    {formatTimestamp(ch.end_sec)}
                    {ch.exercise_ref ? ` · ${ch.exercise_ref}` : ""}
                  </span>
                </span>
              </button>
            </li>
          );
        })}
      </ol>
    </nav>
  );
});
