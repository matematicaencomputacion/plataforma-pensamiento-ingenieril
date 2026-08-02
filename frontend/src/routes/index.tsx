import { component$, useSignal } from "@builder.io/qwik";
import type { DocumentHead } from "@builder.io/qwik-city";

type EvaluateResponse = {
  passed: boolean;
  feedback?: string;
};

export default component$(() => {
  const code = useSignal(
    '# Escribe tu solución en Python\nprint("Hola, pensamiento ingenieril")\n',
  );
  const result = useSignal("");
  const feedback = useSignal("");
  const isEvaluating = useSignal(false);
  const passed = useSignal<boolean | null>(null);

  return (
    <main class="workspace">
      <header class="workspace__header">
        <p class="workspace__brand">Pensamiento Ingenieril</p>
        <h1 class="workspace__title">Plataforma de Pensamiento Ingenieril</h1>
        <p class="workspace__subtitle">
          Escribe, evalúa y avanza: Abstracción → Diseño → Implementación →
          Pruebas.
        </p>
      </header>

      <section class="workspace__panel" aria-labelledby="editor-heading">
        <div class="workspace__panel-head">
          <h2 id="editor-heading">Editor de Python</h2>
          <span class="workspace__level">Nivel 1</span>
        </div>

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
            disabled={isEvaluating.value}
            onClick$={async () => {
              isEvaluating.value = true;
              result.value = "Evaluando...";
              feedback.value = "";
              passed.value = null;

              try {
                const response = await fetch(
                  "http://localhost:8080/api/evaluate",
                  {
                    method: "POST",
                    headers: {
                      "Content-Type": "application/json",
                    },
                    body: JSON.stringify({
                      code: code.value,
                      level_id: 1,
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
                  ? "Aprobado ✅"
                  : "Desaprobado ❌";
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
          <aside class="workspace__feedback" aria-label="Nota del profesor">
            <p class="workspace__feedback-label">Nota del profesor</p>
            <p class="workspace__feedback-text">{feedback.value}</p>
          </aside>
        )}
      </section>
    </main>
  );
});

export const head: DocumentHead = {
  title: "Plataforma de Pensamiento Ingenieril",
  meta: [
    {
      name: "description",
      content:
        "Interfaz del estudiante para escribir y evaluar ejercicios de Python.",
    },
  ],
};
