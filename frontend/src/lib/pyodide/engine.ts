/**
 * Motor Python client-side (Pyodide / WebAssembly).
 * Solo debe ejecutarse en el navegador — nunca en SSR de Qwik.
 *
 * Estrategia de Check: escribir `solution.py` en el FS virtual y correr
 * tests con forma pytest (`def test_*(capsys)`) vía un harness ligero
 * compatible (sin instalar pytest por micropip en el primer load).
 */

export const PYODIDE_VERSION = "0.27.7";
export const PYODIDE_INDEX_URL = `https://cdn.jsdelivr.net/pyodide/v${PYODIDE_VERSION}/full/`;

export type PyodideEngineStatus =
  | "idle"
  | "loading"
  | "ready"
  | "error";

export type RunResult = {
  ok: boolean;
  stdout: string;
  stderr: string;
  error?: string;
};

export type CheckResult = {
  passed: boolean;
  stdout: string;
  stderr: string;
  summary: string;
  details: string;
};

type PyodideLike = {
  runPython: (code: string) => unknown;
  runPythonAsync: (code: string) => Promise<unknown>;
  setStdout: (opts: {
    batched?: (text: string) => void;
    raw?: (charCode: number) => void;
  }) => void;
  setStderr: (opts: {
    batched?: (text: string) => void;
    raw?: (charCode: number) => void;
  }) => void;
  FS: {
    writeFile: (path: string, data: string | Uint8Array) => void;
  };
};

type LoadPyodideFn = (config: {
  indexURL: string;
}) => Promise<PyodideLike>;

let loadPromise: Promise<PyodideLike> | null = null;
let instance: PyodideLike | null = null;
let lastError: string | null = null;

function isBrowser(): boolean {
  return typeof window !== "undefined" && typeof document !== "undefined";
}

async function injectPyodideScript(): Promise<LoadPyodideFn> {
  const existing = (globalThis as { loadPyodide?: LoadPyodideFn }).loadPyodide;
  if (existing) {
    return existing;
  }

  await new Promise<void>((resolve, reject) => {
    const script = document.createElement("script");
    script.src = `${PYODIDE_INDEX_URL}pyodide.js`;
    script.async = true;
    script.onload = () => resolve();
    script.onerror = () =>
      reject(new Error(`No se pudo cargar Pyodide desde ${PYODIDE_INDEX_URL}`));
    document.head.appendChild(script);
  });

  const loadPyodide = (globalThis as { loadPyodide?: LoadPyodideFn }).loadPyodide;
  if (!loadPyodide) {
    throw new Error("Pyodide cargó pero loadPyodide no está disponible");
  }
  return loadPyodide;
}

/** Carga lazy (singleton) del runtime Python en el navegador. */
export async function ensurePyodide(): Promise<PyodideLike> {
  if (!isBrowser()) {
    throw new Error("Pyodide solo está disponible en el navegador");
  }
  if (instance) {
    return instance;
  }
  if (loadPromise) {
    return loadPromise;
  }

  loadPromise = (async () => {
    try {
      const loadPyodide = await injectPyodideScript();
      const pyodide = await loadPyodide({ indexURL: PYODIDE_INDEX_URL });
      instance = pyodide;
      lastError = null;
      return pyodide;
    } catch (err) {
      loadPromise = null;
      lastError = err instanceof Error ? err.message : String(err);
      throw err;
    }
  })();

  return loadPromise;
}

export function getPyodideLoadError(): string | null {
  return lastError;
}

export function isPyodideReady(): boolean {
  return instance !== null;
}

function collectIo(): {
  stdout: string[];
  stderr: string[];
  attach: (py: PyodideLike) => void;
  join: () => { stdout: string; stderr: string };
} {
  const stdout: string[] = [];
  const stderr: string[] = [];
  return {
    stdout,
    stderr,
    attach(py) {
      py.setStdout({
        batched: (text) => {
          stdout.push(text);
        },
      });
      py.setStderr({
        batched: (text) => {
          stderr.push(text);
        },
      });
    },
    join() {
      return {
        stdout: stdout.join("").replace(/\r\n/g, "\n"),
        stderr: stderr.join("").replace(/\r\n/g, "\n"),
      };
    },
  };
}

/** Ejecuta el código del alumno y captura stdout/stderr. */
export async function runStudentCode(code: string): Promise<RunResult> {
  const py = await ensurePyodide();
  const io = collectIo();
  io.attach(py);

  try {
    await py.runPythonAsync(code);
    const { stdout, stderr } = io.join();
    return { ok: true, stdout, stderr };
  } catch (err) {
    const { stdout, stderr } = io.join();
    const message = err instanceof Error ? err.message : String(err);
    return {
      ok: false,
      stdout,
      stderr: stderr || message,
      error: message,
    };
  }
}

/**
 * Harness que ejecuta funciones test_* con fixture capsys compatible
 * con la semilla (open('solution.py') / exec + asserts).
 */
export function buildCheckHarnessPython(): string {
  return `
import io
import sys
import types
import traceback
import importlib.util

failures = []
passed = 0

spec = importlib.util.spec_from_file_location("student_tests", "test_step.py")
mod = importlib.util.module_from_spec(spec)
assert spec.loader is not None
spec.loader.exec_module(mod)

test_names = sorted(n for n in dir(mod) if n.startswith("test_") and callable(getattr(mod, n)))
if not test_names:
    failures.append("No se encontró ninguna función test_* en el micro-reto.")

for name in test_names:
    fn = getattr(mod, name)
    buf_out = io.StringIO()
    buf_err = io.StringIO()
    old_out, old_err = sys.stdout, sys.stderr
    sys.stdout, sys.stderr = buf_out, buf_err

    class Capsys:
        def readouterr(self):
            out = buf_out.getvalue()
            err = buf_err.getvalue()
            buf_out.seek(0)
            buf_out.truncate(0)
            buf_err.seek(0)
            buf_err.truncate(0)
            return types.SimpleNamespace(out=out, err=err)

    try:
        import inspect
        params = inspect.signature(fn).parameters
        kwargs = {}
        if "capsys" in params:
            kwargs["capsys"] = Capsys()
        fn(**kwargs)
        passed += 1
        print(f"PASSED {name}")
    except Exception:
        failures.append(f"FAILED {name}\\n{traceback.format_exc()}")
        print(f"FAILED {name}")
    finally:
        sys.stdout, sys.stderr = old_out, old_err

if failures:
    print("---")
    for f in failures:
        print(f)
else:
    print(f"OK — {passed} test(s) passed")

_CHECK_PASSED = len(failures) == 0
`.trim();
}

/** Evalúa el código del alumno contra el testSource del micro-reto. */
export async function checkStudentCode(
  studentCode: string,
  testSource: string,
): Promise<CheckResult> {
  const py = await ensurePyodide();
  const io = collectIo();
  io.attach(py);

  py.FS.writeFile("solution.py", studentCode);
  py.FS.writeFile("test_step.py", testSource);
  py.FS.writeFile("_check_harness.py", buildCheckHarnessPython());

  try {
    await py.runPythonAsync(
      "exec(open('_check_harness.py', encoding='utf-8').read(), globals())",
    );
    const passed = Boolean(py.runPython("_CHECK_PASSED"));
    const { stdout, stderr } = io.join();
    if (passed) {
      return {
        passed: true,
        stdout,
        stderr,
        summary: "✓ Checks OK — podés Continuar",
        details: stdout.trim() || "Todos los tests pasaron.",
      };
    }
    return {
      passed: false,
      stdout,
      stderr,
      summary: "✗ Checks fallaron — revisá el enunciado y el código",
      details: [stdout.trim(), stderr.trim()].filter(Boolean).join("\n\n"),
    };
  } catch (err) {
    const { stdout, stderr } = io.join();
    const message = err instanceof Error ? err.message : String(err);
    const details = [stdout.trim(), stderr.trim(), message]
      .filter(Boolean)
      .join("\n\n");
    return {
      passed: false,
      stdout,
      stderr: stderr || message,
      summary: "✗ Error al validar — revisá la sintaxis o el runtime",
      details,
    };
  }
}

/** Formatea un RunResult para el panel de resultados. */
export function formatRunLog(result: RunResult): string {
  const parts: string[] = ["=== Run ==="];
  if (result.stdout.trim()) {
    parts.push(result.stdout.replace(/\n$/, ""));
  } else if (result.ok) {
    parts.push("(sin salida — usá print(...) para ver texto aquí)");
  }
  if (result.stderr.trim()) {
    parts.push("--- stderr ---");
    parts.push(result.stderr.replace(/\n$/, ""));
  }
  if (!result.ok && result.error) {
    parts.push("--- error ---");
    parts.push(result.error);
  }
  if (result.ok) {
    parts.push("");
    parts.push("✓ Ejecución finalizada");
  }
  return parts.join("\n");
}

/** Formatea un CheckResult para el panel de resultados. */
export function formatCheckLog(result: CheckResult): string {
  const parts: string[] = ["=== Validar ===", result.summary, ""];
  if (result.details.trim()) {
    parts.push(result.details.replace(/\n$/, ""));
  }
  return parts.join("\n");
}
