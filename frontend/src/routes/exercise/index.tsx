import { component$ } from "@builder.io/qwik";
import type { DocumentHead } from "@builder.io/qwik-city";
import { useLocation, useNavigate } from "@builder.io/qwik-city";
import { ExerciseWorkspace } from "../../components/exercise-workspace/exercise-workspace";
import { resolveStep } from "../../lib/microsteps";

/**
 * Ruta dedicada: bifurca Onboarding (`layoutType: onboarding`) vs Coding.
 * Query: /exercise?step=py-01-home | /exercise?step=py-02-variables
 */
export default component$(() => {
  const loc = useLocation();
  const nav = useNavigate();
  const stepParam = loc.url.searchParams.get("step");
  const { step } = resolveStep(stepParam);

  return (
    <main class="exercise-page">
      <ExerciseWorkspace
        key={step.id}
        initialStepId={step.id}
        onStepChange$={async (stepId) => {
          await nav(`/exercise?step=${encodeURIComponent(stepId)}`);
        }}
      />
    </main>
  );
});

export const head: DocumentHead = {
  title: "Micro-pasos Python · IngenierIA",
  meta: [
    {
      name: "description",
      content:
        "Harness de micro-pasos interactivos (teoría + editor + checks) inspirado en W3Schools Exercise.",
    },
  ],
};
