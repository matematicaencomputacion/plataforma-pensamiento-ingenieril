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
    micro: 703,
    id: "py-703-segtree-sum",
    title: "DSA SegTree · Range Sum",
    solution: "class SegSum:\n    def __init__(self, nums):\n        self.n = len(nums)\n        self.t = [0] * (4 * self.n)\n        self._build(nums, 1, 0, self.n - 1)\n    def _build(self, a, v, tl, tr):\n        if tl == tr:\n            self.t[v] = a[tl]\n            return\n        tm = (tl + tr) // 2\n        self._build(a, v * 2, tl, tm)\n        self._build(a, v * 2 + 1, tm + 1, tr)\n        self.t[v] = self.t[v * 2] + self.t[v * 2 + 1]\n    def update(self, idx, val):\n        self._upd(1, 0, self.n - 1, idx, val)\n    def _upd(self, v, tl, tr, idx, val):\n        if tl == tr:\n            self.t[v] = val\n            return\n        tm = (tl + tr) // 2\n        if idx <= tm:\n            self._upd(v * 2, tl, tm, idx, val)\n        else:\n            self._upd(v * 2 + 1, tm + 1, tr, idx, val)\n        self.t[v] = self.t[v * 2] + self.t[v * 2 + 1]\n    def query(self, l, r):\n        return self._qry(1, 0, self.n - 1, l, r)\n    def _qry(self, v, tl, tr, l, r):\n        if l > r:\n            return 0\n        if l == tl and r == tr:\n            return self.t[v]\n        tm = (tl + tr) // 2\n        return self._qry(v * 2, tl, tm, l, min(r, tm)) + self._qry(v * 2 + 1, tm + 1, tr, max(l, tm + 1), r)\n\nst = SegSum([1, 3, 5, 7, 9, 11])\na = st.query(1, 3)\nst.update(1, 10)\nprint([a, st.query(1, 3)])\n",
    nextUrl: /\/learn\/py-704-segtree-min/,
    cursorAfter: "704",
  },
  {
    micro: 704,
    id: "py-704-segtree-min",
    title: "DSA SegTree · Range Min",
    solution: "class SegMin:\n    def __init__(self, nums):\n        self.n = len(nums)\n        self.t = [0] * (4 * self.n)\n        self._build(nums, 1, 0, self.n - 1)\n    def _build(self, a, v, tl, tr):\n        if tl == tr:\n            self.t[v] = a[tl]\n            return\n        tm = (tl + tr) // 2\n        self._build(a, v * 2, tl, tm)\n        self._build(a, v * 2 + 1, tm + 1, tr)\n        self.t[v] = min(self.t[v * 2], self.t[v * 2 + 1])\n    def query(self, l, r):\n        return self._qry(1, 0, self.n - 1, l, r)\n    def _qry(self, v, tl, tr, l, r):\n        if l > r:\n            return 10 ** 18\n        if l == tl and r == tr:\n            return self.t[v]\n        tm = (tl + tr) // 2\n        return min(self._qry(v * 2, tl, tm, l, min(r, tm)), self._qry(v * 2 + 1, tm + 1, tr, max(l, tm + 1), r))\n\nprint(SegMin([1, 3, 2, 7, 9, 11]).query(1, 4))\n",
    nextUrl: /\/learn\/py-705-segtree-max/,
    cursorAfter: "705",
  },
  {
    micro: 705,
    id: "py-705-segtree-max",
    title: "DSA SegTree · Range Max",
    solution: "class SegMax:\n    def __init__(self, nums):\n        self.n = len(nums)\n        self.t = [0] * (4 * self.n)\n        self._build(nums, 1, 0, self.n - 1)\n    def _build(self, a, v, tl, tr):\n        if tl == tr:\n            self.t[v] = a[tl]\n            return\n        tm = (tl + tr) // 2\n        self._build(a, v * 2, tl, tm)\n        self._build(a, v * 2 + 1, tm + 1, tr)\n        self.t[v] = max(self.t[v * 2], self.t[v * 2 + 1])\n    def query(self, l, r):\n        return self._qry(1, 0, self.n - 1, l, r)\n    def _qry(self, v, tl, tr, l, r):\n        if l > r:\n            return -10 ** 18\n        if l == tl and r == tr:\n            return self.t[v]\n        tm = (tl + tr) // 2\n        return max(self._qry(v * 2, tl, tm, l, min(r, tm)), self._qry(v * 2 + 1, tm + 1, tr, max(l, tm + 1), r))\n\nprint(SegMax([1, 3, 2, 7, 9, 11]).query(2, 5))\n",
    nextUrl: /\/learn\/py-706-segtree-xor/,
    cursorAfter: "706",
  },
  {
    micro: 706,
    id: "py-706-segtree-xor",
    title: "DSA SegTree · Range XOR",
    solution: "class SegXor:\n    def __init__(self, nums):\n        self.n = len(nums)\n        self.t = [0] * (4 * self.n)\n        self._build(nums, 1, 0, self.n - 1)\n    def _build(self, a, v, tl, tr):\n        if tl == tr:\n            self.t[v] = a[tl]\n            return\n        tm = (tl + tr) // 2\n        self._build(a, v * 2, tl, tm)\n        self._build(a, v * 2 + 1, tm + 1, tr)\n        self.t[v] = self.t[v * 2] ^ self.t[v * 2 + 1]\n    def query(self, l, r):\n        return self._qry(1, 0, self.n - 1, l, r)\n    def _qry(self, v, tl, tr, l, r):\n        if l > r:\n            return 0\n        if l == tl and r == tr:\n            return self.t[v]\n        tm = (tl + tr) // 2\n        return self._qry(v * 2, tl, tm, l, min(r, tm)) ^ self._qry(v * 2 + 1, tm + 1, tr, max(l, tm + 1), r)\n\nprint(SegXor([1, 3, 4, 5]).query(0, 3))\n",
    nextUrl: /\/learn\/py-707-segtree-kth/,
    cursorAfter: "707",
  },
  {
    micro: 707,
    id: "py-707-segtree-kth",
    title: "DSA SegTree · Kth Value",
    solution: "class SegKth:\n    def __init__(self, n):\n        self.n = n\n        self.t = [0] * (4 * n)\n    def add(self, idx, delta):\n        self._upd(1, 0, self.n - 1, idx, delta)\n    def _upd(self, v, tl, tr, idx, delta):\n        if tl == tr:\n            self.t[v] += delta\n            return\n        tm = (tl + tr) // 2\n        if idx <= tm:\n            self._upd(v * 2, tl, tm, idx, delta)\n        else:\n            self._upd(v * 2 + 1, tm + 1, tr, idx, delta)\n        self.t[v] = self.t[v * 2] + self.t[v * 2 + 1]\n    def kth(self, k):\n        return self._kth(1, 0, self.n - 1, k)\n    def _kth(self, v, tl, tr, k):\n        if tl == tr:\n            return tl\n        tm = (tl + tr) // 2\n        if self.t[v * 2] >= k:\n            return self._kth(v * 2, tl, tm, k)\n        return self._kth(v * 2 + 1, tm + 1, tr, k - self.t[v * 2])\n\nsk = SegKth(5)\nfor x in [2, 0, 2, 1]:\n    sk.add(x, 1)\nprint([sk.kth(1), sk.kth(2), sk.kth(3)])\n",
    nextUrl: /\/learn\/py-708-lazy-segtree/,
    cursorAfter: "708",
  },
  {
    micro: 708,
    id: "py-708-lazy-segtree",
    title: "DSA SegTree · Lazy Add",
    solution: "class LazySeg:\n    def __init__(self, nums):\n        self.n = len(nums)\n        self.t = [0] * (4 * self.n)\n        self.lz = [0] * (4 * self.n)\n        self._build(nums, 1, 0, self.n - 1)\n    def _build(self, a, v, tl, tr):\n        if tl == tr:\n            self.t[v] = a[tl]\n            return\n        tm = (tl + tr) // 2\n        self._build(a, v * 2, tl, tm)\n        self._build(a, v * 2 + 1, tm + 1, tr)\n        self.t[v] = self.t[v * 2] + self.t[v * 2 + 1]\n    def _push(self, v, tl, tr):\n        if not self.lz[v]:\n            return\n        tm = (tl + tr) // 2\n        for ch, l, r in ((v * 2, tl, tm), (v * 2 + 1, tm + 1, tr)):\n            self.t[ch] += self.lz[v] * (r - l + 1)\n            self.lz[ch] += self.lz[v]\n        self.lz[v] = 0\n    def range_add(self, l, r, delta):\n        self._add(1, 0, self.n - 1, l, r, delta)\n    def _add(self, v, tl, tr, l, r, delta):\n        if l > r:\n            return\n        if l == tl and r == tr:\n            self.t[v] += delta * (tr - tl + 1)\n            self.lz[v] += delta\n            return\n        self._push(v, tl, tr)\n        tm = (tl + tr) // 2\n        self._add(v * 2, tl, tm, l, min(r, tm), delta)\n        self._add(v * 2 + 1, tm + 1, tr, max(l, tm + 1), r, delta)\n        self.t[v] = self.t[v * 2] + self.t[v * 2 + 1]\n    def query(self, l, r):\n        return self._qry(1, 0, self.n - 1, l, r)\n    def _qry(self, v, tl, tr, l, r):\n        if l > r:\n            return 0\n        if l == tl and r == tr:\n            return self.t[v]\n        self._push(v, tl, tr)\n        tm = (tl + tr) // 2\n        return self._qry(v * 2, tl, tm, l, min(r, tm)) + self._qry(v * 2 + 1, tm + 1, tr, max(l, tm + 1), r)\n\nls = LazySeg([1, 2, 3, 4, 5])\nls.range_add(1, 3, 10)\nprint(ls.query(0, 4))\n",
    nextUrl: /\/learn\/py-709-tree-parents/,
    cursorAfter: "709",
  },
];

test("declares the contiguous learn-route family", () => {
  for (const step of FAMILY) {
    expect(step.id).toMatch(/^py-(?:703|704|705|706|707|708)-/);
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

test.describe("micro-steps 703–708 · segtree", () => {
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
      if (nextMicro <= 708) {
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

      if (step.micro < 708) {
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
