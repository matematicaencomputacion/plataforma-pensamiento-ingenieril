import { expect, type Page } from "@playwright/test";

/** Wait until Leptos Wasm shell can handle form submit (preventDefault). */
export async function gotoApp(page: Page, path: string) {
  await page.goto(path, { waitUntil: "load" });
  await expect(page.locator(".shell__main")).toBeVisible();
  await page.waitForFunction(() => {
    const w = window as unknown as { wasmBindings?: unknown };
    const hasUi = !!document.querySelector(
      "#login-email, #register-email, #forgot-email, #reset-password, .hero__title, .workspace__title",
    );
    return !!w.wasmBindings && hasUi;
  });
  await page.waitForTimeout(400);
}

/**
 * Fill a Leptos controlled input (`prop:value` + `on:input`) via the native
 * value setter so the signal updates and the DOM is not wiped on redraw.
 */
export async function fillLeptosInput(page: Page, selector: string, value: string) {
  await page.locator(selector).evaluate((el, v) => {
    const input = el as HTMLInputElement;
    const desc = Object.getOwnPropertyDescriptor(
      window.HTMLInputElement.prototype,
      "value",
    );
    desc?.set?.call(input, v);
    input.dispatchEvent(new Event("input", { bubbles: true }));
    input.dispatchEvent(new Event("change", { bubbles: true }));
  }, value);
  await expect(page.locator(selector)).toHaveValue(value);
}
