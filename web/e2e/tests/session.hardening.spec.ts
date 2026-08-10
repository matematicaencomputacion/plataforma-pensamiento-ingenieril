import { expect, test } from "@playwright/test";
import {
  e2eTimeout,
  fillLeptosInput,
  gotoApp,
  waitForAuthFormReady,
} from "./helpers";

/**
 * Rebanada 1 — email trim + recovery guards when already authenticated.
 */
test.describe("session hardening (trim + recovery guards)", () => {
  test("login accepts email with surrounding whitespace", async ({ page, request }) => {
    const stamp = `${Date.now()}-${Math.floor(Math.random() * 1e6)}`;
    const email = `trim-${stamp}@example.com`;
    const password = "secreto12trim";

    const reg = await request.post("/api/auth/register", {
      data: { email, password },
      timeout: e2eTimeout,
    });
    expect(reg.ok(), await reg.text()).toBeTruthy();

    await gotoApp(page, "/login");
    await waitForAuthFormReady(page, {
      emailSelector: "#login-email",
      passwordSelector: "#login-password",
      submitName: "Entrar",
    });
    // type=email may strip spaces in the DOM; still feed padded/cased input via setter
    // and assert the *request body* is sanitized (trim + lowercase).
    await page.locator("#login-email").evaluate((el, v) => {
      const input = el as HTMLInputElement;
      const desc = Object.getOwnPropertyDescriptor(
        window.HTMLInputElement.prototype,
        "value",
      );
      desc?.set?.call(input, v);
      input.dispatchEvent(new Event("input", { bubbles: true }));
      input.dispatchEvent(new Event("change", { bubbles: true }));
    }, `  ${email.toUpperCase()}  `);
    await fillLeptosInput(page, "#login-password", password);

    const loginResponse = page.waitForResponse(
      (res) =>
        res.url().includes("/api/auth/login") && res.request().method() === "POST",
      { timeout: e2eTimeout },
    );
    await page.getByRole("button", { name: "Entrar" }).click();
    const loginRes = await loginResponse;
    expect(loginRes.ok(), await loginRes.text()).toBeTruthy();
    const posted = loginRes.request().postDataJSON() as { email?: string };
    expect(posted.email).toBe(email);

    await expect(page).toHaveURL(/\/workspace/, { timeout: e2eTimeout });
    await expect(page.locator(".session-bar__email")).toContainText(email);
  });

  test("authenticated visits to forgot/reset redirect to workspace", async ({
    page,
    request,
  }) => {
    const email = `guard-${Date.now()}@example.com`;
    const password = "secreto12guard";

    const reg = await request.post("/api/auth/register", {
      data: { email, password },
      timeout: e2eTimeout,
    });
    expect(reg.ok(), await reg.text()).toBeTruthy();
    const body = (await reg.json()) as { token: string };

    await page.goto("/");
    await page.evaluate((token) => {
      localStorage.setItem("ppi.auth.token", token);
    }, body.token);

    await gotoApp(page, "/forgot-password");
    await expect(page).toHaveURL(/\/workspace/, { timeout: e2eTimeout });
    await expect(page.getByRole("heading", { name: "Workspace" })).toBeVisible({
      timeout: e2eTimeout,
    });

    await gotoApp(page, "/reset-password?token=deadbeef");
    await expect(page).toHaveURL(/\/workspace/, { timeout: e2eTimeout });
    await expect(page.locator(".session-bar__email")).toContainText(email);
  });
});
