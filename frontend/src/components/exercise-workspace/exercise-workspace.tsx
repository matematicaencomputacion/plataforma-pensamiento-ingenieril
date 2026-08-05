import {
  component$,
  useStore,
  useTask$,
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
  normalizeCheckPayload,
  resolveStep,
  type Microstep,
} from "../../lib/microsteps";

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
};

function initState(stepId?: string): ExerciseState {
  const { step } = resolveStep(stepId);
  return {
    stepId: step.id,
    code: step.content.starter_code,
    checkStatus: "idle",
    resultsLog:
      "Motor Python (Pyodide) pendiente — Bloque 3.\nUsá Validar (demo) para recorrer la navegación de coding.",
    showHint: false,
    showSolution: false,
    lotComplete: false,
    coachingNotes: "",
    profileSaved: false,
  };
}

/**
 * Bifurca onboarding (coaching) vs coding (teoría + editor).
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
    if (getLayoutType(step) === "onboarding") {
      state.coachingNotes = "";
      state.profileSaved = false;
    }
    state.resultsLog = "Step sincronizado desde la URL.";
  });

  const loadStep = $(async (next: Microstep) => {
    state.stepId = next.id;
    state.code = next.content.starter_code;
    state.checkStatus = "idle";
    state.showHint = false;
    state.showSolution = false;
    state.lotComplete = false;
    if (getLayoutType(next) === "onboarding") {
      state.coachingNotes = "";
      state.profileSaved = false;
      state.resultsLog = "Onboarding: contanos tu propósito.";
    } else {
      state.resultsLog =
        "Step de código cargado. Escribí en el editor y usá Validar (demo).";
    }
    if (props.onStepChange$) {
      await props.onStepChange$(next.id);
    }
  });

  const goNext = $(async () => {
    const current = resolveStep(state.stepId).step;
    const layout = getLayoutType(current);
    // En onboarding, el CTA solo aparece tras `saved`; sincronizamos el flag
    // por si el prop canContinue quedó stale entre padre/hijo.
    if (layout === "onboarding") {
      state.profileSaved = true;
    }
    const allowed =
      layout === "onboarding"
        ? state.profileSaved && !state.lotComplete
        : state.checkStatus === "pass" && !state.lotComplete;
    if (!allowed) {
      return;
    }
    if (!current.next || isFrontierNext(current.next)) {
      state.lotComplete = true;
      state.resultsLog = "Lote completado. Próximamente: Strings.";
      return;
    }
    const { step: nextStep } = resolveStep(current.next);
    await loadStep(nextStep);
  });

  const { step, fallback } = resolveStep(state.stepId);
  const layout = getLayoutType(step);
  const seedTotal = getSeedStepCount();
  const isOnboarding = layout === "onboarding";
  const canContinue = isOnboarding
    ? state.profileSaved && !state.lotComplete
    : state.checkStatus === "pass" && !state.lotComplete;

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
          <Link class="exercise-ws__btn exercise-ws__btn--ghost" href="/">
            Salir al workspace
          </Link>
        </div>
      </header>

      {isOnboarding ? (
        <OnboardingLayout
          step={step}
          notes={state.coachingNotes}
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
          onCodeInput$={(value) => {
            state.code = value;
            if (state.checkStatus !== "idle") {
              state.checkStatus = "idle";
            }
          }}
          onValidateDemo$={() => {
            const current = resolveStep(state.stepId).step;
            const payload = normalizeCheckPayload(current, state.code);
            state.resultsLog = [
              "[demo] Check stub (sin Pyodide aún)",
              `step=${payload.stepId}`,
              `mode=${payload.mode}`,
              "",
              "✓ Validación demo OK — puedes Continuar",
            ].join("\n");
            state.checkStatus = "pass";
          }}
          onContinue$={goNext}
        />
      )}
    </div>
  );
});
