/**
 * Self-check del split de encabezado Variables.
 * Ejecutar: cd frontend && npx --yes tsx src/components/exercise-workspace/python-type-catalog.selfcheck.ts
 */
import assert from "node:assert/strict";
import {
  splitVariablesPromptHeading,
  stepShowsPythonTypeChips,
} from "./python-type-catalog";

function main() {
  assert.equal(stepShowsPythonTypeChips("py-02-variables", "x"), true);

  const sample =
    "**Variables**\n\nUna variable guarda un valor.\n\n**Micro-reto:**\n1. Crea `nombre`";
  const parts = splitVariablesPromptHeading(sample);
  assert.equal(parts.heading, "Variables");
  assert.ok(parts.body.startsWith("Una variable"));
  assert.ok(!parts.body.includes("**Variables**"));

  const plain = splitVariablesPromptHeading("Sin encabezado especial");
  assert.equal(plain.heading, null);
  assert.equal(plain.body, "Sin encabezado especial");

  console.log("python-type-catalog.selfcheck: OK");
}

main();
