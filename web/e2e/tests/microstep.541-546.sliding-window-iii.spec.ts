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
    micro: 541,
    id: "py-541-max-vowels",
    title: "DSA Max Vowels",
    solution: `def max_vowels(s, k):
    vowels = set("aeiou")
    cur = sum(1 for c in s[:k] if c in vowels)
    best = cur
    for i in range(k, len(s)):
        cur += (s[i] in vowels) - (s[i - k] in vowels)
        best = max(best, cur)
    return best

print(max_vowels("abciiidef", 3))
`,
    nextUrl: /\/learn\/py-542-longest-ones/,
    cursorAfter: "542",
  },
  {
    micro: 542,
    id: "py-542-longest-ones",
    title: "DSA Longest Ones",
    solution: `def longest_ones(nums, k):
    i = zeros = best = 0
    for j, x in enumerate(nums):
        zeros += x == 0
        while zeros > k:
            zeros -= nums[i] == 0
            i += 1
        best = max(best, j - i + 1)
    return best

print(longest_ones([1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], 2))
`,
    nextUrl: /\/learn\/py-543-min-window/,
    cursorAfter: "543",
  },
  {
    micro: 543,
    id: "py-543-min-window",
    title: "DSA Min Window",
    solution: `def min_window(s, t):
    from collections import Counter
    need = Counter(t)
    missing = len(t)
    i = start = 0
    best = (float("inf"), 0, 0)
    for j, ch in enumerate(s, 1):
        if need[ch] > 0:
            missing -= 1
        need[ch] -= 1
        if missing == 0:
            while i < j and need[s[i]] < 0:
                need[s[i]] += 1
                i += 1
            if j - i < best[0]:
                best = (j - i, i, j)
            need[s[i]] += 1
            missing += 1
            i += 1
    return "" if best[0] == float("inf") else s[best[1]:best[2]]

print(min_window("ADOBECODEBANC", "ABC"))
`,
    nextUrl: /\/learn\/py-544-find-anagrams/,
    cursorAfter: "544",
  },
  {
    micro: 544,
    id: "py-544-find-anagrams",
    title: "DSA Find Anagrams",
    solution: `def find_anagrams(s, p):
    from collections import Counter
    need, cur = Counter(p), Counter()
    k, res = len(p), []
    for i, ch in enumerate(s):
        cur[ch] += 1
        if i >= k:
            old = s[i - k]
            cur[old] -= 1
            if cur[old] == 0:
                del cur[old]
        if i >= k - 1 and cur == need:
            res.append(i - k + 1)
    return res

print(find_anagrams("cbaebabacd", "abc"))
`,
    nextUrl: /\/learn\/py-545-max-sliding/,
    cursorAfter: "545",
  },
  {
    micro: 545,
    id: "py-545-max-sliding",
    title: "DSA Max Sliding",
    solution: `def max_sliding_window(nums, k):
    from collections import deque
    q, out = deque(), []
    for i, x in enumerate(nums):
        while q and nums[q[-1]] <= x:
            q.pop()
        q.append(i)
        if q[0] <= i - k:
            q.popleft()
        if i >= k - 1:
            out.append(nums[q[0]])
    return out

print(max_sliding_window([1, 3, -1, -3, 5, 3, 6, 7], 3))
`,
    nextUrl: /\/learn\/py-546-length-k-distinct/,
    cursorAfter: "546",
  },
  {
    micro: 546,
    id: "py-546-length-k-distinct",
    title: "DSA K Distinct",
    solution: `def length_of_longest_substring_k_distinct(s, k):
    from collections import defaultdict
    cnt = defaultdict(int)
    i = best = 0
    for j, ch in enumerate(s):
        cnt[ch] += 1
        while len(cnt) > k:
            cnt[s[i]] -= 1
            if cnt[s[i]] == 0:
                del cnt[s[i]]
            i += 1
        best = max(best, j - i + 1)
    return best

print(length_of_longest_substring_k_distinct("eceba", 2))
`,
    nextUrl: /\/workspace/,
    cursorAfter: "547",
  }
];

test("declares the contiguous learn-route family", () => {
  for (const step of FAMILY) {
    expect(step.id).toMatch(/^py-(?:541|542|543|544|545|546)-/);
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

test.describe("micro-steps 541–546 · sliding window III", () => {
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
      if (nextMicro <= 582) {
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
