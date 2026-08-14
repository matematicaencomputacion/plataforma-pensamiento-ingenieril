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
    micro: 347,
    id: "py-347-next-greater",
    title: "DSA Next Greater",
    solution: `def next_greater_element(nums1, nums2):
    stack = []
    nxt = {}
    for x in nums2:
        while stack and stack[-1] < x:
            nxt[stack.pop()] = x
        stack.append(x)
    return [nxt.get(x, -1) for x in nums1]

print(next_greater_element([4, 1, 2], [1, 3, 4, 2]))
`,
    nextUrl: /\/learn\/py-348-next-greater-ii/,
    cursorAfter: "348",
  },
  {
    micro: 348,
    id: "py-348-next-greater-ii",
    title: "DSA Next Greater II",
    solution: `def next_greater_elements(nums):
    n = len(nums)
    out = [-1] * n
    stack = []
    for i in range(2 * n):
        x = nums[i % n]
        while stack and nums[stack[-1]] < x:
            out[stack.pop()] = x
        if i < n:
            stack.append(i)
    return out

print(next_greater_elements([1, 2, 1]))
`,
    nextUrl: /\/learn\/py-349-daily-temps/,
    cursorAfter: "349",
  },
  {
    micro: 349,
    id: "py-349-daily-temps",
    title: "DSA Daily Temps",
    solution: `def daily_temperatures(temperatures):
    n = len(temperatures)
    out = [0] * n
    stack = []
    for i, t in enumerate(temperatures):
        while stack and temperatures[stack[-1]] < t:
            j = stack.pop()
            out[j] = i - j
        stack.append(i)
    return out

print(daily_temperatures([73, 74, 75, 71, 69, 72, 76, 73]))
`,
    nextUrl: /\/learn\/py-350-online-stock/,
    cursorAfter: "350",
  },
  {
    micro: 350,
    id: "py-350-online-stock",
    title: "DSA Online Stock",
    solution: `class StockSpanner:
    def __init__(self):
        self.stack = []

    def next(self, price):
        span = 1
        while self.stack and self.stack[-1][0] <= price:
            span += self.stack.pop()[1]
        self.stack.append((price, span))
        return span

s = StockSpanner()
print([s.next(p) for p in [100, 80, 60, 70, 60, 75, 85]])
`,
    nextUrl: /\/learn\/py-351-sum-subarray-mins/,
    cursorAfter: "351",
  },
  {
    micro: 351,
    id: "py-351-sum-subarray-mins",
    title: "DSA Sum Subarray Mins",
    solution: `def sum_subarray_mins(arr):
    MOD = 10**9 + 7
    n = len(arr)
    left = [0] * n
    right = [0] * n
    stack = []
    for i, x in enumerate(arr):
        while stack and arr[stack[-1]] > x:
            stack.pop()
        left[i] = i - stack[-1] if stack else i + 1
        stack.append(i)
    stack = []
    for i in range(n - 1, -1, -1):
        while stack and arr[stack[-1]] >= arr[i]:
            stack.pop()
        right[i] = stack[-1] - i if stack else n - i
        stack.append(i)
    return sum(a * l * r for a, l, r in zip(arr, left, right)) % MOD

print(sum_subarray_mins([3, 1, 2, 4]))
`,
    nextUrl: /\/learn\/py-352-remove-k-dupes/,
    cursorAfter: "352",
  },
  {
    micro: 352,
    id: "py-352-remove-k-dupes",
    title: "DSA Remove K Dupes",
    solution: `def remove_duplicates(s, k):
    stack = []
    for ch in s:
        if stack and stack[-1][0] == ch:
            stack[-1][1] += 1
            if stack[-1][1] == k:
                stack.pop()
        else:
            stack.append([ch, 1])
    return "".join(ch * c for ch, c in stack)

print(remove_duplicates("deeedbbcccbdaa", 3))
`,
    nextUrl: /\/learn\/py-353-num-provinces/,
    cursorAfter: "353",
  },
];

test("declares the contiguous learn-route family", () => {
  for (const step of FAMILY) {
    expect(step.id).toMatch(/^py-34[7-9]-|^py-35[0-2]-/);
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

test.describe("micro-steps 347–352 · monotonic stack II", () => {
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
      if (nextMicro <= 534) {
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
