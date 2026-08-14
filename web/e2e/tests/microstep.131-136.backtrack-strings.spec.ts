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
    micro: 131,
    id: "py-131-two-pointers",
    title: "DSA Two Pointers",
    solution: `def two_sum(nums, target):
    left, right = 0, len(nums) - 1
    while left < right:
        s = nums[left] + nums[right]
        if s == target:
            return (left, right)
        if s < target:
            left += 1
        else:
            right -= 1
    return None
print(two_sum([2, 7, 11, 15], 9))
`,
    nextUrl: /\/learn\/py-132-sliding-window/,
    cursorAfter: "132",
  },
  {
    micro: 132,
    id: "py-132-sliding-window",
    title: "DSA Sliding Window",
    solution: `def max_window(nums, k):
    window = sum(nums[:k])
    best = window
    for i in range(k, len(nums)):
        window += nums[i] - nums[i - k]
        best = max(best, window)
    return best
print(max_window([2, 1, 5, 1, 3, 2], 3))
`,
    nextUrl: /\/learn\/py-133-permutations/,
    cursorAfter: "133",
  },
  {
    micro: 133,
    id: "py-133-permutations",
    title: "DSA Permutations",
    solution: `def permute(nums):
    res = []
    def bt(path, unused):
        if not unused:
            res.append(path[:])
            return
        for i, x in enumerate(unused):
            path.append(x)
            bt(path, unused[:i] + unused[i + 1:])
            path.pop()
    bt([], list(nums))
    return res
print(sorted(permute([1, 2, 3])))
`,
    nextUrl: /\/learn\/py-134-nqueens-count/,
    cursorAfter: "134",
  },
  {
    micro: 134,
    id: "py-134-nqueens-count",
    title: "DSA N-Queens Count",
    solution: `def nqueens_count(n):
    cols = set()
    diag1 = set()
    diag2 = set()
    def bt(row):
        if row == n:
            return 1
        total = 0
        for c in range(n):
            if c in cols or row - c in diag1 or row + c in diag2:
                continue
            cols.add(c); diag1.add(row - c); diag2.add(row + c)
            total += bt(row + 1)
            cols.remove(c); diag1.remove(row - c); diag2.remove(row + c)
        return total
    return bt(0)
print(nqueens_count(4))
`,
    nextUrl: /\/learn\/py-135-trie/,
    cursorAfter: "135",
  },
  {
    micro: 135,
    id: "py-135-trie",
    title: "DSA Trie",
    solution: `class Trie:
    def __init__(self):
        self.root = {}
    def insert(self, word):
        node = self.root
        for ch in word:
            node = node.setdefault(ch, {})
        node['#'] = True
    def search(self, word):
        node = self.root
        for ch in word:
            if ch not in node:
                return False
            node = node[ch]
        return '#' in node
t = Trie()
t.insert('cat')
t.insert('car')
print(t.search('cat'))
print(t.search('car'))
print(t.search('cap'))
`,
    nextUrl: /\/learn\/py-136-bit-count/,
    cursorAfter: "136",
  },
  {
    micro: 136,
    id: "py-136-bit-count",
    title: "DSA Bit Count",
    solution: `def bit_count(n):
    count = 0
    while n:
        n &= n - 1
        count += 1
    return count
print(bit_count(13))
`,
    nextUrl: /\/learn\/py-137-kadane/,
    cursorAfter: "137",
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

test.describe("micro-steps 131–136 · Two pointers / window / backtrack / trie / bits", () => {
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
      if (nextMicro <= 576) {
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
