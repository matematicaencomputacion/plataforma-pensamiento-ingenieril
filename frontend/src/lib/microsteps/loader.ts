import seedJson from "../../data/python-foundations-microsteps.json";
import type {
  LayoutType,
  Microstep,
  MicrostepSeed,
  NormalizedCheckPayload,
} from "./types";

const seed = seedJson as MicrostepSeed;

/** Normaliza layoutType; default `coding` si falta (semillas antiguas). */
export function getLayoutType(step: Microstep): LayoutType {
  return step.layoutType === "onboarding" ? "onboarding" : "coding";
}

/** Semilla runtime versionada (copia de docs/seeds v0.2). */
export function getMicrostepSeed(): MicrostepSeed {
  return seed;
}

export function listMicrosteps(): Microstep[] {
  return seed.steps;
}

/**
 * Resuelve un step por id. Si no existe, cae al primer step del seed
 * (comportamiento recuperable según spec microstep-seed-runtime).
 */
export function resolveStep(stepId: string | null | undefined): {
  step: Microstep;
  fallback: boolean;
} {
  const steps = seed.steps;
  if (!steps.length) {
    throw new Error("Microstep seed has no steps");
  }
  if (stepId) {
    const found = steps.find((s) => s.id === stepId);
    if (found) {
      return { step: found, fallback: false };
    }
  }
  return { step: steps[0], fallback: true };
}

export function isFrontierNext(nextId: string | undefined): boolean {
  if (!nextId) {
    return true;
  }
  return !seed.steps.some((s) => s.id === nextId);
}

/**
 * Adapta checks autoriales (placeholders exec(open('solution.py'))) a un
 * payload inyectable en Pyodide: el código del alumno + testSource.
 */
export function normalizeCheckPayload(
  step: Microstep,
  studentCode: string,
): NormalizedCheckPayload {
  const raw = step.checks.pytest ?? null;
  const testSource = raw
    ? adaptPytestPlaceholder(raw, studentCode)
    : null;

  return {
    stepId: step.id,
    studentCode,
    testSource,
    mode: step.checks.mode,
    mcq: step.checks.mcq ?? null,
    mcqBank: step.content.mcq_bank ?? [],
  };
}

/**
 * Reemplaza el patrón de curaduría:
 *   exec(open('solution.py', encoding='utf-8').read())
 * por una asignación/ejecución del código del alumno en el namespace de test.
 *
 * Nota: el runner Pyodide (Bloque 3) inyectará `STUDENT_CODE` / archivo virtual;
 * aquí dejamos un string ejecutable y portable.
 */
export function adaptPytestPlaceholder(
  pytestSource: string,
  _studentCode: string,
): string {
  const patterns = [
    /exec\(\s*open\(\s*['"]solution\.py['"]\s*(?:,\s*encoding\s*=\s*['"]utf-8['"])?\s*\)\.read\(\s*\)\s*\)/g,
    /exec\(\s*open\(\s*['"]solution\.py['"]\s*\)\.read\(\s*\)\s*\)/g,
  ];

  let adapted = pytestSource;
  for (const re of patterns) {
    adapted = adapted.replace(
      re,
      "exec(STUDENT_CODE, student_ns); globals().update(student_ns)",
    );
  }

  // Prefijo: namespace del alumno + código inyectado por el runner.
  const preamble = [
    "# Normalized by microstep loader — STUDENT_CODE lo provee el runner.",
    "student_ns = {}",
    "",
  ].join("\n");

  if (adapted.includes("STUDENT_CODE")) {
    return `${preamble}${adapted}`;
  }

  // Si no había placeholder, devolver tests tal cual (+ preamble vacío útil).
  return `${preamble}${adapted}`;
}

export function getSeedStepCount(): number {
  return seed.metadata.total_steps;
}
