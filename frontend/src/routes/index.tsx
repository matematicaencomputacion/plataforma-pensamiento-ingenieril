import { component$, useSignal, useVisibleTask$ } from "@builder.io/qwik";
import type { DocumentHead } from "@builder.io/qwik-city";
import {
  BrandName,
  BRAND_NAME_PLAIN,
} from "../components/brand-name/brand-name";
import { InteractiveStage } from "../components/interactive-stage/interactive-stage";
import { LevelDescription } from "../components/level-description/level-description";
import { ToolStackBar } from "../components/tool-stack-bar/tool-stack-bar";
import {
  API_BASE_URL,
  DEMO_STUDENT_ID,
  type EvaluateResponse,
  type Level,
} from "../lib/api";
import type { PedTopicContext } from "../lib/curriculum-media";

export default component$(() => {
  const code = useSignal(
    '# Escribe tu solución en Python\nprint("Hola, pensamiento ingenieril")\n',
  );
  const level = useSignal<Level | null>(null);
  const levelLoading = useSignal(true);
  const levelError = useSignal("");
  const result = useSignal("");
  const feedback = useSignal("");
  const isEvaluating = useSignal(false);
  const passed = useSignal<boolean | null>(null);
  const activeTopic = useSignal<PedTopicContext | null>(null);

  // eslint-disable-next-line qwik/no-use-visible-task
  useVisibleTask$(async () => {
    levelLoading.value = true;
    levelError.value = "";

    try {
      const response = await fetch(`${API_BASE_URL}/api/levels/current`);
      if (!response.ok) {
        levelError.value =
          "No se pudo cargar el enunciado del nivel actual.";
        level.value = null;
        return;
      }

      level.value = (await response.json()) as Level;
    } catch {
      levelError.value =
        "No se pudo conectar con el backend para cargar el nivel.";
      level.value = null;
    } finally {
      levelLoading.value = false;
    }
  });

  return (
    <main class="workspace">
      <header class="workspace__header">
        <div class="workspace__brand-row">
          <h1 class="workspace__title" aria-label={BRAND_NAME_PLAIN}>
            <BrandName />
          </h1>
          <ToolStackBar />
        </div>
        <p class="workspace__subtitle">
          Escribe, evalúa y avanza: Abstracción → Diseño → Implementación →
          Pruebas.
        </p>
        <p class="workspace__cta-row">
          <a class="workspace__cta" href="/exercise?step=py-01-home">
            Micro-pasos Python
          </a>
        </p>
      </header>

      {levelLoading.value && (
        <p class="workspace__status" role="status">
          Cargando enunciado del nivel...
        </p>
      )}

      {levelError.value && (
        <p class="workspace__status workspace__status--error" role="alert">
          {levelError.value}
        </p>
      )}

      {level.value && <LevelDescription level={level.value} />}

      <InteractiveStage
        onTopicChange$={(ctx) => {
          activeTopic.value = ctx;
        }}
      />

      <section
        class="workspace__panel"
        aria-labelledby="editor-heading"
        data-exercise-ref={activeTopic.value?.exerciseRef ?? ""}
        data-active-chapter={activeTopic.value?.chapterId ?? ""}
      >
        <div class="workspace__panel-head">
          <h2 id="editor-heading">Editor de Python</h2>
          <span class="workspace__level">
            {activeTopic.value?.chapterTitle
              ? activeTopic.value.chapterTitle
              : level.value
                ? `Nivel ${level.value.id}`
                : "Sin nivel"}
          </span>
        </div>
        {activeTopic.value?.exerciseRef && (
          <p class="workspace__exercise-ref" aria-live="polite">
            Ejercicio vinculado: <code>{activeTopic.value.exerciseRef}</code>
            {activeTopic.value.chapterTitle
              ? ` · tema “${activeTopic.value.chapterTitle}”`
              : ""}
          </p>
        )}

        <label class="sr-only" for="student-code">
          Código Python del estudiante
        </label>
        <textarea
          id="student-code"
          class="workspace__editor"
          rows={16}
          spellcheck={false}
          bind:value={code}
        />

        <div class="workspace__actions">
          <button
            class="workspace__button"
            type="button"
            disabled={
              isEvaluating.value || levelLoading.value || !level.value
            }
            onClick$={async () => {
              if (!level.value) {
                result.value = "No hay un nivel cargado para evaluar.";
                return;
              }

              isEvaluating.value = true;
              result.value = "Evaluando...";
              feedback.value = "";
              passed.value = null;

              try {
                const response = await fetch(
                  `${API_BASE_URL}/api/evaluate`,
                  {
                    method: "POST",
                    headers: {
                      "Content-Type": "application/json",
                    },
                    body: JSON.stringify({
                      code: code.value,
                      level_id: level.value.id,
                      student_id: DEMO_STUDENT_ID,
                    }),
                  },
                );

                if (!response.ok) {
                  result.value =
                    "Error al evaluar: el servidor respondió con un estado inesperado.";
                  feedback.value = "";
                  return;
                }

                const data = (await response.json()) as EvaluateResponse;
                passed.value = data.passed;
                result.value = data.passed
                  ? "Aprobado ✅ Reto superado"
                  : "Desaprobado ❌ Sigue intentando";
                feedback.value = data.feedback?.trim() ?? "";
              } catch {
                result.value =
                  "No se pudo conectar con el backend en http://localhost:8080.";
                feedback.value = "";
              } finally {
                isEvaluating.value = false;
              }
            }}
          >
            {isEvaluating.value ? "Evaluando..." : "Evaluar Código"}
          </button>
        </div>

        <div
          class={`workspace__result${
            passed.value === true ? " workspace__result--pass" : ""
          }${passed.value === false ? " workspace__result--fail" : ""}`}
          role="status"
          aria-live="polite"
        >
          {result.value || "El resultado de la evaluación aparecerá aquí."}
        </div>

        {feedback.value && (
          <aside
            class={`workspace__feedback${
              passed.value === true ? " workspace__feedback--pass" : ""
            }${passed.value === false ? " workspace__feedback--fail" : ""}`}
            aria-label="Nota del profesor"
          >
            <p class="workspace__feedback-label">Nota del profesor</p>
            <p class="workspace__feedback-text">{feedback.value}</p>
          </aside>
        )}
      </section>
    </main>
  );
});

export const head: DocumentHead = {
  title: BRAND_NAME_PLAIN,
  meta: [
    {
      name: "description",
      content:
        "IngenierIA — interfaz del estudiante para escribir y evaluar ejercicios de Python.",
    },
  ],
};
