import { expect, test } from "@playwright/test";
import { gotoApp } from "./helpers";

/**
 * Auth navigation must be unidirectional — no /workspace ↔ / bounce.
 * Seeds session via API + localStorage to isolate route-guard behavior.
 */
test.describe("session navigation", () => {
  test("portada stays put when authenticated; logout clears without bounce", async ({
    page,
    request,
  }) => {
    const email = `nav-${Date.now()}@example.com`;
    const password = "secreto12";

    const reg = await request.post("/api/auth/register", {
      data: { email, password },
    });
    expect(reg.ok(), await reg.text()).toBeTruthy();
    const body = (await reg.json()) as { token: string };
    expect(body.token).toBeTruthy();

    await page.goto("/");
    await page.evaluate((token) => {
      localStorage.setItem("ppi.auth.token", token);
    }, body.token);

    await gotoApp(page, "/workspace");
    await expect(page.getByRole("heading", { name: "Workspace" })).toBeVisible({
      timeout: 20_000,
    });
    await expect(page.locator(".session-bar__email")).toContainText(email);

    await page.getByRole("link", { name: "Volver a la portada" }).click();
    await page.waitForURL((url) => new URL(url).pathname === "/");
    await expect(page.getByRole("heading", { name: "IngenierIA" })).toBeVisible();
    await expect(page.locator(".session-bar__email")).toContainText(email);

    // Stay on portada — no guard should yank us back to /workspace.
    await page.waitForTimeout(1000);
    await expect(page).toHaveURL((url) => new URL(url).pathname === "/");

    await page.getByRole("button", { name: "Salir" }).click();
    await expect(page.getByRole("link", { name: "Iniciar sesión" }).first()).toBeVisible({
      timeout: 10_000,
    });
    await expect(page.locator(".session-bar__email")).toHaveCount(0);

    await page.waitForTimeout(1000);
    await expect(page).toHaveURL((url) => new URL(url).pathname === "/");
    const token = await page.evaluate(() => localStorage.getItem("ppi.auth.token"));
    expect(token).toBeNull();
  });
});
