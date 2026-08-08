import { component$, type QRL } from "@builder.io/qwik";
import type { McqOptionBank, StepChecks } from "../../lib/microsteps";

export type McqBankPanelProps = {
  bank: McqOptionBank[];
  checksMcq: StepChecks["mcq"] | null;
  /** true cuando el MCQ es refuerzo opcional (no gatea Continuar). */
  optional: boolean;
  /** id de pregunta → opción elegida por el alumno. */
  answers: Record<string, string>;
  onAnswer$: QRL<(questionId: string, option: string) => void>;
};

/**
 * MCQ del banco de la semilla (`content.mcq_bank`), tarea 4.1 del harness.
 * Feedback inmediato correcto/incorrecto + rationale de `checks.mcq` si existe.
 * Estado 100% local (sin persistencia backend).
 */
export const McqBankPanel = component$((props: McqBankPanelProps) => {
  return (
    <div class="mcq-bank" aria-label="Preguntas de opción múltiple">
      <p class="mcq-bank__label">
        Check de concepto{props.optional ? " · opcional" : ""}
      </p>

      {props.bank.map((question) => {
        const chosen = props.answers[question.id];
        const isCorrect = chosen === question.correct;
        return (
          <fieldset key={question.id} class="mcq-bank__question">
            <legend class="mcq-bank__prompt">{question.prompt}</legend>
            <div class="mcq-bank__options" role="radiogroup">
              {question.options.map((option) => {
                const isChosen = chosen === option;
                const stateClass = !chosen
                  ? ""
                  : isChosen
                    ? isCorrect
                      ? " mcq-bank__option--correct"
                      : " mcq-bank__option--incorrect"
                    : " mcq-bank__option--muted";
                return (
                  <button
                    key={option}
                    type="button"
                    role="radio"
                    aria-checked={isChosen ? "true" : "false"}
                    class={`mcq-bank__option${stateClass}`}
                    onClick$={() => {
                      props.onAnswer$(question.id, option);
                    }}
                  >
                    {option}
                  </button>
                );
              })}
            </div>
            {chosen && (
              <p
                class={`mcq-bank__feedback${
                  isCorrect
                    ? " mcq-bank__feedback--pass"
                    : " mcq-bank__feedback--fail"
                }`}
                role="status"
                aria-live="polite"
              >
                {isCorrect
                  ? "Correcto."
                  : "No es la opción esperada; probá de nuevo."}
                {isCorrect && props.checksMcq?.rationale
                  ? ` ${props.checksMcq.rationale}`
                  : ""}
              </p>
            )}
          </fieldset>
        );
      })}
    </div>
  );
});
