import { component$, type QRL } from "@builder.io/qwik";
import type { AdaptiveMcq, AdaptiveMcqOption } from "../../lib/microsteps";

export type AdaptiveMcqPanelProps = {
  mcq: AdaptiveMcq;
  selectedKey: string | null;
  onSelect$: QRL<(key: string) => void>;
};

function clampScore(score: number): number {
  if (Number.isNaN(score)) return 1;
  return Math.min(5, Math.max(1, Math.round(score)));
}

/**
 * Check rápido con feedback adaptativo por alineación.
 * Selección → oculta el resto → feedback + fuegos 🔥.
 */
export const AdaptiveMcqPanel = component$((props: AdaptiveMcqPanelProps) => {
  const selected: AdaptiveMcqOption | undefined = props.selectedKey
    ? props.mcq.options.find((o) => o.key === props.selectedKey)
    : undefined;

  return (
    <div
      class={`adaptive-mcq${selected ? " adaptive-mcq--resolved" : ""}`}
      aria-label="Check rápido"
    >
      <p class="adaptive-mcq__label">Check rápido</p>

      {!selected && (
        <div class="adaptive-mcq__options" role="radiogroup">
          {props.mcq.options.map((opt) => (
            <button
              key={opt.key}
              type="button"
              role="radio"
              aria-checked="false"
              class="adaptive-mcq__btn"
              onClick$={() => {
                props.onSelect$(opt.key);
              }}
            >
              <span class="adaptive-mcq__key">{opt.key.toUpperCase()}</span>
              <span class="adaptive-mcq__text">{opt.text}</span>
            </button>
          ))}
        </div>
      )}

      {selected && (
        <div class="adaptive-mcq__stage">
          <button
            type="button"
            class="adaptive-mcq__btn adaptive-mcq__btn--pinned"
            disabled
            aria-disabled="true"
          >
            <span class="adaptive-mcq__key">{selected.key.toUpperCase()}</span>
            <span class="adaptive-mcq__text">{selected.text}</span>
          </button>

          <div
            class="adaptive-mcq__feedback"
            role="status"
            aria-live="polite"
          >
            <div class="adaptive-mcq__alignment" aria-label={`Alineación ${clampScore(selected.alignmentScore)} de 5`}>
              <span class="adaptive-mcq__alignment-label">Alineación</span>
              <span class="adaptive-mcq__fires" aria-hidden="true">
                {Array.from({ length: clampScore(selected.alignmentScore) }).map(
                  (_, i) => (
                    <span key={`fire-${i}`} class="adaptive-mcq__fire">
                      🔥
                    </span>
                  ),
                )}
              </span>
              <span class="adaptive-mcq__alignment-score">
                {clampScore(selected.alignmentScore)}/5
              </span>
            </div>
            <p class="adaptive-mcq__feedback-text">{selected.feedback}</p>
          </div>
        </div>
      )}
    </div>
  );
});
