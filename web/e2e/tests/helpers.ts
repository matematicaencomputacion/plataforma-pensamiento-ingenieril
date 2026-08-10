import { expect, type Locator, type Page } from "@playwright/test";

/** CI runners are slower at cold Wasm hydrate / first API round-trip. */
export const e2eTimeout = process.env.CI ? 45_000 : 20_000;

/** Wait until Leptos Wasm shell can handle form submit (preventDefault). */
export async function gotoApp(page: Page, path: string) {
  await page.goto(path, { waitUntil: "domcontentloaded" });
  await page.waitForLoadState("load");
  await expect(page.locator(".shell__main")).toBeVisible({ timeout: e2eTimeout });
  await page.waitForFunction(
    () => {
      const w = window as unknown as { wasmBindings?: unknown };
      const hasUi = !!document.querySelector(
        "#login-email, #register-email, #forgot-email, #reset-password, .hero__title, .workspace__title, .onboarding__title, .learn__title",
      );
      return !!w.wasmBindings && hasUi;
    },
    null,
    { timeout: e2eTimeout },
  );
  // Brief settle so CSR listeners attach before the first click/submit.
  await page.waitForTimeout(process.env.CI ? 800 : 400);
}

/**
 * Assert auth form controls are interactive (visible + enabled) after hydrate.
 */
export async function waitForAuthFormReady(
  page: Page,
  opts: { emailSelector: string; passwordSelector: string; submitName: string | RegExp },
) {
  const email = page.locator(opts.emailSelector);
  const password = page.locator(opts.passwordSelector);
  const submit = page.getByRole("button", { name: opts.submitName });

  await expect(email).toBeVisible({ timeout: e2eTimeout });
  await expect(password).toBeVisible({ timeout: e2eTimeout });
  await expect(submit).toBeVisible({ timeout: e2eTimeout });
  await expect(email).toBeEnabled({ timeout: e2eTimeout });
  await expect(password).toBeEnabled({ timeout: e2eTimeout });
  await expect(submit).toBeEnabled({ timeout: e2eTimeout });
}

/**
 * Fill a Leptos controlled input (`prop:value` + `on:input`) via the native
 * value setter so the signal updates and the DOM is not wiped on redraw.
 */
export async function fillLeptosInput(page: Page, selector: string, value: string) {
  const input: Locator = page.locator(selector);
  await expect(input).toBeVisible({ timeout: e2eTimeout });
  await expect(input).toBeEnabled({ timeout: e2eTimeout });
  await input.evaluate((el, v) => {
    const node = el as HTMLInputElement;
    const desc = Object.getOwnPropertyDescriptor(
      window.HTMLInputElement.prototype,
      "value",
    );
    desc?.set?.call(node, v);
    node.dispatchEvent(new Event("input", { bubbles: true }));
    node.dispatchEvent(new Event("change", { bubbles: true }));
  }, value);
  await expect(input).toHaveValue(value, { timeout: e2eTimeout });
  // Leptos redraw can wipe unbound values; confirm the signal stuck.
  await page.waitForTimeout(process.env.CI ? 200 : 50);
  await expect(input).toHaveValue(value, { timeout: e2eTimeout });
}

/**
 * Same as {@link fillLeptosInput} for controlled `<textarea>` bindings.
 */
export async function fillLeptosTextarea(
  page: Page,
  selector: string,
  value: string,
) {
  const area: Locator = page.locator(selector);
  await expect(area).toBeVisible({ timeout: e2eTimeout });
  await expect(area).toBeEnabled({ timeout: e2eTimeout });
  await area.evaluate((el, v) => {
    const node = el as HTMLTextAreaElement;
    const desc = Object.getOwnPropertyDescriptor(
      window.HTMLTextAreaElement.prototype,
      "value",
    );
    desc?.set?.call(node, v);
    node.dispatchEvent(new Event("input", { bubbles: true }));
    node.dispatchEvent(new Event("change", { bubbles: true }));
  }, value);
  await expect(area).toHaveValue(value, { timeout: e2eTimeout });
  await page.waitForTimeout(process.env.CI ? 200 : 50);
  await expect(area).toHaveValue(value, { timeout: e2eTimeout });
}
