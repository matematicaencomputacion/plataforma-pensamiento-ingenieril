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
    micro: 685,
    id: "py-685-furthest-building",
    title: "DSA Heaps IV · Furthest Building",
    solution: "import heapq\n\ndef furthest_building(heights, bricks, ladders):\n    climbs = []\n    for i in range(len(heights) - 1):\n        d = heights[i + 1] - heights[i]\n        if d <= 0:\n            continue\n        heapq.heappush(climbs, d)\n        if len(climbs) > ladders:\n            bricks -= heapq.heappop(climbs)\n            if bricks < 0:\n                return i\n    return len(heights) - 1\n\nprint(furthest_building([4, 2, 7, 6, 9, 14, 12], 5, 1))\n",
    nextUrl: /\/learn\/py-686-k-smallest-pairs/,
    cursorAfter: "686",
  },
  {
    micro: 686,
    id: "py-686-k-smallest-pairs",
    title: "DSA Heaps IV · K Pairs",
    solution: "import heapq\n\ndef k_smallest_pairs(nums1, nums2, k):\n    if not nums1 or not nums2:\n        return []\n    h = [(nums1[0] + nums2[j], 0, j) for j in range(min(k, len(nums2)))]\n    heapq.heapify(h)\n    out = []\n    while h and len(out) < k:\n        _, i, j = heapq.heappop(h)\n        out.append([nums1[i], nums2[j]])\n        if i + 1 < len(nums1):\n            heapq.heappush(h, (nums1[i + 1] + nums2[j], i + 1, j))\n    return out\n\nprint(k_smallest_pairs([1, 7, 11], [2, 4, 6], 3))\n",
    nextUrl: /\/learn\/py-687-kth-largest-stream/,
    cursorAfter: "687",
  },
  {
    micro: 687,
    id: "py-687-kth-largest-stream",
    title: "DSA Heaps IV · Kth Stream",
    solution: "import heapq\n\nclass KthLargest:\n    def __init__(self, k, nums):\n        self.k = k\n        self.h = list(nums)\n        heapq.heapify(self.h)\n        while len(self.h) > k:\n            heapq.heappop(self.h)\n    def add(self, val):\n        heapq.heappush(self.h, val)\n        if len(self.h) > self.k:\n            heapq.heappop(self.h)\n        return self.h[0]\n\nkth = KthLargest(3, [4, 5, 8, 2])\nprint([kth.add(3), kth.add(5), kth.add(10), kth.add(9), kth.add(4)])\n",
    nextUrl: /\/learn\/py-688-connect-sticks/,
    cursorAfter: "688",
  },
  {
    micro: 688,
    id: "py-688-connect-sticks",
    title: "DSA Heaps IV · Connect Sticks",
    solution: "import heapq\n\ndef connect_sticks(sticks):\n    heapq.heapify(sticks)\n    cost = 0\n    while len(sticks) > 1:\n        a = heapq.heappop(sticks)\n        b = heapq.heappop(sticks)\n        s = a + b\n        cost += s\n        heapq.heappush(sticks, s)\n    return cost\n\nprint(connect_sticks([2, 4, 3]))\n",
    nextUrl: /\/learn\/py-689-ipo/,
    cursorAfter: "689",
  },
  {
    micro: 689,
    id: "py-689-ipo",
    title: "DSA Heaps IV · IPO",
    solution: "import heapq\n\ndef find_maximized_capital(k, w, profits, capital):\n    projects = sorted(zip(capital, profits))\n    i, h = 0, []\n    for _ in range(k):\n        while i < len(projects) and projects[i][0] <= w:\n            heapq.heappush(h, -projects[i][1])\n            i += 1\n        if not h:\n            break\n        w += -heapq.heappop(h)\n    return w\n\nprint(find_maximized_capital(2, 0, [1, 2, 3], [0, 1, 1]))\n",
    nextUrl: /\/learn\/py-690-frequency-sort/,
    cursorAfter: "690",
  },
  {
    micro: 690,
    id: "py-690-frequency-sort",
    title: "DSA Heaps IV · Frequency Sort",
    solution: "import heapq\nfrom collections import Counter\n\ndef frequency_sort(s):\n    h = [(-c, ch) for ch, c in Counter(s).items()]\n    heapq.heapify(h)\n    out = []\n    while h:\n        c, ch = heapq.heappop(h)\n        out.append(ch * (-c))\n    return ''.join(out)\n\nprint(frequency_sort(\"tree\"))\n",
    nextUrl: /\/learn\/py-691-prefix-function/,
    cursorAfter: "691",
  },
];

test("declares the contiguous learn-route family", () => {
  for (const step of FAMILY) {
    expect(step.id).toMatch(/^py-(?:685|686|687|688|689|690)-/);
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

test.describe("micro-steps 685–690 · heaps iv", () => {
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
      if (nextMicro <= 690) {
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

      if (step.micro < 690) {
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
