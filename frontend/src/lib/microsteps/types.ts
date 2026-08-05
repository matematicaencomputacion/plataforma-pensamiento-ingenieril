/** Contrato tipado de la semilla de micro-pasos (Python Foundations). */

export type CheckMode =
  | "mcq_or_run"
  | "pytest"
  | "pytest_plus_optional_mcq";

export type McqOptionBank = {
  id: string;
  prompt: string;
  options: string[];
  correct: string;
  source?: string;
};

/** Opción de Check rápido con feedback de alineación (1–5 fuegos). */
export type AdaptiveMcqOption = {
  key: string;
  text: string;
  feedback: string;
  alignmentScore: number;
};

export type AdaptiveMcq = {
  question?: string;
  options: AdaptiveMcqOption[];
};

export type StepChecks = {
  mode: CheckMode;
  pytest?: string;
  mcq?: {
    correct: string;
    rationale?: string;
  };
};

export type LayoutType = "onboarding" | "coding";

export type StepContent = {
  prompt_md: string;
  starter_code: string;
  mcq_bank?: McqOptionBank[];
  /** MCQ adaptativo (feedback + alignmentScore) en el panel de enunciado. */
  adaptive_mcq?: AdaptiveMcq;
  /** Guiones / preguntas del coaching Rogeriano (solo onboarding). */
  coaching_prompts?: string[];
};

export type Microstep = {
  id: string;
  step_number: number;
  slug?: string;
  title: string;
  /** Bifurca la UI: entrevista de propósito vs workspace de código. */
  layoutType: LayoutType;
  type?: string;
  objective?: string;
  w3_ref?: string;
  w3_exercise?: string;
  content: StepContent;
  checks: StepChecks;
  hint?: string;
  solution_example?: string;
  tags?: string[];
  next?: string;
  frontier_note?: string;
};

export type MicrostepSeedMetadata = {
  id: string;
  title: string;
  description?: string;
  version: string;
  status?: string;
  total_steps: number;
  locale?: string;
};

export type MicrostepSeed = {
  metadata: MicrostepSeedMetadata;
  steps: Microstep[];
  ux?: Record<string, unknown>;
  roadmap_after_seed?: unknown[];
  iteration_backlog?: unknown[];
};

/** Payload listo para el runner client-side (Pyodide en Bloque 3). */
export type NormalizedCheckPayload = {
  stepId: string;
  studentCode: string;
  testSource: string | null;
  mode: CheckMode;
  mcq: StepChecks["mcq"] | null;
  mcqBank: McqOptionBank[];
};
