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
