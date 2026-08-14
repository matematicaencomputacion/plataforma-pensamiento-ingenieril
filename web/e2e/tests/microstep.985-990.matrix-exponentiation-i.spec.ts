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
    micro: 985,
    id: "py-985-matrix-exponentiation-i-canonicalize",
    title: "DSA Matrix Exponentiation I · Canonicalize",
    solution: "def canonicalize_matrix_exponentiation_i(values):\n    return sorted(set(values))\n\nprint(canonicalize_matrix_exponentiation_i([3, 1, 2, 3, 1]))\n",
    nextUrl: /\/learn\/py-986-matrix-exponentiation-i-prefix-state/,
    cursorAfter: "986",
  },
  {
    micro: 986,
    id: "py-986-matrix-exponentiation-i-prefix-state",
    title: "DSA Matrix Exponentiation I · Prefix State",
    solution: "def prefix_state_matrix_exponentiation_i(values):\n    out = []\n    running = 0\n    for value in values:\n        running += value\n        out.append(running)\n    return out\n\nprint(prefix_state_matrix_exponentiation_i([3, 1, 4]))\n",
    nextUrl: /\/learn\/py-987-matrix-exponentiation-i-bounded-window/,
    cursorAfter: "987",
  },
  {
    micro: 987,
    id: "py-987-matrix-exponentiation-i-bounded-window",
    title: "DSA Matrix Exponentiation I · Bounded Window",
    solution: "def best_window_matrix_exponentiation_i(values, width):\n    if width <= 0 or width > len(values):\n        raise ValueError(\"invalid width\")\n    current = sum(values[:width])\n    best = current\n    for right in range(width, len(values)):\n        current += values[right] - values[right - width]\n        best = max(best, current)\n    return best\n\nprint(best_window_matrix_exponentiation_i([2, 1, 5, 1, 3], 3))\n",
    nextUrl: /\/learn\/py-988-matrix-exponentiation-i-lower-boundary/,
    cursorAfter: "988",
  },
  {
    micro: 988,
    id: "py-988-matrix-exponentiation-i-lower-boundary",
    title: "DSA Matrix Exponentiation I · Lower Boundary",
    solution: "def lower_boundary_matrix_exponentiation_i(values, target):\n    lo, hi = 0, len(values)\n    while lo < hi:\n        mid = (lo + hi) // 2\n        if values[mid] < target:\n            lo = mid + 1\n        else:\n            hi = mid\n    return lo\n\nprint(lower_boundary_matrix_exponentiation_i([1, 3, 3, 7], 3))\n",
    nextUrl: /\/learn\/py-989-matrix-exponentiation-i-dependency-order/,
    cursorAfter: "989",
  },
  {
    micro: 989,
    id: "py-989-matrix-exponentiation-i-dependency-order",
    title: "DSA Matrix Exponentiation I · Dependency Order",
    solution: "from collections import deque\n\ndef dependency_order_matrix_exponentiation_i(graph, start):\n    queue = deque([start])\n    seen = {start}\n    order = []\n    while queue:\n        node = queue.popleft()\n        order.append(node)\n        for neighbor in graph[node]:\n            if neighbor not in seen:\n                seen.add(neighbor)\n                queue.append(neighbor)\n    return order\n\nprint(dependency_order_matrix_exponentiation_i([[1, 2], [3], [3], []], 0))\n",
    nextUrl: /\/learn\/py-990-matrix-exponentiation-i-minimum-transition/,
    cursorAfter: "990",
  },
  {
    micro: 990,
    id: "py-990-matrix-exponentiation-i-minimum-transition",
    title: "DSA Matrix Exponentiation I · Minimum Transition",
    solution: "def minimum_transition_matrix_exponentiation_i(cost):\n    two_back = one_back = 0\n    for value in cost:\n        two_back, one_back = one_back, value + min(two_back, one_back)\n    return min(two_back, one_back)\n\nprint(minimum_transition_matrix_exponentiation_i([10, 15, 20]))\n",
    nextUrl: /\/workspace/,
    cursorAfter: "991",
  },
];

test("declares the contiguous learn-route family", () => {
  for (const step of FAMILY) {
    expect(step.id).toMatch(/^py-(?:985|986|987|988|989|990)-/);
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

test.describe("micro-steps 985–990 · matrix exponentiation i", () => {
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
      if (nextMicro <= 990) {
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

      if (step.micro < 990) {
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
