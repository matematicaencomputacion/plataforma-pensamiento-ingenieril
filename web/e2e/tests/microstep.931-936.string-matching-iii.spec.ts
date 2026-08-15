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
    micro: 931,
    id: "py-931-string-matching-iii-canonicalize",
    title: "DSA String Matching III · Canonicalize",
    solution: "def canonicalize_string_matching_iii(values):\n    return sorted(set(values))\n\nprint(canonicalize_string_matching_iii([3, 1, 2, 3, 1]))\n",
    nextUrl: /\/learn\/py-932-string-matching-iii-prefix-state/,
    cursorAfter: "932",
  },
  {
    micro: 932,
    id: "py-932-string-matching-iii-prefix-state",
    title: "DSA String Matching III · Prefix State",
    solution: "def prefix_state_string_matching_iii(values):\n    out = []\n    running = 0\n    for value in values:\n        running += value\n        out.append(running)\n    return out\n\nprint(prefix_state_string_matching_iii([3, 1, 4]))\n",
    nextUrl: /\/learn\/py-933-string-matching-iii-bounded-window/,
    cursorAfter: "933",
  },
  {
    micro: 933,
    id: "py-933-string-matching-iii-bounded-window",
    title: "DSA String Matching III · Bounded Window",
    solution: "def best_window_string_matching_iii(values, width):\n    if width <= 0 or width > len(values):\n        raise ValueError(\"invalid width\")\n    current = sum(values[:width])\n    best = current\n    for right in range(width, len(values)):\n        current += values[right] - values[right - width]\n        best = max(best, current)\n    return best\n\nprint(best_window_string_matching_iii([2, 1, 5, 1, 3], 3))\n",
    nextUrl: /\/learn\/py-934-string-matching-iii-lower-boundary/,
    cursorAfter: "934",
  },
  {
    micro: 934,
    id: "py-934-string-matching-iii-lower-boundary",
    title: "DSA String Matching III · Lower Boundary",
    solution: "def lower_boundary_string_matching_iii(values, target):\n    lo, hi = 0, len(values)\n    while lo < hi:\n        mid = (lo + hi) // 2\n        if values[mid] < target:\n            lo = mid + 1\n        else:\n            hi = mid\n    return lo\n\nprint(lower_boundary_string_matching_iii([1, 3, 3, 7], 3))\n",
    nextUrl: /\/learn\/py-935-string-matching-iii-dependency-order/,
    cursorAfter: "935",
  },
  {
    micro: 935,
    id: "py-935-string-matching-iii-dependency-order",
    title: "DSA String Matching III · Dependency Order",
    solution: "from collections import deque\n\ndef dependency_order_string_matching_iii(graph, start):\n    queue = deque([start])\n    seen = {start}\n    order = []\n    while queue:\n        node = queue.popleft()\n        order.append(node)\n        for neighbor in graph[node]:\n            if neighbor not in seen:\n                seen.add(neighbor)\n                queue.append(neighbor)\n    return order\n\nprint(dependency_order_string_matching_iii([[1, 2], [3], [3], []], 0))\n",
    nextUrl: /\/learn\/py-936-string-matching-iii-minimum-transition/,
    cursorAfter: "936",
  },
  {
    micro: 936,
    id: "py-936-string-matching-iii-minimum-transition",
    title: "DSA String Matching III · Minimum Transition",
    solution: "def minimum_transition_string_matching_iii(cost):\n    two_back = one_back = 0\n    for value in cost:\n        two_back, one_back = one_back, value + min(two_back, one_back)\n    return min(two_back, one_back)\n\nprint(minimum_transition_string_matching_iii([10, 15, 20]))\n",
    nextUrl: /\/learn\/py-937-rolling-hash-ii-canonicalize/,
    cursorAfter: "937",
  },
];

test("declares the contiguous learn-route family", () => {
  for (const step of FAMILY) {
    expect(step.id).toMatch(/^py-(?:931|932|933|934|935|936)-/);
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

test.describe("micro-steps 931–936 · string matching iii", () => {
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
      if (nextMicro <= 1000) {
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
