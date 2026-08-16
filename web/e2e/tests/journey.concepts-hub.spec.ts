import { expect, test, type Page } from "@playwright/test";
import {
  e2eTimeout,
  fillLeptosInput,
  gotoApp,
  waitForAuthFormReady,
} from "./helpers";

/**
 * Canonical conceptual-hub journey (ADR 003 / Wave D.5).
 *
 * Pages oil-check:
 *   register → P3 `/workspace` → P4 `/concepts/1`
 *   D.1 heatmap (100 cells) + decade drawer
 *   D.2 AND facet updates list + heatmap
 *   D.4 missing-base alert for a fresh learner
 *
 * No mock of STEP_PARTITIONS tags. No Pyodide mock (this path never
 * submits learner Python).
 */

function stampEmail(prefix: string): string {
  return `${prefix}-${Date.now()}-${Math.floor(Math.random() * 1e6)}@example.com`;
}

async function login(page: Page, email: string, password: string) {
  await gotoApp(page, "/login");
  await waitForAuthFormReady(page, {
    emailSelector: "#login-email",
    passwordSelector: "#login-password",
    submitName: "Entrar",
  });
  await fillLeptosInput(page, "#login-email", email);
  await fillLeptosInput(page, "#login-password", password);
  const loginResponse = page.waitForResponse(
    (res) =>
      res.url().includes("/api/auth/login") && res.request().method() === "POST",
    { timeout: e2eTimeout },
  );
  await page.getByRole("button", { name: "Entrar" }).click();
  expect((await loginResponse).ok()).toBeTruthy();
  await expect(page).toHaveURL(/\/workspace/, { timeout: e2eTimeout });
}

test.describe("journey conceptual hub (P4 /concepts/1)", () => {
  test("register → heatmap → decade drawer → AND facets → prereq alert", async ({
    page,
    request,
  }) => {
    const email = stampEmail("journey-concepts-hub");
    const password = process.env.PPI_E2E_PASSWORD?.trim() || "secreto12ci";

    const reg = await request.post("/api/auth/register", {
      data: { email, password },
      timeout: e2eTimeout,
    });
    expect(reg.ok(), await reg.text()).toBeTruthy();

    await login(page, email, password);
    await expect(page.locator("#partition-nav-1")).toBeVisible({
      timeout: e2eTimeout,
    });

    await page.locator("#partition-nav-1").click();
    await expect(page).toHaveURL(/\/concepts\/1/, { timeout: e2eTimeout });
    await expect(
      page.getByRole("heading", { name: /Modelo de Datos/i }),
    ).toBeVisible({ timeout: e2eTimeout });

    const heatmap = page.locator("#concept-heatmap");
    await expect(heatmap).toBeVisible({ timeout: e2eTimeout });
    await expect(page.locator("#concept-heatmap [id^='concept-heat-']")).toHaveCount(
      100,
    );
    await expect(page.locator("#concepts-drill-list")).toBeVisible();
    await expect(page.locator("#concept-prereq-alert")).toBeVisible({
      timeout: e2eTimeout,
    });
    await expect(page.locator("#concept-prereq-alert")).toHaveAttribute(
      "role",
      "alert",
    );

    const decadeCell = page.locator("#concept-heat-1");
    await expect(decadeCell).toBeVisible({ timeout: e2eTimeout });
    await expect(decadeCell).toHaveAttribute(
      "data-state",
      /pending|partial|done/,
    );
    await decadeCell.click();
    await expect(page).toHaveURL(/\/concepts\/1/);
    const drawer = page.locator("#concept-decade-drawer");
    await expect(drawer).toBeVisible();
    await expect(drawer).toHaveAttribute("role", "dialog");

    await page.keyboard.press("Escape");
    await expect(page.locator("#concept-decade-drawer")).toHaveCount(0);

    await expect(page.locator("#concept-facet-bar")).toBeVisible({
      timeout: e2eTimeout,
    });
    const unfilteredHits = await page
      .locator("#concept-heatmap [data-facet='hit']")
      .count();
    expect(unfilteredHits).toBeGreaterThan(1);

    await page.locator("#concept-facet-p3").click();
    await expect(page.locator("#concept-facet-p3")).toHaveAttribute(
      "aria-pressed",
      "true",
    );
    await expect(page.locator("#concepts-drill-20")).toHaveCount(0);
    const andItems = page.locator("#concepts-drill-list [data-partitions]");
    const andCount = await andItems.count();
    expect(andCount).toBeGreaterThan(0);
    for (let i = 0; i < andCount; i++) {
      const tags = (await andItems.nth(i).getAttribute("data-partitions")) ?? "";
      expect(tags.split(",")).toEqual(expect.arrayContaining(["1", "3"]));
    }
    const filteredHits = await page
      .locator("#concept-heatmap [data-facet='hit']")
      .count();
    expect(filteredHits).toBeGreaterThan(0);
    expect(filteredHits).toBeLessThan(unfilteredHits);

    await page.locator("#concept-facet-clear").click();
    await expect(page.locator("#concept-prereq-alert")).toBeVisible({
      timeout: e2eTimeout,
    });
    await expect(page.locator("#concept-prereq-alert")).toHaveAttribute(
      "role",
      "alert",
    );
    await expect(page.locator("#concept-prereq-alert")).toContainText(
      /mutabilidad/i,
    );
  });
});
