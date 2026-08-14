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
    micro: 553,
    id: "py-553-subsets",
    title: "DSA Subsets",
    solution: `def subsets(nums):
    res = [[]]
    for x in nums:
        res += [r + [x] for r in res]
    return res

print(subsets([1, 2, 3]))
`,
    nextUrl: /\/learn\/py-554-permute/,
    cursorAfter: "554",
  },
  {
    micro: 554,
    id: "py-554-permute",
    title: "DSA Permute",
    solution: `def permute(nums):
    res = []
    def bt(path, used):
        if len(path) == len(nums):
            res.append(path[:]); return
        for i, x in enumerate(nums):
            if used[i]:
                continue
            used[i] = True
            path.append(x)
            bt(path, used)
            path.pop()
            used[i] = False
    bt([], [False] * len(nums))
    return res

print(permute([1, 2, 3]))
`,
    nextUrl: /\/learn\/py-555-combination-sum/,
    cursorAfter: "555",
  },
  {
    micro: 555,
    id: "py-555-combination-sum",
    title: "DSA Comb Sum",
    solution: `def combination_sum(candidates, target):
    res = []
    def bt(start, remain, path):
        if remain == 0:
            res.append(path[:]); return
        for i in range(start, len(candidates)):
            x = candidates[i]
            if x > remain:
                continue
            path.append(x)
            bt(i, remain - x, path)
            path.pop()
    bt(0, target, [])
    return res

print(combination_sum([2, 3, 6, 7], 7))
`,
    nextUrl: /\/learn\/py-556-letter-combos/,
    cursorAfter: "556",
  },
  {
    micro: 556,
    id: "py-556-letter-combos",
    title: "DSA Letter Combos",
    solution: `def letter_combinations(digits):
    if not digits:
        return []
    m = {"2": "abc", "3": "def", "4": "ghi", "5": "jkl", "6": "mno", "7": "pqrs", "8": "tuv", "9": "wxyz"}
    res = [""]
    for d in digits:
        res = [p + c for p in res for c in m[d]]
    return res

print(letter_combinations("23"))
`,
    nextUrl: /\/learn\/py-557-generate-parens/,
    cursorAfter: "557",
  },
  {
    micro: 557,
    id: "py-557-generate-parens",
    title: "DSA Generate Parens",
    solution: `def generate_parenthesis(n):
    res = []
    def bt(s, op, cl):
        if len(s) == 2 * n:
            res.append(s); return
        if op < n:
            bt(s + "(", op + 1, cl)
        if cl < op:
            bt(s + ")", op, cl + 1)
    bt("", 0, 0)
    return res

print(generate_parenthesis(3))
`,
    nextUrl: /\/learn\/py-558-word-search/,
    cursorAfter: "558",
  },
  {
    micro: 558,
    id: "py-558-word-search",
    title: "DSA Word Search",
    solution: `def exist(board, word):
    rows, cols = len(board), len(board[0])
    def dfs(r, c, i):
        if i == len(word):
            return True
        if r < 0 or r >= rows or c < 0 or c >= cols or board[r][c] != word[i]:
            return False
        tmp, board[r][c] = board[r][c], "#"
        ok = dfs(r+1,c,i+1) or dfs(r-1,c,i+1) or dfs(r,c+1,i+1) or dfs(r,c-1,i+1)
        board[r][c] = tmp
        return ok
    return any(dfs(i, j, 0) for i in range(rows) for j in range(cols))

print(exist([["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], "ABCCED"))
`,
    nextUrl: /\/workspace/,
    cursorAfter: "559",
  }
];

test("declares the contiguous learn-route family", () => {
  for (const step of FAMILY) {
    expect(step.id).toMatch(/^py-(?:553|554|555|556|557|558)-/);
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

test.describe("micro-steps 553–558 · backtrack III", () => {
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
      if (nextMicro <= 594) {
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
