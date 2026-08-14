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
    micro: 493,
    id: "py-493-sort-colors",
    title: "DSA Sort Colors",
    solution: `def sort_colors(nums):
    lo = mid = 0
    hi = len(nums) - 1
    while mid <= hi:
        if nums[mid] == 0:
            nums[lo], nums[mid] = nums[mid], nums[lo]
            lo += 1; mid += 1
        elif nums[mid] == 1:
            mid += 1
        else:
            nums[mid], nums[hi] = nums[hi], nums[mid]
            hi -= 1
    return nums

print(sort_colors([2, 0, 2, 1, 1, 0]))
`,
    nextUrl: /\/learn\/py-494-merge-intervals/,
    cursorAfter: "494",
  },
  {
    micro: 494,
    id: "py-494-merge-intervals",
    title: "DSA Merge Intervals",
    solution: `def merge(intervals):
    intervals.sort()
    out = [intervals[0]]
    for s, e in intervals[1:]:
        if s <= out[-1][1]:
            out[-1][1] = max(out[-1][1], e)
        else:
            out.append([s, e])
    return out

print(merge([[1, 3], [2, 6], [8, 10], [15, 18]]))
`,
    nextUrl: /\/learn\/py-495-insert-interval/,
    cursorAfter: "495",
  },
  {
    micro: 495,
    id: "py-495-insert-interval",
    title: "DSA Insert Interval",
    solution: `def insert(intervals, new_interval):
    res = []
    s, e = new_interval
    i = 0
    n = len(intervals)
    while i < n and intervals[i][1] < s:
        res.append(intervals[i]); i += 1
    while i < n and intervals[i][0] <= e:
        s = min(s, intervals[i][0]); e = max(e, intervals[i][1]); i += 1
    res.append([s, e])
    res.extend(intervals[i:])
    return res

print(insert([[1, 3], [6, 9]], [2, 5]))
`,
    nextUrl: /\/learn\/py-496-largest-number/,
    cursorAfter: "496",
  },
  {
    micro: 496,
    id: "py-496-largest-number",
    title: "DSA Largest Number",
    solution: `def largest_number(nums):
    from functools import cmp_to_key
    s = [str(x) for x in nums]
    s.sort(key=cmp_to_key(lambda a, b: (a + b < b + a) - (a + b > b + a)))
    if s[0] == "0":
        return "0"
    return "".join(s)

print(largest_number([10, 2]))
`,
    nextUrl: /\/learn\/py-497-sort-by-parity/,
    cursorAfter: "497",
  },
  {
    micro: 497,
    id: "py-497-sort-by-parity",
    title: "DSA Sort Parity",
    solution: `def sort_array_by_parity(nums):
    i = 0
    for j, x in enumerate(nums):
        if x % 2 == 0:
            nums[i], nums[j] = nums[j], nums[i]
            i += 1
    return nums

print(sort_array_by_parity([3, 1, 2, 4]))
`,
    nextUrl: /\/learn\/py-498-wiggle-sort/,
    cursorAfter: "498",
  },
  {
    micro: 498,
    id: "py-498-wiggle-sort",
    title: "DSA Wiggle Sort",
    solution: `def wiggle_sort(nums):
    nums.sort()
    mid = (len(nums) + 1) // 2
    left, right = nums[:mid][::-1], nums[mid:][::-1]
    nums[:] = [left[i // 2] if i % 2 == 0 else right[i // 2] for i in range(len(nums))]
    return nums

print(wiggle_sort([1, 5, 1, 1, 6, 4]))
`,
    nextUrl: /\/workspace/,
    cursorAfter: "499",
  }
];

test("declares the contiguous learn-route family", () => {
  for (const step of FAMILY) {
    expect(step.id).toMatch(/^py-(?:493|494|495|496|497|498)-/);
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

test.describe("micro-steps 493–498 · sorting III", () => {
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
      if (nextMicro <= 552) {
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
