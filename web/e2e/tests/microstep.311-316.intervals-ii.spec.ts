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
    micro: 311,
    id: "py-311-min-arrows",
    title: "DSA Min Arrows",
    solution: `def find_min_arrow_shots(points):
    points = sorted(points, key=lambda p: p[1])
    arrows = 0
    end = float('-inf')
    for s, e in points:
        if s > end:
            arrows += 1
            end = e
    return arrows

print(find_min_arrow_shots([[10, 16], [2, 8], [1, 6], [7, 12]]))
`,
    nextUrl: /\/learn\/py-312-car-pooling/,
    cursorAfter: "312",
  },
  {
    micro: 312,
    id: "py-312-car-pooling",
    title: "DSA Car Pooling",
    solution: `def car_pooling(trips, capacity):
    diff = [0] * 1001
    for num, start, end in trips:
        diff[start] += num
        diff[end] -= num
    cur = 0
    for x in diff:
        cur += x
        if cur > capacity:
            return False
    return True

print(car_pooling([[2, 1, 5], [3, 3, 7]], 4))
`,
    nextUrl: /\/learn\/py-313-interval-intersect/,
    cursorAfter: "313",
  },
  {
    micro: 313,
    id: "py-313-interval-intersect",
    title: "DSA Interval Intersection",
    solution: `def interval_intersection(first, second):
    i = j = 0
    out = []
    while i < len(first) and j < len(second):
        lo = max(first[i][0], second[j][0])
        hi = min(first[i][1], second[j][1])
        if lo <= hi:
            out.append([lo, hi])
        if first[i][1] < second[j][1]:
            i += 1
        else:
            j += 1
    return out

print(interval_intersection([[0, 2], [5, 10], [13, 23], [24, 25]], [[1, 5], [8, 12], [15, 24], [25, 26]]))
`,
    nextUrl: /\/learn\/py-314-my-calendar/,
    cursorAfter: "314",
  },
  {
    micro: 314,
    id: "py-314-my-calendar",
    title: "DSA My Calendar",
    solution: `class MyCalendar:
    def __init__(self):
        self.books = []

    def book(self, start, end):
        for s, e in self.books:
            if start < e and end > s:
                return False
        self.books.append((start, end))
        return True

cal = MyCalendar()
print([cal.book(10, 20), cal.book(15, 25), cal.book(20, 30)])
`,
    nextUrl: /\/learn\/py-315-non-overlap/,
    cursorAfter: "315",
  },
  {
    micro: 315,
    id: "py-315-non-overlap",
    title: "DSA Non Overlap",
    solution: `def max_non_overlapping(intervals):
    intervals = sorted(intervals, key=lambda x: x[1])
    keep = 0
    end = float('-inf')
    for s, e in intervals:
        if s >= end:
            keep += 1
            end = e
    return keep

print(max_non_overlapping([[1, 2], [2, 3], [3, 4], [1, 3]]))
`,
    nextUrl: /\/learn\/py-316-video-stitch/,
    cursorAfter: "316",
  },
  {
    micro: 316,
    id: "py-316-video-stitch",
    title: "DSA Video Stitch",
    solution: `def video_stitching(clips, time):
    clips = sorted(clips)
    end = farthest = used = i = 0
    while end < time:
        while i < len(clips) and clips[i][0] <= end:
            farthest = max(farthest, clips[i][1])
            i += 1
        if farthest == end:
            return -1
        used += 1
        end = farthest
    return used

print(video_stitching([[0, 2], [4, 6], [8, 10], [1, 9], [1, 5], [5, 9]], 10))
`,
    nextUrl: /\/learn\/py-317-permutations/,
    cursorAfter: "317",
  },
];

test("declares the contiguous learn-route family", () => {
  for (const step of FAMILY) {
    expect(step.id).toMatch(/^py-31[1-6]-/);
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

test.describe("micro-steps 311–316 · intervals II", () => {
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
      if (nextMicro <= 528) {
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
