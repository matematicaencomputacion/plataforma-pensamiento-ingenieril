import { expect, test } from "@playwright/test";
import { fillLeptosInput, gotoApp } from "./helpers";

/**
 * Password recovery smoke:
 * - Login exposes the forgot link
 * - Forgot token via API (DX exposure)
 * - Reset form UI updates password and session
 */
test.describe("password reset smoke", () => {
  test("forgot link exists and reset form establishes session", async ({
    page,
    request,
  }) => {
    const email = `reset-e2e-${Date.now()}@example.com`;
    const oldPassword = "secreto12";
    const newPassword = "nuevaClave9";

    const reg = await request.post("/api/auth/register", {
      data: { email, password: oldPassword },
    });
    expect(reg.ok(), `register status ${reg.status()} ${await reg.text()}`).toBeTruthy();

    await gotoApp(page, "/login");
    await expect(page.getByRole("link", { name: "Olvidé mi contraseña" })).toBeVisible();
    await page.getByRole("link", { name: "Olvidé mi contraseña" }).click();
    await expect(page).toHaveURL(/\/forgot-password/);
    await expect(page.getByRole("heading", { name: "Recuperar contraseña" })).toBeVisible();

    const forgot = await request.post("/api/auth/forgot-password", {
      data: { email },
    });
    expect(forgot.ok()).toBeTruthy();
    const forgotBody = (await forgot.json()) as {
      message?: string;
      resetToken?: string;
    };
    expect(forgotBody.resetToken, JSON.stringify(forgotBody)).toBeTruthy();

    await gotoApp(page, `/reset-password?token=${forgotBody.resetToken}`);
    await expect(page.getByRole("heading", { name: "Nueva contraseña" })).toBeVisible();
    await expect(page.locator("#reset-token")).toHaveValue(forgotBody.resetToken!);

    await fillLeptosInput(page, "#reset-password", newPassword);
    await page.getByRole("button", { name: "Restablecer contraseña" }).click();

    await Promise.race([
      page.waitForURL(/\/workspace/, { timeout: 20_000 }),
      page
        .getByRole("alert")
        .waitFor({ state: "visible", timeout: 20_000 })
        .then(async () => {
          const msg =
            (await page.getByRole("alert").textContent())?.trim() ?? "reset failed";
          throw new Error(`Reset did not reach /workspace: ${msg}`);
        }),
    ]);

    await expect(page.getByRole("heading", { name: "Workspace" })).toBeVisible();
    await expect(page.locator(".session-bar__email")).toContainText(email);

    const loginOld = await request.post("/api/auth/login", {
      data: { email, password: oldPassword },
    });
    expect(loginOld.status()).toBe(401);

    const loginNew = await request.post("/api/auth/login", {
      data: { email, password: newPassword },
    });
    expect(loginNew.ok()).toBeTruthy();
  });
});
