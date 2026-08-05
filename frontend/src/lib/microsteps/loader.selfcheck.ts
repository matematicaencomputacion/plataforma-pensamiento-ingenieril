/**
 * Self-check del loader (sin Vitest en el frontend aún).
 * Ejecutar: cd frontend && npx --yes tsx src/lib/microsteps/loader.selfcheck.ts
 */
import assert from "node:assert/strict";
import {
  adaptPytestPlaceholder,
  getLayoutType,
  getMicrostepSeed,
  getSeedStepCount,
  isFrontierNext,
  listMicrosteps,
  normalizeCheckPayload,
  resolveStep,
} from "./loader";
function main() {
  const seed = getMicrostepSeed();
  assert.equal(seed.metadata.id, "seed:python-foundations-microsteps");
  assert.equal(getSeedStepCount(), listMicrosteps().length);
  assert.ok(listMicrosteps().length >= 11);

  const home = resolveStep("py-01-home");
  assert.equal(home.step.id, "py-01-home");
  assert.equal(getLayoutType(home.step), "onboarding");
  assert.equal(home.step.next, "py-02-variables");

  const bridge = resolveStep("py-02-variables").step;
  assert.equal(getLayoutType(bridge), "coding");
  assert.equal(bridge.next, "py-02-intro");

  const missing = resolveStep("py-does-not-exist");
  assert.equal(missing.step.id, "py-01-home");
  assert.equal(missing.fallback, true);

  const casting = resolveStep("py-10-casting").step;
  assert.equal(casting.id, "py-10-casting");
  assert.equal(isFrontierNext(casting.next), true);

  const raw =
    "def test_hello(capsys):\n    exec(open('solution.py', encoding='utf-8').read())\n    assert True\n";
  const adapted = adaptPytestPlaceholder(raw, 'print("hi")');
  assert.ok(!adapted.includes("open('solution.py'"));
  assert.ok(adapted.includes("STUDENT_CODE"));

  const payload = normalizeCheckPayload(
    casting,
    "x = 1\na = float(x)\nb = str(x)\nprint(a)\nprint(b)",
  );
  assert.equal(payload.stepId, "py-10-casting");
  assert.ok(payload.testSource);

  console.log("loader.selfcheck: OK");
}

main();
