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
    micro: 227,
    id: "py-227-generate-parens",
    title: "DSA Generar Paréntesis",
    solution: `def generate_parenthesis(n):
    out = []
    def bt(s, open_n, close_n):
        if len(s) == 2 * n:
            out.append(s)
            return
        if open_n < n:
            bt(s + '(', open_n + 1, close_n)
        if close_n < open_n:
            bt(s + ')', open_n, close_n + 1)
    bt('', 0, 0)
    return out
print(generate_parenthesis(3))
`,
    nextUrl: /\/learn\/py-228-combination-sum/,
    cursorAfter: "228",
  },
  {
    micro: 228,
    id: "py-228-combination-sum",
    title: "DSA Combination Sum",
    solution: `def combination_sum(candidates, target):
    candidates = sorted(candidates)
    out = []
    def bt(start, remain, path):
        if remain == 0:
            out.append(path[:])
            return
        for i in range(start, len(candidates)):
            value = candidates[i]
            if value > remain:
                break
            path.append(value)
            bt(i, remain - value, path)
            path.pop()
    bt(0, target, [])
    return out
print(combination_sum([2, 3, 6, 7], 7))
`,
    nextUrl: /\/learn\/py-229-word-search/,
    cursorAfter: "229",
  },
  {
    micro: 229,
    id: "py-229-word-search",
    title: "DSA Word Search",
    solution: `def exist(board, word):
    rows, cols = len(board), len(board[0])
    def dfs(i, j, k):
        if k == len(word):
            return True
        if i < 0 or j < 0 or i >= rows or j >= cols or board[i][j] != word[k]:
            return False
        tmp = board[i][j]
        board[i][j] = '#'
        ok = (
            dfs(i + 1, j, k + 1)
            or dfs(i - 1, j, k + 1)
            or dfs(i, j + 1, k + 1)
            or dfs(i, j - 1, k + 1)
        )
        board[i][j] = tmp
        return ok
    return any(dfs(i, j, 0) for i in range(rows) for j in range(cols))
print(exist([['A','B','C','E'],['S','F','C','S'],['A','D','E','E']], 'ABCCED'))
`,
    nextUrl: /\/learn\/py-230-letter-combos/,
    cursorAfter: "230",
  },
  {
    micro: 230,
    id: "py-230-letter-combos",
    title: "DSA Letter Combinations",
    solution: `def letter_combinations(digits):
    if not digits:
        return []
    phone = {
        '2': 'abc', '3': 'def', '4': 'ghi', '5': 'jkl',
        '6': 'mno', '7': 'pqrs', '8': 'tuv', '9': 'wxyz',
    }
    out = ['']
    for digit in digits:
        out = [prefix + ch for prefix in out for ch in phone[digit]]
    return out
print(letter_combinations('23'))
`,
    nextUrl: /\/learn\/py-231-subsets-ii/,
    cursorAfter: "231",
  },
  {
    micro: 231,
    id: "py-231-subsets-ii",
    title: "DSA Subsets II",
    solution: `def subsets_with_dup(nums):
    nums = sorted(nums)
    out = []
    def bt(start, path):
        out.append(path[:])
        for i in range(start, len(nums)):
            if i > start and nums[i] == nums[i - 1]:
                continue
            path.append(nums[i])
            bt(i + 1, path)
            path.pop()
    bt(0, [])
    return out
print(subsets_with_dup([1, 2, 2]))
`,
    nextUrl: /\/learn\/py-232-palindrome-partition/,
    cursorAfter: "232",
  },
  {
    micro: 232,
    id: "py-232-palindrome-partition",
    title: "DSA Palindrome Partition",
    solution: `def partition(s):
    out = []
    def is_pal(left, right):
        while left < right:
            if s[left] != s[right]:
                return False
            left += 1
            right -= 1
        return True
    def bt(start, path):
        if start == len(s):
            out.append(path[:])
            return
        for end in range(start, len(s)):
            if is_pal(start, end):
                path.append(s[start:end + 1])
                bt(end + 1, path)
                path.pop()
    bt(0, [])
    return out
print(partition('aab'))
`,
    nextUrl: /\/learn\/py-233-reverse-integer/,
    cursorAfter: "233",
  },
];

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

test.describe("micro-steps 227–232 · backtracking", () => {
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
      if (nextMicro <= 600) {
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
