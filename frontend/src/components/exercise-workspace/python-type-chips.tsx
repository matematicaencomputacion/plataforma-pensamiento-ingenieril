import { component$, type QRL } from "@builder.io/qwik";
import {
  PYTHON_TYPE_CHIPS,
  getPythonTypeChip,
  type PythonTypeId,
} from "./python-type-catalog";

export type PythonTypeChipsProps = {
  /** Tipo activo, o `null` si el panel está cerrado. */
  activeType: PythonTypeId | null;
  onSelect$: QRL<(typeId: PythonTypeId | null) => void>;
  /** Título de sección mostrado junto a las píldoras (p. ej. "Variables"). */
  heading?: string;
};

/**
 * Píldoras exclusivas (acordeón): un solo tipo activo.
 * Toggle el mismo chip cierra el panel; otro chip lo reemplaza.
 */
export const PythonTypeChips = component$((props: PythonTypeChipsProps) => {
  const activeChip = getPythonTypeChip(props.activeType);
  const panelOpen = Boolean(activeChip);
  const heading = props.heading?.trim() || "";

  return (
    <div class="type-chips">
      <div class="type-chips__title-row">
        {heading ? (
          <h3 class="type-chips__heading">{heading}</h3>
        ) : (
          <p class="type-chips__label">Tipos de variable</p>
        )}
        <div
          class="type-chips__row"
          role="group"
          aria-label="Tipos de datos Python"
        >
          {PYTHON_TYPE_CHIPS.map((chip) => {
            const on = props.activeType === chip.id;
            return (
              <button
                key={chip.id}
                type="button"
                class={`type-chips__chip${on ? " type-chips__chip--on" : ""}`}
                aria-pressed={on}
                aria-expanded={on}
                aria-controls="python-type-explain-panel"
                title={on ? `Ocultar ${chip.label}` : `Explicar ${chip.label}`}
                onClick$={() => {
                  props.onSelect$(on ? null : chip.id);
                }}
              >
                <code>{chip.label}</code>
              </button>
            );
          })}
        </div>
      </div>

      <div
        id="python-type-explain-panel"
        class={`type-chips__panel${panelOpen ? " type-chips__panel--open" : ""}`}
        aria-hidden={!panelOpen}
      >
        <div class="type-chips__panel-inner">
          {activeChip && (
            <aside class="type-chips__explain" role="region" aria-live="polite">
              <p class="type-chips__explain-text">{activeChip.explanation}</p>
              <pre class="type-chips__code">{activeChip.sampleCode}</pre>
            </aside>
          )}
        </div>
      </div>
    </div>
  );
});
