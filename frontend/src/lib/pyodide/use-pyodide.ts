/**
 * Integración Pyodide orientada a Qwik (no es un hook de React).
 * El nombre `use-pyodide` documenta el punto de entrada de la rebanada;
 * el estado de UI vive en `useStore` del exercise-workspace.
 */
import {
  checkStudentCode,
  ensurePyodide,
  formatCheckLog,
  formatRunLog,
  getPyodideLoadError,
  isPyodideReady,
  runStudentCode,
  type CheckResult,
  type PyodideEngineStatus,
  type RunResult,
} from "./engine";

export type {
  CheckResult,
  PyodideEngineStatus,
  RunResult,
};

export {
  checkStudentCode,
  ensurePyodide,
  formatCheckLog,
  formatRunLog,
  getPyodideLoadError,
  isPyodideReady,
  runStudentCode,
};

export type PyodideReadyState = {
  status: PyodideEngineStatus;
  message: string;
};

/** Mensaje amigable según fase de carga del motor. */
export function pyodideStatusMessage(status: PyodideEngineStatus): string {
  switch (status) {
    case "idle":
      return "Motor Python en espera.";
    case "loading":
      return "Preparando motor Python… (primera carga puede tardar unos segundos)";
    case "ready":
      return "Motor Python listo. Escribí en el editor y usá Run o Validar.";
    case "error":
      return (
        getPyodideLoadError() ??
        "No se pudo iniciar Pyodide. Revisá la red o recargá la página."
      );
    default: {
      const _exhaustive: never = status;
      return _exhaustive;
    }
  }
}

/**
 * Asegura Pyodide listo y devuelve estado de UI.
 * Pensado para llamarse desde `useVisibleTask$` del workspace.
 */
export async function bootstrapPyodide(): Promise<PyodideReadyState> {
  if (isPyodideReady()) {
    return { status: "ready", message: pyodideStatusMessage("ready") };
  }
  try {
    await ensurePyodide();
    return { status: "ready", message: pyodideStatusMessage("ready") };
  } catch {
    return { status: "error", message: pyodideStatusMessage("error") };
  }
}
