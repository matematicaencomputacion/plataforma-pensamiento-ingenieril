import { component$, type QRL } from "@builder.io/qwik";
import type { Microstep } from "../../lib/microsteps";
import { PromptMarkdown } from "./prompt-markdown";

export type CodingLayoutProps = {
  step: Microstep;
  code: string;
  showHint: boolean;
  showSolution: boolean;
  checkStatus: "idle" | "pass" | "fail";
  resultsLog: string;
  lotComplete: boolean;
  canContinue: boolean;
  onCodeInput$: QRL<(value: string) => void>;
  onValidateDemo$: QRL<() => void>;
  onContinue$: QRL<() => void>;
};

/**
 * Layout coding: enunciado atómico (izq) + editor/ayudas (der) + resultados.
 * Sin Check rápido A/B/C: el foco es escribir código.
 */
export const CodingLayout = component$((props: CodingLayoutProps) => {
  const { step } = props;

  return (
    <div class="exercise-ws__grid exercise-ws__grid--coding">
      <section class="exercise-ws__theory" aria-label="Teoría y enunciado">
        <h2 class="exercise-ws__section-title">Enunciado</h2>
        <PromptMarkdown markdown={step.content.prompt_md} />
        {step.objective && (
          <p class="exercise-ws__objective">
            <strong>Objetivo:</strong> {step.objective}
          </p>
        )}
      </section>

      <section class="exercise-ws__editor-pane" aria-label="Editor">
        <h2 class="exercise-ws__section-title">Editor</h2>
        {(props.showHint || props.showSolution) && (
          <div class="exercise-ws__editor-aids" aria-live="polite">
            {props.showHint && step.hint && (
              <aside class="exercise-ws__callout exercise-ws__callout--hint exercise-ws__callout--editor">
                <strong>Pista:</strong> {step.hint}
              </aside>
            )}
            {props.showSolution && step.solution_example && (
              <aside class="exercise-ws__callout exercise-ws__callout--solution exercise-ws__callout--editor">
                <strong>Solución ejemplo:</strong>
                <pre class="exercise-ws__pre">{step.solution_example}</pre>
              </aside>
            )}
          </div>
        )}
        <textarea
          class="exercise-ws__editor"
          spellcheck={false}
          value={props.code}
          onInput$={(_, el) => {
            props.onCodeInput$(el.value);
          }}
        />
      </section>

      <section class="exercise-ws__results" aria-label="Resultados">
        <h2 class="exercise-ws__section-title">Resultados</h2>
        <div class="exercise-ws__toolbar">
          <button
            type="button"
            class="exercise-ws__btn"
            disabled
            title="Disponible en Bloque 3 (Pyodide)"
          >
            Run
          </button>
          <button
            type="button"
            class="exercise-ws__btn exercise-ws__btn--accent"
            onClick$={props.onValidateDemo$}
          >
            Validar (demo)
          </button>
          <button
            type="button"
            class="exercise-ws__btn exercise-ws__btn--primary"
            disabled={!props.canContinue}
            onClick$={props.onContinue$}
          >
            Continuar
          </button>
        </div>
        <pre
          class={`exercise-ws__console${
            props.checkStatus === "pass"
              ? " exercise-ws__console--pass"
              : props.checkStatus === "fail"
                ? " exercise-ws__console--fail"
                : ""
          }`}
        >
          {props.resultsLog}
        </pre>
        {props.lotComplete && (
          <p class="exercise-ws__frontier" role="status">
            Has terminado los micro-pasos de esta semilla. Volvé al workspace o
            esperá el lote de Strings.
          </p>
        )}
      </section>
    </div>
  );
});
