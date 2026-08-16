import { expect, test, type Page } from "@playwright/test";

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
    if ((await emptyCell.count()) > 0) {
      await expect(emptyCell).toBeDisabled();
      await emptyCell.click({ force: true });
      await expect(page).toHaveURL(/\/concepts\/1/);
      await expect(page.locator("#concept-decade-drawer")).toHaveCount(0);
    }

    const decadeCell = page
      .locator("#concept-heatmap [data-state]:not([data-state='empty'])")
      .first();
    const cellId = await decadeCell.getAttribute("id");
    expect(cellId).toMatch(/^concept-heat-\d+$/);
    const lo = Number(cellId?.replace("concept-heat-", ""));
    const hi = lo + 9;

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
});
