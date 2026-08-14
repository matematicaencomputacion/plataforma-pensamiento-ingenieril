import { expect, test, type APIRequestContext, type Page } from "@playwright/test";
import {
  e2eTimeout,
  fillLeptosInput,
  fillLeptosTextarea,
  gotoApp,
  installPyodideMock,
  waitForAuthFormReady,
} from "./helpers";
import { unlockThroughMicroStep } from "./microstepProgress";

const useRealPyodide = process.env.PPI_E2E_REAL_PYODIDE === "1";

type FamilyStep = {
  micro: number;
  id: string;
  title: string;
  solution: string;
  nextUrl: RegExp;
  cursorAfter: string;
};

const FAMILY: FamilyStep[] = [
  {
    micro: 715,
    id: "py-715-sieve-primes",
    title: "DSA NT · Sieve",
    solution: "def sieve(n):\n    is_p = [False, False] + [True] * (n - 1)\n    primes = []\n    for i in range(2, n + 1):\n        if is_p[i]:\n            primes.append(i)\n            for j in range(i * i, n + 1, i):\n                is_p[j] = False\n    return primes\n\nprint(sieve(20))\n",
    nextUrl: /\/learn\/py-716-euler-phi/,
    cursorAfter: "716",
  },
  {
    micro: 716,
    id: "py-716-euler-phi",
    title: "DSA NT · Euler Phi",
    solution: "def euler_phi(n):\n    r, x, i = n, n, 2\n    while i * i <= x:\n        if x % i == 0:\n            while x % i == 0:\n                x //= i\n            r -= r // i\n        i += 1\n    if x > 1:\n        r -= r // x\n    return r\n\nprint(euler_phi(12))\n",
    nextUrl: /\/learn\/py-717-linear-sieve/,
    cursorAfter: "717",
  },
  {
    micro: 717,
    id: "py-717-linear-sieve",
    title: "DSA NT · Linear Sieve",
    solution: "def linear_sieve(n):\n    spf = list(range(n + 1))\n    primes = []\n    for i in range(2, n + 1):\n        if spf[i] == i:\n            primes.append(i)\n        for p in primes:\n            if p > spf[i] or i * p > n:\n                break\n            spf[i * p] = p\n    return primes, spf\n\nprimes, spf = linear_sieve(20)\nprint([primes, spf[18], spf[19]])\n",
    nextUrl: /\/learn\/py-718-binpow/,
    cursorAfter: "718",
  },
  {
    micro: 718,
    id: "py-718-binpow",
    title: "DSA NT · Binpow",
    solution: "def binpow(a, e, mod):\n    r = 1\n    a %= mod\n    while e:\n        if e & 1:\n            r = r * a % mod\n        a = a * a % mod\n        e >>= 1\n    return r\n\nprint(binpow(2, 10, 1000))\n",
    nextUrl: /\/learn\/py-719-mod-inverse/,
    cursorAfter: "719",
  },
  {
    micro: 719,
    id: "py-719-mod-inverse",
    title: "DSA NT · Mod Inverse",
    solution: "def binpow(a, e, mod):\n    r = 1\n    a %= mod\n    while e:\n        if e & 1:\n            r = r * a % mod\n        a = a * a % mod\n        e >>= 1\n    return r\n\ndef mod_inverse(a, mod):\n    return binpow(a, mod - 2, mod)\n\nprint(mod_inverse(3, 11))\n",
    nextUrl: /\/learn\/py-720-crt/,
    cursorAfter: "720",
  },
  {
    micro: 720,
    id: "py-720-crt",
    title: "DSA NT · CRT",
    solution: "def binpow(a, e, mod):\n    r = 1\n    a %= mod\n    while e:\n        if e & 1:\n            r = r * a % mod\n        a = a * a % mod\n        e >>= 1\n    return r\n\ndef mod_inverse(a, mod):\n    return binpow(a, mod - 2, mod)\n\ndef crt(a1, m1, a2, m2):\n    k = (a2 - a1) * mod_inverse(m1 % m2, m2) % m2\n    return (a1 + k * m1) % (m1 * m2)\n\nprint(crt(2, 3, 3, 5))\n",
    nextUrl: /\/workspace/,
    cursorAfter: "721",
  },
];

test("declares the contiguous learn-route family", () => {
  for (const step of FAMILY) {
    expect(step.id).toMatch(/^py-(?:715|716|717|718|719|720)-/);
    expect(step.nextUrl).toBeInstanceOf(RegExp);
  }
});

function uniqueCreds(micro: number) {
  const password = process.env.PPI_E2E_PASSWORD?.trim() || "secreto12ci";
  const stamp = `${Date.now()}-${Math.floor(Math.random() * 1e6)}`;
  return { email: `e2e-ms${micro}-${stamp}@example.com`, password };
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

test.describe("micro-steps 715–720 · number theory", () => {
  test.beforeEach(async ({ page }) => {
    if (!useRealPyodide) {
      await installPyodideMock(page);
    }
  });

  for (const step of FAMILY) {
    test(`rail opens ${step.id}; pass advances chain`, async ({
      page,
      request,
    }: {
      page: Page;
      request: APIRequestContext;
    }) => {
      const { email, password } = uniqueCreds(step.micro);
      const reg = await request.post("/api/auth/register", {
        data: { email, password },
        timeout: e2eTimeout,
      });
      expect(reg.ok(), await reg.text()).toBeTruthy();
      const regJson = (await reg.json()) as { token: string };

      await login(page, email, password);
      await unlockThroughMicroStep(request, regJson.token, step.micro - 1);
      await page.reload();
      await expect(page).toHaveURL(/\/workspace/, { timeout: e2eTimeout });
      await expect(page.locator("#workspace-microsteps")).toHaveAttribute(
        "data-current-level",
        String(step.micro),
        { timeout: e2eTimeout },
      );

      await expect(
        page.locator(`#workspace-microstep-link-${step.micro}`),
      ).toBeVisible();
      const nextMicro = step.micro + 1;
      if (nextMicro <= 720) {
        await expect(
          page.locator(
            `#workspace-microsteps [data-microstep="${nextMicro}"]`,
          ),
        ).toHaveClass(/workspace__microstep--open|workspace__microstep--jumpable/);
      } else {
        await expect(
          page.locator(`#workspace-microstep-link-${nextMicro}`),
        ).toHaveCount(0);
      }

      await page.locator(`#workspace-microstep-link-${step.micro}`).click();
      await expect(page).toHaveURL(new RegExp(`/learn/${step.id}`), {
        timeout: e2eTimeout,
      });
      await expect(
        page.getByRole("heading", { name: step.title }),
      ).toBeVisible({ timeout: e2eTimeout });

      const engineTimeout = useRealPyodide ? 120_000 : e2eTimeout;
      await expect(page.locator("#learn-engine-status")).toHaveAttribute(
        "data-status",
        "ready",
        { timeout: engineTimeout },
      );

      await fillLeptosTextarea(page, "#learn-editor", step.solution);
      await page.getByRole("button", { name: "Validar solución" }).click();
      await expect(page.locator("#learn-progress-check")).toBeVisible({
        timeout: engineTimeout,
      });

      await page.locator("#learn-continue").click();
      await expect(page).toHaveURL(step.nextUrl, { timeout: e2eTimeout });

      if (step.micro < 720) {
        await page
          .getByLabel("Navegación del Paso 2")
          .getByRole("link", { name: "Workspace" })
          .click();
        await expect(page).toHaveURL(/\/workspace/, { timeout: e2eTimeout });
      }

      await expect(page.locator("#workspace-microsteps")).toHaveAttribute(
        "data-current-level",
        step.cursorAfter,
      );
      const cell = page.locator(
        `#workspace-microsteps [data-microstep="${step.micro}"]`,
      );
      await expect(cell).toHaveClass(/workspace__microstep--done/);
      await expect(cell.locator(".workspace__microstep-badge")).toBeVisible();
    });
  }
});
