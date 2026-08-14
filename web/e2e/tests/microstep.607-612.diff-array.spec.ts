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
    micro: 607,
    id: "py-607-range-addition",
    title: "DSA Diff Array · Range Addition",
    solution: "def get_modified_array(length, updates):\n    diff = [0] * (length + 1)\n    for left, right, val in updates:\n        diff[left] += val\n        diff[right + 1] -= val\n    out = []\n    running = 0\n    for i in range(length):\n        running += diff[i]\n        out.append(running)\n    return out\n\nprint(get_modified_array(5, [[1, 3, 2], [2, 4, 3], [0, 2, -2]]))\n",
    nextUrl: /\/learn\/py-608-flight-bookings/,
    cursorAfter: "608",
  },
  {
    micro: 608,
    id: "py-608-flight-bookings",
    title: "DSA Diff Array · Flight Bookings",
    solution: "def corp_flight_bookings(bookings, n):\n    diff = [0] * (n + 1)\n    for first, last, seats in bookings:\n        diff[first - 1] += seats\n        diff[last] -= seats\n    out = []\n    running = 0\n    for i in range(n):\n        running += diff[i]\n        out.append(running)\n    return out\n\nprint(corp_flight_bookings([[1, 2, 10], [2, 3, 20], [2, 5, 25]], 5))\n",
    nextUrl: /\/learn\/py-609-car-pooling/,
    cursorAfter: "609",
  },
  {
    micro: 609,
    id: "py-609-car-pooling",
    title: "DSA Diff Array · Car Pooling",
    solution: "def car_pooling(trips, capacity):\n    diff = [0] * 1001\n    for passengers, start, end in trips:\n        diff[start] += passengers\n        diff[end] -= passengers\n    load = 0\n    for delta in diff:\n        load += delta\n        if load > capacity:\n            return False\n    return True\n\nprint(car_pooling([[2, 1, 5], [3, 3, 7]], 4))\n",
    nextUrl: /\/learn\/py-610-range-addition-ii/,
    cursorAfter: "610",
  },
  {
    micro: 610,
    id: "py-610-range-addition-ii",
    title: "DSA Diff Array · Max Count",
    solution: "def max_count(m, n, ops):\n    min_a, min_b = m, n\n    for a, b in ops:\n        min_a = min(min_a, a)\n        min_b = min(min_b, b)\n    return min_a * min_b\n\nprint(max_count(3, 3, [[2, 2], [3, 3]]))\n",
    nextUrl: /\/learn\/py-611-population-year/,
    cursorAfter: "611",
  },
  {
    micro: 611,
    id: "py-611-population-year",
    title: "DSA Diff Array · Max Population",
    solution: "def maximum_population(logs):\n    diff = [0] * 101\n    for birth, death in logs:\n        diff[birth - 1950] += 1\n        diff[death - 1950] -= 1\n    best = year = running = 0\n    for i, delta in enumerate(diff):\n        running += delta\n        if running > best:\n            best = running\n            year = 1950 + i\n    return year\n\nprint(maximum_population([[1993, 1999], [2000, 2010]]))\n",
    nextUrl: /\/learn\/py-612-points-that-intersect/,
    cursorAfter: "612",
  },
  {
    micro: 612,
    id: "py-612-points-that-intersect",
    title: "DSA Diff Array · Intersecting Points",
    solution: "def number_of_points(nums):\n    diff = [0] * 102\n    for start, end in nums:\n        diff[start] += 1\n        diff[end + 1] -= 1\n    cover = total = 0\n    for i in range(1, 101):\n        cover += diff[i]\n        if cover >= 2:\n            total += 1\n    return total\n\nprint(number_of_points([[1, 3], [2, 4]]))\n",
    nextUrl: /\/workspace/,
    cursorAfter: "613",
  },
];

test("declares the contiguous learn-route family", () => {
  for (const step of FAMILY) {
    expect(step.id).toMatch(/^py-(?:607|608|609|610|611|612)-/);
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

test.describe("micro-steps 607–612 · diff array", () => {
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
      if (nextMicro <= 612) {
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

      if (step.micro < 612) {
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
