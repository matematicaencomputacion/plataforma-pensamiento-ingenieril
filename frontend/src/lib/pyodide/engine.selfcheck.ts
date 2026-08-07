/**
 * Self-check del motor Pyodide (sin browser / sin Wasm).
 * Ejecutar: cd frontend && npx --yes tsx src/lib/pyodide/engine.selfcheck.ts
 */
import assert from "node:assert/strict";
import {
  buildCheckHarnessPython,
  formatCheckLog,
  formatRunLog,
  pyodideStatusMessage,
} from "./index";

function main() {
  const harness = buildCheckHarnessPython();
  assert.ok(harness.includes("test_step.py"));
  assert.ok(harness.includes("capsys"));
  assert.ok(harness.includes("solution.py") === false); // harness no hardcodea solution; tests sí

  const runOk = formatRunLog({
    ok: true,
    stdout: "Hello, World!\n",
    stderr: "",
  });
  assert.ok(runOk.includes("Hello, World!"));
  assert.ok(runOk.includes("Ejecución finalizada"));

  const runFail = formatRunLog({
    ok: false,
    stdout: "",
    stderr: "",
    error: "NameError: name 'x' is not defined",
  });
  assert.ok(runFail.includes("NameError"));

  const checkPass = formatCheckLog({
    passed: true,
    stdout: "OK — 1 test(s) passed\n",
    stderr: "",
    summary: "✓ Checks OK — podés Continuar",
    details: "OK — 1 test(s) passed",
  });
  assert.ok(checkPass.includes("Validar"));
  assert.ok(checkPass.includes("Continuar"));

  assert.equal(
    pyodideStatusMessage("loading").includes("Preparando motor Python"),
    true,
  );
  assert.equal(pyodideStatusMessage("ready").includes("listo"), true);

  console.log("pyodide.engine.selfcheck: OK");
}

main();
