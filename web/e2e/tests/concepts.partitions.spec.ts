import { expect, test, type Page } from "@playwright/test";
import { fillLeptosInput } from "./helpers";

const e2eTimeout = 45_000;

function uniqueCreds() {
  const stamp = Date.now();
  return {
    email: `concepts-${stamp}@example.com`,
    password: process.env.PPI_E2E_PASSWORD?.trim() || "secreto12ci",
  };
}

async function login(page: Page, email: string, password: string) {
  await page.goto("/login");
  await page.locator("#login-email").fill(email);
  await page.locator("#login-password").fill(password);
  await page.getByRole("button", { name: "Entrar" }).click();
  await page.waitForURL(/\/workspace/, { timeout: e2eTimeout });
}

test.describe("conceptual partitions hub", () => {
  test("compass opens partition hub and drill lands on learn", async ({
    page,
    request,
  }) => {
    const { email, password } = uniqueCreds();
    const reg = await request.post("/api/auth/register", {
      data: { email, password },
      timeout: e2eTimeout,
    });
    expect(reg.ok(), await reg.text()).toBeTruthy();

    await login(page, email, password);

    await expect(page.locator("#partition-nav-1")).toBeVisible({
      timeout: e2eTimeout,
    });
    await expect(page.locator(".partition-nav")).toBeVisible();

    await page.locator("#partition-nav-1").click();
    await expect(page).toHaveURL(/\/concepts\/1/, { timeout: e2eTimeout });
    await expect(
      page.getByRole("heading", { name: /Modelo de Datos/i }),
    ).toBeVisible();
    await expect(page.locator("#concept-heatmap")).toBeVisible();
    await expect(page.locator("#concept-heatmap [id^='concept-heat-']")).toHaveCount(
      100,
    );
    await expect(page.locator("#concepts-drill-list")).toBeVisible();
    await expect(page.locator("#concepts-drill-20")).toBeVisible();
    await expect(page.locator("#concept-decade-drawer")).toHaveCount(0);

    const emptyCell = page.locator("#concept-heatmap [data-state='empty']").first();
    await expect(emptyCell).toBeVisible({ timeout: e2eTimeout });
    await expect(emptyCell).toBeDisabled();
    await emptyCell.click({ force: true });
    await expect(page).toHaveURL(/\/concepts\/1/);
    await expect(page.locator("#concept-decade-drawer")).toHaveCount(0);

    const decadeCell = page.locator("#concept-heat-1");
    await expect(decadeCell).toBeVisible({ timeout: e2eTimeout });
    await expect(decadeCell).toHaveAttribute("data-state", /pending|partial|done/);
    const lo = 1;
    const hi = 10;

    await decadeCell.click();
    await expect(page).toHaveURL(/\/concepts\/1/);
    const drawer = page.locator("#concept-decade-drawer");
    await expect(drawer).toBeVisible();
    await expect(drawer).toHaveAttribute("role", "dialog");
    await expect(page.locator("#concept-decade-title")).toContainText(
      `${lo}–${hi}`,
    );
    await expect(page.locator("#concepts-drill-list")).toBeVisible();

    const items = page.locator("#concept-decade-list [data-micro]");
    const count = await items.count();
    expect(count).toBeGreaterThan(0);
    expect(count).toBeLessThanOrEqual(10);
    for (let i = 0; i < count; i++) {
      const micro = Number(await items.nth(i).getAttribute("data-micro"));
      expect(micro).toBeGreaterThanOrEqual(lo);
      expect(micro).toBeLessThanOrEqual(hi);
    }

    await page.keyboard.press("Escape");
    await expect(page.locator("#concept-decade-drawer")).toHaveCount(0);
    await expect(page).toHaveURL(/\/concepts\/1/);

    await decadeCell.click();
    await expect(page.locator("#concept-decade-drawer")).toBeVisible();
    await items.first().click();
    await expect(page).toHaveURL(/\/learn\/.+/, { timeout: e2eTimeout });

    await page.goto("/concepts/1");
    await expect(page).toHaveURL(/\/concepts\/1/, { timeout: e2eTimeout });
    await expect(page.locator("#concepts-drill-list")).toBeVisible();
    await expect(page.locator("#concepts-drill-20")).toBeVisible();

    await page.locator("#concepts-drill-20").click();
    await expect(page).toHaveURL(/\/learn\/py-20-list-change/, {
      timeout: e2eTimeout,
    });
    await expect(page.locator(".partition-badge").first()).toBeVisible({
      timeout: e2eTimeout,
    });
    await expect(page.locator("#partition-nav-1")).toHaveClass(
      /partition-nav__btn--active/,
    );
  });

  test("facet search filters list and heatmap then opens learn", async ({
    page,
    request,
  }) => {
    const { email, password } = uniqueCreds();
    const reg = await request.post("/api/auth/register", {
      data: { email, password },
      timeout: e2eTimeout,
    });
    expect(reg.ok(), await reg.text()).toBeTruthy();

    await login(page, email, password);

    await page.locator("#partition-nav-1").click();
    await expect(page).toHaveURL(/\/concepts\/1/, { timeout: e2eTimeout });
    await expect(page.locator("#concept-facet-bar")).toBeVisible({
      timeout: e2eTimeout,
    });
    await expect(page.locator("#concept-facet-query")).toBeVisible();
    await expect(page.locator("#concept-heatmap")).toBeVisible();
    await expect(page.locator("#concepts-drill-list")).toBeVisible();
    await expect(page.locator("#concepts-drill-20")).toBeVisible();
    await expect(page.locator("#concepts-drill-1")).toBeVisible();

    const unfilteredHits = await page
      .locator("#concept-heatmap [data-facet='hit']")
      .count();
    expect(unfilteredHits).toBeGreaterThan(1);

    await fillLeptosInput(page, "#concept-facet-query", "append");

    await expect(page.locator("#concepts-drill-20")).toBeVisible({
      timeout: e2eTimeout,
    });
    await expect(page.locator("#concepts-drill-1")).toHaveCount(0);
    await expect(page.locator("#concept-heat-11")).toHaveAttribute(
      "data-facet",
      "hit",
    );

    const filteredHits = await page
      .locator("#concept-heatmap [data-facet='hit']")
      .count();
    expect(filteredHits).toBeGreaterThan(0);
    expect(filteredHits).toBeLessThan(unfilteredHits);

    await page.locator("#concept-facet-clear").click();
    await expect(page.locator("#concepts-drill-1")).toBeVisible({
      timeout: e2eTimeout,
    });

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

    await page.locator("#concept-facet-clear").click();
    await fillLeptosInput(page, "#concept-facet-query", "append");
    await expect(page.locator("#concepts-drill-20")).toBeVisible({
      timeout: e2eTimeout,
    });
    await page.locator("#concepts-drill-20").click();
    await expect(page).toHaveURL(/\/learn\/py-20-list-change/, {
      timeout: e2eTimeout,
    });
    await expect(page.locator("#concept-fab")).toBeVisible({
      timeout: e2eTimeout,
    });
  });

  test("heatmap decade open surfaces analytics hint", async ({
    page,
    request,
  }) => {
    const { email, password } = uniqueCreds();
    const reg = await request.post("/api/auth/register", {
      data: { email, password },
      timeout: e2eTimeout,
    });
    expect(reg.ok(), await reg.text()).toBeTruthy();

    await login(page, email, password);

    await page.locator("#partition-nav-1").click();
    await expect(page).toHaveURL(/\/concepts\/1/, { timeout: e2eTimeout });
    await expect(page.locator("#concept-analytics")).toBeVisible({
      timeout: e2eTimeout,
    });
    await expect(page.locator("#concept-analytics-hint")).toBeVisible();
    await expect(page.locator("#concept-heatmap")).toBeVisible();
    await expect(page.locator("#concept-facet-bar")).toBeVisible();

    await page.locator("#concept-heat-1").click();
    await expect(page.locator("#concept-decade-drawer")).toBeVisible();
    await expect(page.locator("#concept-analytics-hint")).toHaveAttribute(
      "data-hint",
      /^(decade|partition)$/,
      { timeout: e2eTimeout },
    );
    await expect(page.locator("#concept-analytics-hint")).toContainText(
      /fricción|Década|Partición/i,
    );

    const token = await page.evaluate(() =>
      window.localStorage.getItem("ppi.auth.token"),
    );
    expect(token).toBeTruthy();
    const summary = await request.get("/api/concept-analytics", {
      headers: { Authorization: `Bearer ${token}` },
      timeout: e2eTimeout,
    });
    expect(summary.status(), await summary.text()).toBe(200);
    const body = await summary.json();
    expect(body.bottleneck).toBeTruthy();
    expect(body.bottleneck.friction).toBeGreaterThan(0);

    await page.goto("/learn/py-20-list-change");
    await expect(page.locator("#concept-fab")).toBeVisible({
      timeout: e2eTimeout,
    });
  });
});
