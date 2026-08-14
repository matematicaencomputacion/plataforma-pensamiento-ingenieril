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
    micro: 649,
    id: "py-649-num-distinct",
    title: "DSA DP Strings IV · Distinct Subsequences",
    solution: "def num_distinct(s, t):\n    m = len(t)\n    dp = [0] * (m + 1)\n    dp[0] = 1\n    for ch in s:\n        for j in range(m, 0, -1):\n            if ch == t[j - 1]:\n                dp[j] += dp[j - 1]\n    return dp[m]\n\nprint(num_distinct('rabbbit', 'rabbit'))\n",
    nextUrl: /\/learn\/py-650-is-interleave/,
    cursorAfter: "650",
  },
  {
    micro: 650,
    id: "py-650-is-interleave",
    title: "DSA DP Strings IV · Interleaving",
    solution: "def is_interleave(s1, s2, s3):\n    n, m = len(s1), len(s2)\n    if n + m != len(s3):\n        return False\n    dp = [False] * (m + 1)\n    dp[0] = True\n    for j in range(1, m + 1):\n        dp[j] = dp[j - 1] and s2[j - 1] == s3[j - 1]\n    for i in range(1, n + 1):\n        dp[0] = dp[0] and s1[i - 1] == s3[i - 1]\n        for j in range(1, m + 1):\n            dp[j] = (dp[j] and s1[i - 1] == s3[i + j - 1]) or (dp[j - 1] and s2[j - 1] == s3[i + j - 1])\n    return dp[m]\n\nprint(is_interleave('aabcc', 'dbbca', 'aadbbcbcac'))\n",
    nextUrl: /\/learn\/py-651-min-cut-palindrome/,
    cursorAfter: "651",
  },
  {
    micro: 651,
    id: "py-651-min-cut-palindrome",
    title: "DSA DP Strings IV · Palindrome Cuts",
    solution: "def min_cut(s):\n    n = len(s)\n    pal = [[False] * n for _ in range(n)]\n    for i in range(n - 1, -1, -1):\n        for j in range(i, n):\n            pal[i][j] = s[i] == s[j] and (j - i < 2 or pal[i + 1][j - 1])\n    dp = [0] * n\n    for i in range(n):\n        if pal[0][i]:\n            dp[i] = 0\n        else:\n            dp[i] = min(dp[j] + 1 for j in range(i) if pal[j + 1][i])\n    return dp[-1]\n\nprint(min_cut('aab'))\n",
    nextUrl: /\/learn\/py-652-longest-common-subseq/,
    cursorAfter: "652",
  },
  {
    micro: 652,
    id: "py-652-longest-common-subseq",
    title: "DSA DP Strings IV · LCS Length",
    solution: "def longest_common_subsequence(text1, text2):\n    n, m = len(text1), len(text2)\n    prev = [0] * (m + 1)\n    for i in range(1, n + 1):\n        cur = [0] * (m + 1)\n        for j in range(1, m + 1):\n            if text1[i - 1] == text2[j - 1]:\n                cur[j] = prev[j - 1] + 1\n            else:\n                cur[j] = max(prev[j], cur[j - 1])\n        prev = cur\n    return prev[m]\n\nprint(longest_common_subsequence('abcde', 'ace'))\n",
    nextUrl: /\/learn\/py-653-wildcard-match/,
    cursorAfter: "653",
  },
  {
    micro: 653,
    id: "py-653-wildcard-match",
    title: "DSA DP Strings IV · Wildcard Match",
    solution: "def is_match(s, p):\n    n, m = len(s), len(p)\n    dp = [False] * (m + 1)\n    dp[0] = True\n    for j in range(1, m + 1):\n        dp[j] = dp[j - 1] and p[j - 1] == '*'\n    for i in range(1, n + 1):\n        prev = dp[0]\n        dp[0] = False\n        for j in range(1, m + 1):\n            temp = dp[j]\n            if p[j - 1] == '*':\n                dp[j] = dp[j] or dp[j - 1]\n            elif p[j - 1] == '?' or p[j - 1] == s[i - 1]:\n                dp[j] = prev\n            else:\n                dp[j] = False\n            prev = temp\n    return dp[m]\n\nprint(is_match('adceb', '*a*b'))\n",
    nextUrl: /\/learn\/py-654-longest-palindrome-subseq/,
    cursorAfter: "654",
  },
  {
    micro: 654,
    id: "py-654-longest-palindrome-subseq",
    title: "DSA DP Strings IV · Palindromic Subseq",
    solution: "def longest_palindrome_subseq(s):\n    n = len(s)\n    dp = [0] * n\n    for i in range(n - 1, -1, -1):\n        dp[i] = 1\n        prev = 0\n        for j in range(i + 1, n):\n            temp = dp[j]\n            if s[i] == s[j]:\n                dp[j] = prev + 2\n            else:\n                dp[j] = max(dp[j], dp[j - 1])\n            prev = temp\n    return dp[-1]\n\nprint(longest_palindrome_subseq('bbbab'))\n",
    nextUrl: /\/workspace/,
    cursorAfter: "655",
  },
];

test("declares the contiguous learn-route family", () => {
  for (const step of FAMILY) {
    expect(step.id).toMatch(/^py-(?:649|650|651|652|653|654)-/);
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

test.describe("micro-steps 649–654 · dp strings iv", () => {
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
      if (nextMicro <= 654) {
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

      if (step.micro < 654) {
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
