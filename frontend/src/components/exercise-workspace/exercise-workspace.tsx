import {
  component$,
  useStore,
  useTask$,
  useVisibleTask$,
  $,
  type QRL,
} from "@builder.io/qwik";
import { Link } from "@builder.io/qwik-city";
import { CodingLayout } from "./coding-layout";
import { OnboardingLayout } from "./onboarding-layout";
import {
  getLayoutType,
  getSeedStepCount,
  isFrontierNext,
  resolveStep,
} from "../../lib/microsteps";
import {
  bootstrapPyodide,
  checkStudentCode,
  formatCheckLog,
  formatRunLog,
  pyodideStatusMessage,
  runStudentCode,
  type PyodideEngineStatus,
} from "../../lib/pyodide";

export type ExerciseWorkspaceProps = {
  initialStepId?: string;
  onStepChange$?: QRL<(stepId: string) => void>;
};

type ExerciseState = {
  stepId: string;
  code: string;
  checkStatus: "idle" | "pass" | "fail";
  resultsLog: string;
  showHint: boolean;
  showSolution: boolean;
  lotComplete: boolean;
  coachingNotes: string;
  profileSaved: boolean;
  isAdvancing: boolean;
  engineStatus: PyodideEngineStatus;
  isBusy: boolean;
};

function initState(stepId?: string): ExerciseState {
  const { step } = resolveStep(stepId);
  return {
    stepId: step.id,
    code: step.content.starter_code,
    checkStatus: "idle",
    resultsLog:
      "Al abrir un paso de código se prepara el motor Python (Pyodide) en el navegador.",
    showHint: false,
    showSolution: false,
    lotComplete: false,
    coachingNotes: "",
    profileSaved: false,
    isAdvancing: false,
    engineStatus: "idle",
    isBusy: false,
  };
}

function stepHref(stepId: string): string {
  return `/exercise?step=${encodeURIComponent(stepId)}`;
}

/**
 * Bifurca onboarding (coaching) vs coding (teoría + editor + Pyodide).
 */
export const ExerciseWorkspace = component$((props: ExerciseWorkspaceProps) => {
  const state = useStore<ExerciseState>(initState(props.initialStepId));

  useTask$(({ track }) => {
    const incoming = track(() => props.initialStepId);
    if (!incoming || incoming === state.stepId) {
      return;
    }
    const { step } = resolveStep(incoming);
    state.stepId = step.id;
    state.code = step.content.starter_code;
    state.checkStatus = "idle";
    state.showHint = false;
    state.showSolution = false;
    state.lotComplete = false;
    state.isAdvancing = false;
    state.isBusy = false;
    if (getLayoutType(step) === "onboarding") {
      state.coachingNotes = "";
      state.profileSaved = false;
      state.resultsLog = "Onboarding: contanos tu propósito.";
    } else {
      state.resultsLog =
        state.engineStatus === "ready"
          ? pyodideStatusMessage("ready")
          : pyodideStatusMessage(
              state.engineStatus === "loading" ? "loading" : "idle",
            );
    }
  });

  // Lazy-load Pyodide al montar el harness (cliente). No corre en SSR.
  // eslint-disable-next-line qwik/no-use-visible-task
  useVisibleTask$(async () => {
    if (state.engineStatus === "ready" || state.engineStatus === "loading") {
      return;
    }
    state.engineStatus = "loading";
    state.resultsLog = pyodideStatusMessage("loading");
    const ready = await bootstrapPyodide();
    state.engineStatus = ready.status;
    if (getLayoutType(resolveStep(state.stepId).step) === "coding") {
      state.resultsLog = ready.message;
    }
  });

  const goNext = $(async () => {
    if (state.isAdvancing || state.lotComplete) {
      return;
    }
    const current = resolveStep(state.stepId).step;
    const layout = getLayoutType(current);
    if (layout === "onboarding") {
      state.profileSaved = true;
    }
    const allowed =
      layout === "onboarding"
        ? state.profileSaved
        : state.checkStatus === "pass";
    if (!allowed) {
      return;
    }
    const nextId = current.next;
    if (!nextId || isFrontierNext(nextId)) {
      state.lotComplete = true;
      state.resultsLog = "Lote completado. Próximamente: Strings.";
      return;
    }
    state.isAdvancing = true;
    try {
      if (props.onStepChange$) {
        await props.onStepChange$(nextId);
      } else {
        const next = resolveStep(nextId).step;
        state.stepId = next.id;
        state.code = next.content.starter_code;
        state.checkStatus = "idle";
        state.showHint = false;
        state.showSolution = false;
        state.lotComplete = false;
        state.resultsLog =
          getLayoutType(next) === "onboarding"
            ? "Onboarding: contanos tu propósito."
            : pyodideStatusMessage(
                state.engineStatus === "ready" ? "ready" : state.engineStatus,
              );
      }
    } finally {
      state.isAdvancing = false;
    }
  });

  const runCode$ = $(async () => {
    if (state.engineStatus !== "ready" || state.isBusy) {
      return;
    }
    state.isBusy = true;
    state.checkStatus = "idle";
    state.resultsLog = "Ejecutando…";
    try {
      const result = await runStudentCode(state.code);
      state.resultsLog = formatRunLog(result);
      if (!result.ok) {
        state.checkStatus = "fail";
      }
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      state.resultsLog = `=== Run ===\n--- error ---\n${message}`;
      state.checkStatus = "fail";
    } finally {
      state.isBusy = false;
    }
  });

  const validate$ = $(async () => {
    if (state.engineStatus !== "ready" || state.isBusy) {
      return;
    }
    const current = resolveStep(state.stepId).step;
    const testSource = current.checks.pytest?.trim();
    if (!testSource) {
      state.checkStatus = "fail";
      state.resultsLog =
        "=== Validar ===\nEste paso no define checks pytest en la semilla.";
      return;
    }

    state.isBusy = true;
    state.checkStatus = "idle";
    state.resultsLog = "Validando contra los checks del micro-reto…";
    try {
      const result = await checkStudentCode(state.code, testSource);
      state.resultsLog = formatCheckLog(result);
      state.checkStatus = result.passed ? "pass" : "fail";
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      state.resultsLog = `=== Validar ===\n--- error ---\n${message}`;
      state.checkStatus = "fail";
    } finally {
      state.isBusy = false;
    }
  });

  const { step, fallback } = resolveStep(state.stepId);
  const layout = getLayoutType(step);
  const seedTotal = getSeedStepCount();
  const isOnboarding = layout === "onboarding";
  const nextStepId =
    step.next && !isFrontierNext(step.next) ? step.next : undefined;
  const nextStepHref = nextStepId ? stepHref(nextStepId) : undefined;
  const canContinue = isOnboarding
    ? state.profileSaved && !state.lotComplete && !state.isAdvancing
    : state.checkStatus === "pass" && !state.lotComplete && !state.isAdvancing;

  return (
    <div
      class={`exercise-ws${isOnboarding ? " exercise-ws--onboarding" : " exercise-ws--coding"}`}
    >
      <header class="exercise-ws__header">
        <div class="exercise-ws__meta">
          <p class="exercise-ws__progress">
            Paso {step.step_number}/{seedTotal}
            {fallback ? " · (fallback al primero)" : ""}
            {isOnboarding ? " · Onboarding" : " · Coding"}
            {!isOnboarding && state.engineStatus === "loading"
              ? " · Preparando Python…"
              : ""}
            {!isOnboarding && state.engineStatus === "ready"
              ? " · Pyodide listo"
              : ""}
          </p>
          <h1 class="exercise-ws__title">{step.title}</h1>
        </div>
        <div class="exercise-ws__actions">
          {!isOnboarding && (
            <>
              <button
                type="button"
                class="exercise-ws__btn"
                disabled={!step.hint}
                onClick$={() => {
                  state.showHint = !state.showHint;
                }}
              >
                {state.showHint ? "Ocultar pista" : "Pista"}
              </button>
              <button
                type="button"
                class="exercise-ws__btn"
                disabled={!step.solution_example}
                onClick$={() => {
                  state.showSolution = !state.showSolution;
                }}
              >
                {state.showSolution ? "Ocultar solución" : "Solución"}
              </button>
            </>
          )}
          <Link class="exercise-ws__btn exercise-ws__btn--ghost" href="/workspace">
            Salir al workspace
          </Link>
        </div>
      </header>

      {isOnboarding ? (
        <OnboardingLayout
          step={step}
          notes={state.coachingNotes}
          nextStepHref={nextStepHref}
          onNotesChange$={(value) => {
            state.coachingNotes = value;
          }}
          onProfileSaved$={() => {
            state.profileSaved = true;
          }}
          onProfileReset$={() => {
            state.profileSaved = false;
          }}
          onContinue$={goNext}
        />
      ) : (
        <CodingLayout
          step={step}
          code={state.code}
          showHint={state.showHint}
          showSolution={state.showSolution}
          checkStatus={state.checkStatus}
          resultsLog={state.resultsLog}
          lotComplete={state.lotComplete}
          canContinue={canContinue}
          engineStatus={state.engineStatus}
          isBusy={state.isBusy}
          onCodeInput$={(value) => {
            state.code = value;
            if (state.checkStatus !== "idle") {
              state.checkStatus = "idle";
            }
          }}
          onRun$={runCode$}
          onValidate$={validate$}
          onContinue$={goNext}
        />
      )}
    </div>
  );
});
