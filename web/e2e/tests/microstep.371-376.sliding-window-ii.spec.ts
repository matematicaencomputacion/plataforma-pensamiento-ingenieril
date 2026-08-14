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
    micro: 371,
    id: "py-371-longest-substr",
    title: "DSA Longest Substr",
    solution: `def length_of_longest_substring(s):
    seen = set()
    l = 0
    best = 0
    for r, ch in enumerate(s):
        while ch in seen:
            seen.remove(s[l])
            l += 1
        seen.add(ch)
        best = max(best, r - l + 1)
    return best

print(length_of_longest_substring("abcabcbb"))
`,
    nextUrl: /\/learn\/py-372-min-window/,
    cursorAfter: "372",
  },
  {
    micro: 372,
    id: "py-372-min-window",
    title: "DSA Min Window",
    solution: `from collections import Counter

def min_window(s, t):
    need = Counter(t)
    missing = len(t)
    l = 0
    best = (0, float("inf"))
    for r, ch in enumerate(s):
        if need[ch] > 0:
            missing -= 1
        need[ch] -= 1
        while missing == 0:
            if r - l + 1 < best[1] - best[0]:
                best = (l, r + 1)
            need[s[l]] += 1
            if need[s[l]] > 0:
                missing += 1
            l += 1
    return s[best[0]:best[1]] if best[1] != float("inf") else ""

print(min_window("ADOBECODEBANC", "ABC"))
`,
    nextUrl: /\/learn\/py-373-max-avg-sub/,
    cursorAfter: "373",
  },
  {
    micro: 373,
    id: "py-373-max-avg-sub",
    title: "DSA Max Avg Sub",
    solution: `def find_max_average(nums, k):
    s = sum(nums[:k])
    best = s
    for i in range(k, len(nums)):
        s += nums[i] - nums[i - k]
        best = max(best, s)
    return best / k

print(find_max_average([1, 12, -5, -6, 50, 3], 4))
`,
    nextUrl: /\/learn\/py-374-fruits-baskets/,
    cursorAfter: "374",
  },
  {
    micro: 374,
    id: "py-374-fruits-baskets",
    title: "DSA Fruits Baskets",
    solution: `from collections import defaultdict

def total_fruit(fruits):
    cnt = defaultdict(int)
    l = 0
    best = 0
    for r, x in enumerate(fruits):
        cnt[x] += 1
        while len(cnt) > 2:
            cnt[fruits[l]] -= 1
            if cnt[fruits[l]] == 0:
                del cnt[fruits[l]]
            l += 1
        best = max(best, r - l + 1)
    return best

print(total_fruit([1, 2, 1]))
`,
    nextUrl: /\/learn\/py-375-longest-ones/,
    cursorAfter: "375",
  },
  {
    micro: 375,
    id: "py-375-longest-ones",
    title: "DSA Longest Ones",
    solution: `def longest_ones(nums, k):
    l = zeros = best = 0
    for r, x in enumerate(nums):
        zeros += x == 0
        while zeros > k:
            zeros -= nums[l] == 0
            l += 1
        best = max(best, r - l + 1)
    return best

print(longest_ones([1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], 2))
`,
    nextUrl: /\/learn\/py-376-find-anagrams/,
    cursorAfter: "376",
  },
  {
    micro: 376,
    id: "py-376-find-anagrams",
    title: "DSA Find Anagrams",
    solution: `from collections import Counter

def find_anagrams(s, p):
    need = Counter(p)
    n = len(p)
    win = Counter(s[:n])
    out = []
    if win == need:
        out.append(0)
    for i in range(n, len(s)):
        win[s[i]] += 1
        win[s[i - n]] -= 1
        if win[s[i - n]] == 0:
            del win[s[i - n]]
        if win == need:
            out.append(i - n + 1)
    return out

print(find_anagrams("cbaebabacd", "abc"))
`,
    nextUrl: /\/learn\/py-377-single-number/,
    cursorAfter: "377",
  }
];

test("declares the contiguous learn-route family", () => {
  for (const step of FAMILY) {
    expect(step.id).toMatch(/^py-(?:371|372|373|374|375|376)-/);
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

test.describe("micro-steps 371–376 · sliding window II", () => {
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
      if (nextMicro <= 480) {
        await expect(
          page.locator(
            `#workspace-microsteps [data-microstep="${nextMicro}"]`,
          ),
        ).toHaveClass(/workspace__microstep--jumpable/);
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

      if (!step.nextUrl.source.includes("workspace")) {
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
