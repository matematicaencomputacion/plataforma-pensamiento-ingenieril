/** Dominio de una herramienta del stack: 0 desconocido → 1 dominado → 2 experto. */
export type ToolMastery = 0 | 1 | 2;

export type StackToolId =
  | "github"
  | "cursor"
  | "positron"
  | "rstudio"
  | "colab"
  | "jupyter"
  | "python";

export type StackTool = {
  id: StackToolId;
  label: string;
};

export const STACK_TOOLS: readonly StackTool[] = [
  { id: "github", label: "GitHub" },
  { id: "cursor", label: "Cursor" },
  { id: "positron", label: "Positron" },
  { id: "rstudio", label: "R/RStudio" },
  { id: "colab", label: "Colab" },
  { id: "jupyter", label: "Jupyter" },
  { id: "python", label: "Python" },
] as const;

export const MASTERY_LABELS: Record<ToolMastery, string> = {
  0: "Pendiente",
  1: "Dominado",
  2: "Nivel Experto 🔥",
};

export function nextMastery(current: ToolMastery): ToolMastery {
  return ((current + 1) % 3) as ToolMastery;
}

export function masteryClass(level: ToolMastery): string {
  switch (level) {
    case 1:
      return "tool-chip--mastered";
    case 2:
      return "tool-chip--expert";
    default:
      return "tool-chip--unknown";
  }
}

export type ToolStates = Record<StackToolId, ToolMastery>;

export function initialToolStates(): ToolStates {
  return {
    github: 0,
    cursor: 0,
    positron: 0,
    rstudio: 0,
    colab: 0,
    jupyter: 0,
    python: 0,
  };
}
