import {
  $,
  component$,
  useSignal,
  useStore,
  type QRL,
} from "@builder.io/qwik";
import { Link } from "@builder.io/qwik-city";
import type { Microstep } from "../../lib/microsteps";
import {
  CoachingInterface,
  type CoachingInteractionState,
} from "./coaching-interface";
import { saveLearnerProfile } from "./learner-profile";
import { ProfileBuilder } from "./profile-builder";
import {
  EMPTY_PROFILE_SYNTHESIS,
  applyProfileSynthesis,
  type ProfileSynthesis,
} from "./profile-synthesis";
import { synthesizeLearnerProfile } from "./synthesize-profile";

export type OnboardingLayoutProps = {
  step: Microstep;
  notes: string;
  /** Href del paso 2 — navegación por Link (más fiable que onClick$ QRL). */
  nextStepHref?: string;
  onNotesChange$: QRL<(value: string) => void>;
  onContinue$: QRL<() => void>;
  onProfileSaved$?: QRL<() => void>;
  onProfileReset$?: QRL<() => void>;
};

/** Layout de onboarding: coaching (izq) + profile builder (der). */
export const OnboardingLayout = component$((props: OnboardingLayoutProps) => {
  const interactionState = useSignal<CoachingInteractionState>("drafting");
  const profileSynthesis = useStore<ProfileSynthesis>({
    ...EMPTY_PROFILE_SYNTHESIS,
  });
  const saveUi = useStore({ isSaving: false, error: "" });
  const analyzeUi = useStore({ isAnalyzing: false, error: "" });

  const analyze$ = $(async () => {
    if (analyzeUi.isAnalyzing) {
      return;
    }
    analyzeUi.isAnalyzing = true;
    analyzeUi.error = "";
    saveUi.error = "";
    if (props.onProfileReset$) {
      await props.onProfileReset$();
    }
    try {
      const result = await synthesizeLearnerProfile({
        rawNotes: props.notes,
        sourceStepId: props.step.id,
      });
      if (!result.ok) {
        analyzeUi.error = result.message;
        return;
      }
      applyProfileSynthesis(profileSynthesis, result.synthesis);
      interactionState.value = "reviewing";
    } catch {
      analyzeUi.error =
        "Error inesperado al analizar. Revisá el backend e intentá de nuevo.";
    } finally {
      analyzeUi.isAnalyzing = false;
    }
  });

  const redesign$ = $(() => {
    interactionState.value = "drafting";
    applyProfileSynthesis(profileSynthesis, EMPTY_PROFILE_SYNTHESIS);
    saveUi.error = "";
    saveUi.isSaving = false;
    analyzeUi.error = "";
    analyzeUi.isAnalyzing = false;
    if (props.onProfileReset$) {
      props.onProfileReset$();
    }
  });

  const save$ = $(async () => {
    if (interactionState.value !== "reviewing" || saveUi.isSaving) {
      return;
    }
    saveUi.isSaving = true;
    saveUi.error = "";
    try {
      const result = await saveLearnerProfile({
        purpose: profileSynthesis.purpose,
        urgency: profileSynthesis.urgency,
        vision: profileSynthesis.vision,
        stack: profileSynthesis.stack,
        rawNotes: props.notes.trim(),
        sourceStepId: props.step.id,
        savedAt: new Date().toISOString(),
      });
      if (!result.success) {
        saveUi.error = "No pudimos guardar el perfil. Intentá de nuevo.";
        return;
      }
      if (props.onProfileSaved$) {
        await props.onProfileSaved$();
      }
      interactionState.value = "saved";
    } catch {
      saveUi.error =
        "Error al guardar el perfil. Revisá la conexión e intentá otra vez.";
    } finally {
      saveUi.isSaving = false;
    }
  });

  const snippet = props.notes.trim()
    ? props.notes.trim().slice(0, 160) +
      (props.notes.trim().length > 160 ? "…" : "")
    : undefined;

  const showContinue = interactionState.value === "saved";

  return (
    <div class="exercise-ws__grid exercise-ws__grid--onboarding">
      <div class="exercise-ws__theory exercise-ws__pane--coaching">
        <CoachingInterface
          prompts={props.step.content.coaching_prompts ?? []}
          notes={props.notes}
          interactionState={interactionState.value}
          isAnalyzing={analyzeUi.isAnalyzing}
          analyzeError={analyzeUi.error}
          onNotesChange$={props.onNotesChange$}
          onAnalyze$={analyze$}
          onRedesign$={redesign$}
        />
        {showContinue && (
          <div class="coaching__continue">
            {props.nextStepHref ? (
              <Link
                href={props.nextStepHref}
                class="exercise-ws__btn exercise-ws__btn--primary adaptive-mcq__continue"
              >
                Avanzar hacia los ejercicios
              </Link>
            ) : (
              <button
                type="button"
                class="exercise-ws__btn exercise-ws__btn--primary adaptive-mcq__continue"
                onClick$={props.onContinue$}
              >
                Avanzar hacia los ejercicios
              </button>
            )}
          </div>
        )}
      </div>
      <div class="exercise-ws__editor-pane exercise-ws__pane--profile">
        <ProfileBuilder
          interactionState={interactionState.value}
          synthesis={profileSynthesis}
          learnerSnippet={
            interactionState.value === "drafting" ? undefined : snippet
          }
          isSaving={saveUi.isSaving}
          saveError={saveUi.error}
          nextStepHref={props.nextStepHref}
          onSave$={save$}
          onContinue$={props.onContinue$}
        />
      </div>
    </div>
  );
});
