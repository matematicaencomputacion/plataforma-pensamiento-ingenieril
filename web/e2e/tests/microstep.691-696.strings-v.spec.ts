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
    micro: 691,
    id: "py-691-prefix-function",
    title: "DSA Strings V · Prefix Function",
    solution: "def prefix_function(s):\n    pi = [0] * len(s)\n    j = 0\n    for i in range(1, len(s)):\n        while j and s[i] != s[j]:\n            j = pi[j - 1]\n        if s[i] == s[j]:\n            j += 1\n        pi[i] = j\n    return pi\n\nprint(prefix_function(\"ababaca\"))\n",
    nextUrl: /\/learn\/py-692-kmp-search/,
    cursorAfter: "692",
  },
  {
    micro: 692,
    id: "py-692-kmp-search",
    title: "DSA Strings V · KMP Search",
    solution: "def prefix_function(s):\n    pi = [0] * len(s)\n    j = 0\n    for i in range(1, len(s)):\n        while j and s[i] != s[j]:\n            j = pi[j - 1]\n        if s[i] == s[j]:\n            j += 1\n        pi[i] = j\n    return pi\n\ndef kmp_search(haystack, needle):\n    if not needle:\n        return 0\n    pi = prefix_function(needle)\n    j = 0\n    for i, ch in enumerate(haystack):\n        while j and ch != needle[j]:\n            j = pi[j - 1]\n        if ch == needle[j]:\n            j += 1\n            if j == len(needle):\n                return i - j + 1\n    return -1\n\nprint(kmp_search(\"sadbutsad\", \"sad\"))\n",
    nextUrl: /\/learn\/py-693-z-function/,
    cursorAfter: "693",
  },
  {
    micro: 693,
    id: "py-693-z-function",
    title: "DSA Strings V · Z Function",
    solution: "def z_function(s):\n    n = len(s)\n    z = [0] * n\n    l = r = 0\n    for i in range(1, n):\n        if i <= r:\n            z[i] = min(r - i + 1, z[i - l])\n        while i + z[i] < n and s[z[i]] == s[i + z[i]]:\n            z[i] += 1\n        if i + z[i] - 1 > r:\n            l, r = i, i + z[i] - 1\n    return z\n\nprint(z_function(\"aabcaabxaaaz\"))\n",
    nextUrl: /\/learn\/py-694-rabin-karp/,
    cursorAfter: "694",
  },
  {
    micro: 694,
    id: "py-694-rabin-karp",
    title: "DSA Strings V · Rabin-Karp",
    solution: "def rabin_karp(haystack, needle):\n    n, m = len(haystack), len(needle)\n    if m == 0:\n        return 0\n    if m > n:\n        return -1\n    base, mod = 256, 10 ** 9 + 7\n    h = pow(base, m - 1, mod)\n    th = ph = 0\n    for i in range(m):\n        th = (th * base + ord(haystack[i])) % mod\n        ph = (ph * base + ord(needle[i])) % mod\n    for i in range(n - m + 1):\n        if th == ph and haystack[i:i + m] == needle:\n            return i\n        if i + m < n:\n            th = ((th - ord(haystack[i]) * h) * base + ord(haystack[i + m])) % mod\n    return -1\n\nprint(rabin_karp(\"hello\", \"ll\"))\n",
    nextUrl: /\/learn\/py-695-repeated-substring/,
    cursorAfter: "695",
  },
  {
    micro: 695,
    id: "py-695-repeated-substring",
    title: "DSA Strings V · Repeated Pattern",
    solution: "def repeated_substring_pattern(s):\n    return s in (s + s)[1:-1]\n\nprint(repeated_substring_pattern(\"abab\"))\n",
    nextUrl: /\/learn\/py-696-shortest-palindrome/,
    cursorAfter: "696",
  },
  {
    micro: 696,
    id: "py-696-shortest-palindrome",
    title: "DSA Strings V · Shortest Palindrome",
    solution: "def prefix_function(s):\n    pi = [0] * len(s)\n    j = 0\n    for i in range(1, len(s)):\n        while j and s[i] != s[j]:\n            j = pi[j - 1]\n        if s[i] == s[j]:\n            j += 1\n        pi[i] = j\n    return pi\n\ndef shortest_palindrome(s):\n    rev = s[::-1]\n    pi = prefix_function(s + \"#\" + rev)\n    return rev[: len(s) - pi[-1]] + s\n\nprint(shortest_palindrome(\"aacecaaa\"))\n",
    nextUrl: /\/learn\/py-697-fenwick-prefix/,
    cursorAfter: "697",
  },
];

test("declares the contiguous learn-route family", () => {
  for (const step of FAMILY) {
    expect(step.id).toMatch(/^py-(?:691|692|693|694|695|696)-/);
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

test.describe("micro-steps 691–696 · strings v", () => {
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
      if (nextMicro <= 696) {
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

      if (step.micro < 696) {
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
