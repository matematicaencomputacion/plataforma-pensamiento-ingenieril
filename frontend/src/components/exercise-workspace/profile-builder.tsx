import { component$, type QRL } from "@builder.io/qwik";
import type { CoachingInteractionState } from "./coaching-interface";
import {
  PROFILE_WAITING_COPY,
  type ProfileSynthesis,
} from "./profile-synthesis";

export type ProfileCardTone = "purpose" | "urgency" | "stack" | "default";

type ProfileField = {
  id: keyof ProfileSynthesis;
  label: string;
  tone: ProfileCardTone;
};

const PROFILE_FIELDS: ProfileField[] = [
  { id: "purpose", label: "Propósito", tone: "purpose" },
  { id: "urgency", label: "Urgencia", tone: "urgency" },
  { id: "vision", label: "Visión a 5 años", tone: "purpose" },
  { id: "stack", label: "Stack previo", tone: "stack" },
];

export type ProfileBuilderProps = {
  interactionState: CoachingInteractionState;
  synthesis: ProfileSynthesis;
  learnerSnippet?: string;
  isSaving?: boolean;
  saveError?: string;
  isAdvancing?: boolean;
  advanceError?: string;
  onSave$: QRL<() => void>;
  onContinue$?: QRL<() => void>;
};

/**
 * Panel derecho: espera en drafting; síntesis confirmable en reviewing/saved.
 */
export const ProfileBuilder = component$((props: ProfileBuilderProps) => {
  const showCards = props.interactionState !== "drafting";
  const isReviewing = props.interactionState === "reviewing";
  const isSaved = props.interactionState === "saved";
  const canAdvance = Boolean(props.onContinue$);

  return (
    <section
      class="profile-builder"
      aria-label="Resumen de perfil"
      aria-live="polite"
    >
      <p class="profile-builder__eyebrow">
        Perfil
        {props.interactionState === "drafting"
          ? " · a la espera"
          : isReviewing
            ? " · revisión"
            : " · guardado"}
      </p>
      <h2 class="profile-builder__title">Lo que estamos escuchando</h2>
      <p class="profile-builder__lead">
        {props.interactionState === "drafting"
          ? "Cuando envíes tu respuesta, IngenierIA sintetizará propósito, urgencia, visión y stack."
          : isSaved
            ? "Perfil listo. Avanzá para persistirlo en tu cuenta y continuar a los ejercicios."
            : "Revisá lo que dedujimos. Si no refleja tu historia, volvé a rediseñar tu respuesta."}
      </p>

      {showCards && props.learnerSnippet && (
        <blockquote class="profile-builder__snippet">
          “{props.learnerSnippet}”
        </blockquote>
      )}

      <ul class="profile-builder__cards">
        {PROFILE_FIELDS.map((field) => {
          const value = showCards ? props.synthesis[field.id] : "";
          const waiting = !value;
          return (
            <li
              key={field.id}
              class={`profile-builder__card profile-builder__card--${field.tone}${waiting ? " profile-builder__card--waiting" : ""}`}
            >
              <p class="profile-builder__card-label">{field.label}</p>
              <p
                class={`profile-builder__card-value${waiting ? " profile-builder__card-value--waiting" : ""}`}
              >
                {waiting ? PROFILE_WAITING_COPY : value}
              </p>
            </li>
          );
        })}
      </ul>

      {isReviewing && (
        <div class="profile-builder__confirm">
          <button
            type="button"
            class="exercise-ws__btn exercise-ws__btn--primary profile-builder__save"
            disabled={Boolean(props.isSaving)}
            onClick$={async () => {
              await props.onSave$();
            }}
          >
            {props.isSaving
              ? "Guardando…"
              : "Sí, guardar esta información en mi perfil"}
          </button>
          {props.saveError && (
            <p class="profile-builder__save-error" role="alert">
              {props.saveError}
            </p>
          )}
        </div>
      )}

      {isSaved && canAdvance && (
        <div class="profile-builder__after-save">
          <p class="profile-builder__saved" role="status">
            ✅ Síntesis confirmada — al avanzar se guarda en tu cuenta
          </p>
          <button
            type="button"
            class="exercise-ws__btn exercise-ws__btn--primary profile-builder__next"
            disabled={Boolean(props.isAdvancing)}
            onClick$={async () => {
              if (props.onContinue$) {
                await props.onContinue$();
              }
            }}
          >
            {props.isAdvancing
              ? "Guardando perfil…"
              : "Avanzar hacia los ejercicios"}
          </button>
          {props.advanceError && (
            <p class="profile-builder__save-error" role="alert">
              {props.advanceError}
            </p>
          )}
        </div>
      )}
    </section>
  );
});
