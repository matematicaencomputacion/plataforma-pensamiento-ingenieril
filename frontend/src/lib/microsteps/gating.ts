import type { CheckMode, McqOptionBank, Microstep } from "./types";

/**
 * Reglas de avance por modo de check (spec exercise-workspace, tarea 4.1):
 * - `mcq_or_run`: alcanza con responder bien el MCQ o pasar los checks de código.
 * - `pytest`: solo los checks de código habilitan Continuar.
 * - `pytest_plus_optional_mcq`: el MCQ es refuerzo opcional; gatea pytest.
 */
export function isStepGateOpen(
  mode: CheckMode,
  checksPassed: boolean,
  mcqAllCorrect: boolean,
): boolean {
  switch (mode) {
    case "mcq_or_run":
      return checksPassed || mcqAllCorrect;
    case "pytest":
      return checksPassed;
    case "pytest_plus_optional_mcq":
      return checksPassed;
    default: {
      const exhaustive: never = mode;
      throw new Error(`CheckMode no soportado: ${String(exhaustive)}`);
    }
  }
}

/** Banco MCQ efectivo del paso (vacío si el paso no define preguntas). */
export function getStepMcqBank(step: Microstep): McqOptionBank[] {
  return step.content.mcq_bank ?? [];
}

/** ¿El paso tiene un MCQ que la UI deba renderizar? (tarea 4.1) */
export function stepHasMcq(step: Microstep): boolean {
  return getStepMcqBank(step).length > 0 || Boolean(step.checks.mcq);
}

/**
 * ¿Todas las preguntas del banco fueron respondidas correctamente?
 * `answers` mapea id de pregunta → opción elegida. Sin banco ⇒ false
 * (no se puede abrir el gate `mcq_or_run` sin responder nada).
 */
export function areBankAnswersCorrect(
  bank: McqOptionBank[],
  answers: Record<string, string>,
): boolean {
  if (!bank.length) {
    return false;
  }
  return bank.every((q) => answers[q.id] === q.correct);
}
