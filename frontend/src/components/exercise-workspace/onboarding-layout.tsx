import {
  $,
  component$,
  useSignal,
  useStore,
  useVisibleTask$,
  type QRL,
} from "@builder.io/qwik";
import { useNavigate } from "@builder.io/qwik-city";
import type { Microstep } from "../../lib/microsteps";
import {
  CoachingInterface,
  type CoachingInteractionState,
} from "./coaching-interface";
import {
  fetchUserProfile,
  isUserProfileEmpty,
  putUserProfile,
  saveLearnerProfile,
  snapshotUserProfile,
  synthesisToUserProfile,
  userProfileToSynthesis,
} from "./learner-profile";
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
  /** Href del paso 2 — se navega tras persistir (si dirty) o en seco (si limpio). */
  nextStepHref?: string;
  onNotesChange$: QRL<(value: string) => void>;
  onContinue$: QRL<() => void>;
  onProfileSaved$?: QRL<() => void>;
  onProfileReset$?: QRL<() => void>;
};

/** Layout de onboarding: coaching (izq) + profile builder (der). */
export const OnboardingLayout = component$((props: OnboardingLayoutProps) => {
  const nav = useNavigate();
  const interactionState = useSignal<CoachingInteractionState>("drafting");
  const profileSynthesis = useStore<ProfileSynthesis>({
    ...EMPTY_PROFILE_SYNTHESIS,
  });
  /** Snapshot JSON del perfil rehidratado / último persistido (dirty checking). */
  const baselineSnapshot = useSignal<string | null>(null);
  const saveUi = useStore({ isSaving: false, error: "" });
  const analyzeUi = useStore({ isAnalyzing: false, error: "" });
  const advanceUi = useStore({ isAdvancing: false, error: "" });
  const hydrateUi = useStore({ loading: true, error: "" });

  // Rehidratación: GET /api/user/profile al montar el onboarding (cliente).
  // eslint-disable-next-line qwik/no-use-visible-task
  useVisibleTask$(async () => {
    hydrateUi.loading = true;
    hydrateUi.error = "";
    try {
      const result = await fetchUserProfile();
      if (!result.ok) {
        // Sin sesión o error de red: el alumno puede seguir redactando.
        if (result.status !== 401) {
          hydrateUi.error = result.message;
        }
        return;
      }
      if (result.empty) {
        baselineSnapshot.value = null;
        return;
      }
      applyProfileSynthesis(
        profileSynthesis,
        userProfileToSynthesis(result.profile),
      );
      baselineSnapshot.value = snapshotUserProfile(result.profile);
      interactionState.value = "saved";
      if (props.onProfileSaved$) {
        await props.onProfileSaved$();
      }
    } catch {
      hydrateUi.error = "No pudimos rehidratar el perfil guardado.";
    } finally {
      hydrateUi.loading = false;
    }
  });

  const analyze$ = $(async () => {
    if (analyzeUi.isAnalyzing) {
      return;
    }
    analyzeUi.isAnalyzing = true;
    analyzeUi.error = "";
    saveUi.error = "";
    advanceUi.error = "";
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
    advanceUi.error = "";
    advanceUi.isAdvancing = false;
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
    advanceUi.error = "";
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

  /**
   * Botón verde: PUT solo si el perfil cambió vs baseline;
   * si está limpio, navega sin pegarle al API.
   */
  const advance$ = $(async () => {
    if (interactionState.value !== "saved" || advanceUi.isAdvancing) {
      return;
    }
    if (!props.nextStepHref) {
      await props.onContinue$();
      return;
    }

    advanceUi.isAdvancing = true;
    advanceUi.error = "";
    try {
      const current = synthesisToUserProfile(profileSynthesis);
      const currentSnap = snapshotUserProfile(current);
      const dirty =
        baselineSnapshot.value === null ||
        baselineSnapshot.value !== currentSnap;

      if (dirty) {
        if (isUserProfileEmpty(current)) {
          advanceUi.error =
            "El perfil está vacío. Analizá y confirmá tu síntesis antes de avanzar.";
          return;
        }
        const result = await putUserProfile(current);
        if (!result.ok) {
          advanceUi.error = result.message;
          return;
        }
        baselineSnapshot.value = snapshotUserProfile(result.profile);
      }

      await nav(props.nextStepHref);
    } catch (err) {
      const detail = err instanceof Error ? err.message : String(err);
      advanceUi.error = detail
        ? `Error al avanzar: ${detail}`
        : "Error al persistir el perfil. Intentá avanzar de nuevo.";
    } finally {
      advanceUi.isAdvancing = false;
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
        {hydrateUi.loading && (
          <p class="exercise-ws__objective" role="status">
            Recuperando perfil guardado…
          </p>
        )}
        {hydrateUi.error && (
          <p class="profile-builder__save-error" role="alert">
            {hydrateUi.error}
          </p>
        )}
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
            <button
              type="button"
              class="exercise-ws__btn exercise-ws__btn--primary adaptive-mcq__continue"
              disabled={advanceUi.isAdvancing}
              onClick$={advance$}
            >
              {advanceUi.isAdvancing
                ? "Guardando perfil…"
                : "Avanzar hacia los ejercicios"}
            </button>
            {advanceUi.error && (
              <p class="profile-builder__save-error" role="alert">
                {advanceUi.error}
              </p>
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
          isAdvancing={advanceUi.isAdvancing}
          advanceError={advanceUi.error}
          onSave$={save$}
          onContinue$={advance$}
        />
      </div>
    </div>
  );
});
