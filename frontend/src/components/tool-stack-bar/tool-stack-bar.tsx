import { component$, useStore } from "@builder.io/qwik";
import { ToolIcon } from "./tool-icons";
import {
  initialToolStates,
  MASTERY_LABELS,
  masteryClass,
  nextMastery,
  STACK_TOOLS,
  type ToolStates,
} from "./tool-mastery";

/**
 * Barra de herramientas del stack junto a la marca.
 * Logo en el botón + nombre debajo; rota 0 → 1 → 2 (experto + fuego).
 */
export const ToolStackBar = component$(() => {
  const states = useStore<ToolStates>(initialToolStates());

  return (
    <nav class="tool-stack" aria-label="Dominio del stack de herramientas">
      <ul class="tool-stack__list">
        {STACK_TOOLS.map((tool) => {
          const level = states[tool.id];
          const statusLabel = MASTERY_LABELS[level];
          const tip = `${tool.label}: ${statusLabel}`;

          return (
            <li key={tool.id} class="tool-stack__item">
              <button
                type="button"
                class={`tool-chip tool-chip--${tool.id} ${masteryClass(level)}`}
                title={tip}
                aria-label={tip}
                aria-pressed={level > 0 ? "true" : "false"}
                data-mastery={String(level)}
                data-tool={tool.id}
                onClick$={() => {
                  states[tool.id] = nextMastery(states[tool.id]);
                }}
              >
                <span class="tool-chip__glyph">
                  <ToolIcon id={tool.id} />
                  {level === 2 && (
                    <span class="tool-chip__fire" aria-hidden="true">
                      🔥
                    </span>
                  )}
                </span>
                <span class="tool-chip__label">{tool.label}</span>
              </button>
            </li>
          );
        })}
      </ul>
    </nav>
  );
});
