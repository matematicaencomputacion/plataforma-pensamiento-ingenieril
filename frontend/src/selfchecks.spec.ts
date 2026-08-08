import { describe, it } from "vitest";

/**
 * Puente Vitest ↔ selfchecks: cada `*.selfcheck.ts` ejecuta sus asserts
 * (node:assert/strict) al importarse y lanza AssertionError si algo falla.
 * El glob los descubre automáticamente, así los selfchecks nuevos quedan
 * cubiertos por `npm test` y por CI sin registro manual.
 */
const selfchecks = import.meta.glob("./**/*.selfcheck.ts");

describe("selfchecks", () => {
  const entries = Object.entries(selfchecks);

  it("descubre al menos un selfcheck", () => {
    if (entries.length === 0) {
      throw new Error("No se encontró ningún *.selfcheck.ts bajo src/");
    }
  });

  for (const [path, load] of entries) {
    it(path, async () => {
      await load();
    });
  }
});
