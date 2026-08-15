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
    micro: 601,
    id: "py-601-subarray-sum-k",
    title: "DSA Prefix Sum IV · Subarray Sum K",
    solution: `def subarray_sum(nums, k):
    frequencies = {0: 1}
    prefix = total = 0
    for value in nums:
        prefix += value
        total += frequencies.get(prefix - k, 0)
        frequencies[prefix] = frequencies.get(prefix, 0) + 1
    return total

print(subarray_sum([1, 1, 1], 2))
`,
    nextUrl: /\/learn\/py-602-pivot-index/,
    cursorAfter: "602",
  },
  {
    micro: 602,
    id: "py-602-pivot-index",
    title: "DSA Prefix Sum IV · Pivot Index",
    solution: `def pivot_index(nums):
    total = sum(nums)
    left = 0
    for index, value in enumerate(nums):
        if left == total - left - value:
            return index
        left += value
    return -1

print(pivot_index([1, 7, 3, 6, 5, 6]))
`,
    nextUrl: /\/learn\/py-603-range-sum-query/,
    cursorAfter: "603",
  },
  {
    micro: 603,
    id: "py-603-range-sum-query",
    title: "DSA Prefix Sum IV · Range Query",
    solution: `class NumArray:
    def __init__(self, nums):
        self.prefix = [0]
        for value in nums:
            self.prefix.append(self.prefix[-1] + value)

    def sum_range(self, left, right):
        return self.prefix[right + 1] - self.prefix[left]

print(NumArray([-2, 0, 3, -5, 2, -1]).sum_range(0, 2))
`,
    nextUrl: /\/learn\/py-604-continuous-subarray/,
    cursorAfter: "604",
  },
  {
    micro: 604,
    id: "py-604-continuous-subarray",
    title: "DSA Prefix Sum IV · Multiple of K",
    solution: `def check_subarray_sum(nums, k):
    first = {0: -1}
    prefix = 0
    for index, value in enumerate(nums):
        prefix += value
        remainder = prefix if k == 0 else prefix % k
        if remainder in first:
            if index - first[remainder] >= 2:
                return True
        else:
            first[remainder] = index
    return False

print(check_subarray_sum([23, 2, 4, 6, 7], 6))
`,
    nextUrl: /\/learn\/py-605-subarrays-divisible-k/,
    cursorAfter: "605",
  },
  {
    micro: 605,
    id: "py-605-subarrays-divisible-k",
    title: "DSA Prefix Sum IV · Divisible Subarrays",
    solution: `def subarrays_div_by_k(nums, k):
    frequencies = {0: 1}
    prefix = total = 0
    for value in nums:
        prefix = (prefix + value) % k
        total += frequencies.get(prefix, 0)
        frequencies[prefix] = frequencies.get(prefix, 0) + 1
    return total

print(subarrays_div_by_k([4, 5, 0, -2, -3, 1], 5))
`,
    nextUrl: /\/learn\/py-606-max-subarray-len-k/,
    cursorAfter: "606",
  },
  {
    micro: 606,
    id: "py-606-max-subarray-len-k",
    title: "DSA Prefix Sum IV · Longest Sum K",
    solution: `def max_subarray_len(nums, k):
    first = {0: -1}
    prefix = longest = 0
    for index, value in enumerate(nums):
        prefix += value
        if prefix - k in first:
            longest = max(longest, index - first[prefix - k])
        first.setdefault(prefix, index)
    return longest

print(max_subarray_len([1, -1, 5, -2, 3], 3))
`,
    nextUrl: /\/learn\/py-607-range-addition/,
    cursorAfter: "607",
  },
];

test("declares the contiguous learn-route family", () => {
  for (const step of FAMILY) {
    expect(step.id).toMatch(/^py-(?:601|602|603|604|605|606)-/);
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

test.describe("micro-steps 601–606 · prefix sum IV", () => {
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
