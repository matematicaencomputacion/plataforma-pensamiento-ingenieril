import { component$, type QRL } from "@builder.io/qwik";
import {
  formatTimestamp,
  type TranscriptSegment,
} from "../../lib/curriculum-media";

export type TranscriptPanelProps = {
  segments: TranscriptSegment[];
  activeStartSec: number | null;
  onSeek$: QRL<(startSec: number) => void>;
};

export const TranscriptPanel = component$<TranscriptPanelProps>((props) => {
  return (
    <aside class="transcript-panel" aria-label="Transcripción interactiva">
      <header class="transcript-panel__head">
        <h3 class="transcript-panel__title">Transcripción</h3>
        <p class="transcript-panel__hint">Clic en una línea para saltar al video</p>
      </header>
      <ol class="transcript-panel__list">
        {props.segments.map((seg) => {
          const active = props.activeStartSec === seg.start_sec;
          return (
            <li key={`${seg.start_sec}-${seg.end_sec}`}>
              <button
                type="button"
                class={`transcript-panel__cue${active ? " transcript-panel__cue--active" : ""}`}
                onClick$={() => props.onSeek$(seg.start_sec)}
              >
                <span class="transcript-panel__time">
                  {formatTimestamp(seg.start_sec)}
                </span>
                <span class="transcript-panel__text">{seg.text}</span>
              </button>
            </li>
          );
        })}
      </ol>
    </aside>
  );
});
