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
    micro: 697,
    id: "py-697-fenwick-prefix",
    title: "DSA Fenwick · Prefix Sum",
    solution: "class Fenwick:\n    def __init__(self, n):\n        self.bit = [0] * (n + 1)\n    def add(self, i, delta):\n        i += 1\n        while i < len(self.bit):\n            self.bit[i] += delta\n            i += i & -i\n    def prefix(self, i):\n        i += 1\n        s = 0\n        while i > 0:\n            s += self.bit[i]\n            i -= i & -i\n        return s\n    def range_sum(self, l, r):\n        if l == 0:\n            return self.prefix(r)\n        return self.prefix(r) - self.prefix(l - 1)\n\nfw = Fenwick(5)\nfor i, v in enumerate([1, 2, 3, 4, 5]):\n    fw.add(i, v)\nprint(fw.prefix(4))\n",
    nextUrl: /\/learn\/py-698-num-array-bit/,
    cursorAfter: "698",
  },
  {
    micro: 698,
    id: "py-698-num-array-bit",
    title: "DSA Fenwick · Mutable Range",
    solution: "class Fenwick:\n    def __init__(self, n):\n        self.bit = [0] * (n + 1)\n    def add(self, i, delta):\n        i += 1\n        while i < len(self.bit):\n            self.bit[i] += delta\n            i += i & -i\n    def prefix(self, i):\n        i += 1\n        s = 0\n        while i > 0:\n            s += self.bit[i]\n            i -= i & -i\n        return s\n    def range_sum(self, l, r):\n        if l == 0:\n            return self.prefix(r)\n        return self.prefix(r) - self.prefix(l - 1)\n\nclass NumArray:\n    def __init__(self, nums):\n        self.nums = list(nums)\n        self.fw = Fenwick(len(nums))\n        for i, v in enumerate(nums):\n            self.fw.add(i, v)\n    def update(self, index, val):\n        self.fw.add(index, val - self.nums[index])\n        self.nums[index] = val\n    def sum_range(self, left, right):\n        return self.fw.range_sum(left, right)\n\nna = NumArray([1, 3, 5])\na = na.sum_range(0, 2)\nna.update(1, 2)\nprint([a, na.sum_range(0, 2)])\n",
    nextUrl: /\/learn\/py-699-count-inversions/,
    cursorAfter: "699",
  },
  {
    micro: 699,
    id: "py-699-count-inversions",
    title: "DSA Fenwick · Inversions",
    solution: "class Fenwick:\n    def __init__(self, n):\n        self.bit = [0] * (n + 1)\n    def add(self, i, delta):\n        i += 1\n        while i < len(self.bit):\n            self.bit[i] += delta\n            i += i & -i\n    def prefix(self, i):\n        i += 1\n        s = 0\n        while i > 0:\n            s += self.bit[i]\n            i -= i & -i\n        return s\n    def range_sum(self, l, r):\n        if l == 0:\n            return self.prefix(r)\n        return self.prefix(r) - self.prefix(l - 1)\n\ndef count_inversions(nums):\n    ranks = {v: i for i, v in enumerate(sorted(set(nums)))}\n    fw = Fenwick(len(ranks))\n    inv = 0\n    for x in reversed(nums):\n        r = ranks[x]\n        if r:\n            inv += fw.prefix(r - 1)\n        fw.add(r, 1)\n    return inv\n\nprint(count_inversions([2, 4, 1, 3, 5]))\n",
    nextUrl: /\/learn\/py-700-reverse-pairs/,
    cursorAfter: "700",
  },
  {
    micro: 700,
    id: "py-700-reverse-pairs",
    title: "DSA Fenwick · Reverse Pairs",
    solution: "class Fenwick:\n    def __init__(self, n):\n        self.bit = [0] * (n + 1)\n    def add(self, i, delta):\n        i += 1\n        while i < len(self.bit):\n            self.bit[i] += delta\n            i += i & -i\n    def prefix(self, i):\n        i += 1\n        s = 0\n        while i > 0:\n            s += self.bit[i]\n            i -= i & -i\n        return s\n    def range_sum(self, l, r):\n        if l == 0:\n            return self.prefix(r)\n        return self.prefix(r) - self.prefix(l - 1)\n\ndef reverse_pairs(nums):\n    vals = sorted(set(nums + [2 * x for x in nums]))\n    rank = {v: i for i, v in enumerate(vals)}\n    fw = Fenwick(len(vals))\n    ans = 0\n    last = len(vals) - 1\n    for x in nums:\n        lo = rank[2 * x] + 1\n        if lo <= last:\n            ans += fw.range_sum(lo, last)\n        fw.add(rank[x], 1)\n    return ans\n\nprint(reverse_pairs([1, 3, 2, 3, 1]))\n",
    nextUrl: /\/learn\/py-701-count-smaller/,
    cursorAfter: "701",
  },
  {
    micro: 701,
    id: "py-701-count-smaller",
    title: "DSA Fenwick · Count Smaller",
    solution: "class Fenwick:\n    def __init__(self, n):\n        self.bit = [0] * (n + 1)\n    def add(self, i, delta):\n        i += 1\n        while i < len(self.bit):\n            self.bit[i] += delta\n            i += i & -i\n    def prefix(self, i):\n        i += 1\n        s = 0\n        while i > 0:\n            s += self.bit[i]\n            i -= i & -i\n        return s\n    def range_sum(self, l, r):\n        if l == 0:\n            return self.prefix(r)\n        return self.prefix(r) - self.prefix(l - 1)\n\ndef count_smaller(nums):\n    ranks = {v: i for i, v in enumerate(sorted(set(nums)))}\n    fw = Fenwick(len(ranks))\n    out = [0] * len(nums)\n    for i in range(len(nums) - 1, -1, -1):\n        r = ranks[nums[i]]\n        out[i] = fw.prefix(r - 1) if r else 0\n        fw.add(r, 1)\n    return out\n\nprint(count_smaller([5, 2, 6, 1]))\n",
    nextUrl: /\/learn\/py-702-fenwick-kth/,
    cursorAfter: "702",
  },
  {
    micro: 702,
    id: "py-702-fenwick-kth",
    title: "DSA Fenwick · Kth Smallest",
    solution: "class Fenwick:\n    def __init__(self, n):\n        self.bit = [0] * (n + 1)\n    def add(self, i, delta):\n        i += 1\n        while i < len(self.bit):\n            self.bit[i] += delta\n            i += i & -i\n    def prefix(self, i):\n        i += 1\n        s = 0\n        while i > 0:\n            s += self.bit[i]\n            i -= i & -i\n        return s\n    def range_sum(self, l, r):\n        if l == 0:\n            return self.prefix(r)\n        return self.prefix(r) - self.prefix(l - 1)\n\ndef kth_smallest(nums, k):\n    vals = sorted(set(nums))\n    rank = {v: i for i, v in enumerate(vals)}\n    fw = Fenwick(len(vals))\n    for x in nums:\n        fw.add(rank[x], 1)\n    lo, hi = 0, len(vals) - 1\n    while lo < hi:\n        mid = (lo + hi) // 2\n        if fw.prefix(mid) >= k:\n            hi = mid\n        else:\n            lo = mid + 1\n    return vals[lo]\n\nprint(kth_smallest([7, 10, 4, 3, 20, 15], 3))\n",
    nextUrl: /\/workspace/,
    cursorAfter: "703",
  },
];

test("declares the contiguous learn-route family", () => {
  for (const step of FAMILY) {
    expect(step.id).toMatch(/^py-(?:697|698|699|700|701|702)-/);
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

test.describe("micro-steps 697–702 · fenwick", () => {
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
      if (nextMicro <= 702) {
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

      if (step.micro < 702) {
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
