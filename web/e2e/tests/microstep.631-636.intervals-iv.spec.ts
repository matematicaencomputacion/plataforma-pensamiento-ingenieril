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
    micro: 631,
    id: "py-631-insert-interval",
    title: "DSA Intervals IV · Insert Interval",
    solution: "def insert(intervals, new_interval):\n    out = []\n    i, n = 0, len(intervals)\n    s, e = new_interval\n    while i < n and intervals[i][1] < s:\n        out.append(intervals[i]); i += 1\n    while i < n and intervals[i][0] <= e:\n        s = min(s, intervals[i][0]); e = max(e, intervals[i][1]); i += 1\n    out.append([s, e])\n    out.extend(intervals[i:])\n    return out\n\nprint(insert([[1, 3], [6, 9]], [2, 5]))\n",
    nextUrl: /\/learn\/py-632-erase-overlap/,
    cursorAfter: "632",
  },
  {
    micro: 632,
    id: "py-632-erase-overlap",
    title: "DSA Intervals IV · Erase Overlap",
    solution: "def erase_overlap_intervals(intervals):\n    intervals.sort(key=lambda x: x[1])\n    end = float('-inf')\n    keep = 0\n    for s, e in intervals:\n        if s >= end:\n            keep += 1\n            end = e\n    return len(intervals) - keep\n\nprint(erase_overlap_intervals([[1, 2], [2, 3], [3, 4], [1, 3]]))\n",
    nextUrl: /\/learn\/py-633-min-meeting-rooms/,
    cursorAfter: "633",
  },
  {
    micro: 633,
    id: "py-633-min-meeting-rooms",
    title: "DSA Intervals IV · Meeting Rooms",
    solution: "def min_meeting_rooms(intervals):\n    starts = sorted(s for s, _ in intervals)\n    ends = sorted(e for _, e in intervals)\n    i = rooms = best = 0\n    for s in starts:\n        rooms += 1\n        while i < len(ends) and ends[i] <= s:\n            rooms -= 1\n            i += 1\n        best = max(best, rooms)\n    return best\n\nprint(min_meeting_rooms([[0, 30], [5, 10], [15, 20]]))\n",
    nextUrl: /\/learn\/py-634-interval-intersect/,
    cursorAfter: "634",
  },
  {
    micro: 634,
    id: "py-634-interval-intersect",
    title: "DSA Intervals IV · Intersections",
    solution: "def interval_intersection(first, second):\n    i = j = 0\n    out = []\n    while i < len(first) and j < len(second):\n        lo = max(first[i][0], second[j][0])\n        hi = min(first[i][1], second[j][1])\n        if lo <= hi:\n            out.append([lo, hi])\n        if first[i][1] < second[j][1]:\n            i += 1\n        else:\n            j += 1\n    return out\n\nprint(interval_intersection([[0, 2], [5, 10], [13, 23], [24, 25]], [[1, 5], [8, 12], [15, 24], [25, 26]]))\n",
    nextUrl: /\/learn\/py-635-burst-balloons-arrows/,
    cursorAfter: "635",
  },
  {
    micro: 635,
    id: "py-635-burst-balloons-arrows",
    title: "DSA Intervals IV · Min Arrows",
    solution: "def find_min_arrow_shots(points):\n    points.sort(key=lambda p: p[1])\n    arrows = 0\n    end = float('-inf')\n    for s, e in points:\n        if s > end:\n            arrows += 1\n            end = e\n    return arrows\n\nprint(find_min_arrow_shots([[10, 16], [2, 8], [1, 6], [7, 12]]))\n",
    nextUrl: /\/learn\/py-636-video-stitching/,
    cursorAfter: "636",
  },
  {
    micro: 636,
    id: "py-636-video-stitching",
    title: "DSA Intervals IV · Video Stitching",
    solution: "def video_stitching(clips, time):\n    farthest = [0] * (time + 1)\n    for s, e in clips:\n        if s <= time:\n            farthest[s] = max(farthest[s], min(e, time))\n    used = cur = reach = 0\n    for i in range(time):\n        reach = max(reach, farthest[i])\n        if i == cur:\n            if reach == cur:\n                return -1\n            used += 1\n            cur = reach\n    return used\n\nprint(video_stitching([[0, 2], [4, 6], [8, 10], [1, 9], [1, 5], [5, 9]], 10))\n",
    nextUrl: /\/learn\/py-637-jump-game-ii/,
    cursorAfter: "637",
  },
];

test("declares the contiguous learn-route family", () => {
  for (const step of FAMILY) {
    expect(step.id).toMatch(/^py-(?:631|632|633|634|635|636)-/);
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

test.describe("micro-steps 631–636 · intervals iv", () => {
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

      if (step.micro < 636) {
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
