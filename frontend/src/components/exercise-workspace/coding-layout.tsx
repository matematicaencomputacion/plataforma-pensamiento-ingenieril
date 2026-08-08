import {
  component$,
  useSignal,
  useTask$,
  type QRL,
} from "@builder.io/qwik";
import type { Microstep } from "../../lib/microsteps";
import type { PyodideEngineStatus } from "../../lib/pyodide";
import { PromptMarkdown } from "./prompt-markdown";
import { PythonTypeChips } from "./python-type-chips";
import {
  PY02_VARIABLE_HINTS,
  splitVariablesPromptHeading,
  stepShowsPythonTypeChips,
  type PythonTypeId,
} from "./python-type-catalog";

export type CodingLayoutProps = {
  step: Microstep;
  code: string;
  showHint: boolean;
  showSolution: boolean;
  checkStatus: "idle" | "pass" | "fail";
  resultsLog: string;
  lotComplete: boolean;
  canContinue: boolean;
  engineStatus: PyodideEngineStatus;
  isBusy: boolean;
  onCodeInput$: QRL<(value: string) => void>;
  onRun$: QRL<() => void>;
  onValidate$: QRL<() => void>;
  onContinue$: QRL<() => void>;
};

/**
 * Layout coding: enunciado atómico (izq) + editor/ayudas (der) + resultados.
 * Run / Validar ejecutan Python en el navegador vía Pyodide.
 */
export const CodingLayout = component$((props: CodingLayoutProps) => {
  const { step } = props;
  const engineReady = props.engineStatus === "ready";
  const engineLoading = props.engineStatus === "loading";
  const controlsDisabled = !engineReady || props.isBusy;
  const showTypeChips = stepShowsPythonTypeChips(step.id, step.title);
  const activeType = useSignal<PythonTypeId | null>(null);
  const promptParts = showTypeChips
    ? splitVariablesPromptHeading(step.content.prompt_md)
    : { heading: null as string | null, body: step.content.prompt_md };

  useTask$(({ track }) => {
    track(() => props.step.id);
    activeType.value = null;
  });

  return (
    <div class="exercise-ws__grid exercise-ws__grid--coding">
      <section class="exercise-ws__theory" aria-label="Teoría y enunciado">
        <h2 class="exercise-ws__section-title">Enunciado</h2>
        {showTypeChips && (
          <PythonTypeChips
            heading={promptParts.heading ?? "Variables"}
            activeType={activeType.value}
            onSelect$={(typeId) => {
              activeType.value = typeId;
            }}
          />
        )}
        <PromptMarkdown
          markdown={promptParts.body}
          variableHints={showTypeChips ? PY02_VARIABLE_HINTS : undefined}
        />
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
            disabled={controlsDisabled}
            title={
              engineLoading
                ? "Preparando motor Python…"
                : engineReady
                  ? "Ejecutar el código y ver la salida (print)"
                  : "Motor Python no disponible"
            }
            onClick$={props.onRun$}
          >
            {props.isBusy ? "Ejecutando…" : "Run"}
          </button>
          <button
            type="button"
            class="exercise-ws__btn exercise-ws__btn--accent"
            disabled={controlsDisabled}
            title={
              engineReady
                ? "Validar el código contra los checks del micro-reto"
                : "Esperá a que el motor Python esté listo"
            }
            onClick$={props.onValidate$}
          >
            {props.isBusy ? "Validando…" : "Validar"}
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
          aria-live="polite"
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
