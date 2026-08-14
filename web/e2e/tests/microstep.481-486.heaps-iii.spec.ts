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
    micro: 481,
    id: "py-481-last-stone",
    title: "DSA Last Stone",
    solution: `def last_stone_weight(stones):
    import heapq
    h = [-s for s in stones]
    heapq.heapify(h)
    while len(h) > 1:
        a = -heapq.heappop(h)
        b = -heapq.heappop(h)
        if a != b:
            heapq.heappush(h, -(a - b))
    return -h[0] if h else 0

print(last_stone_weight([2, 7, 4, 1, 8, 1]))
`,
    nextUrl: /\/learn\/py-482-kth-largest/,
    cursorAfter: "482",
  },
  {
    micro: 482,
    id: "py-482-kth-largest",
    title: "DSA Kth Largest",
    solution: `def find_kth_largest(nums, k):
    import heapq
    return heapq.nlargest(k, nums)[-1]

print(find_kth_largest([3, 2, 1, 5, 6, 4], 2))
`,
    nextUrl: /\/learn\/py-483-top-k-freq/,
    cursorAfter: "483",
  },
  {
    micro: 483,
    id: "py-483-top-k-freq",
    title: "DSA Top K Freq",
    solution: `def top_k_frequent(nums, k):
    from collections import Counter
    import heapq
    return [x for x, _ in Counter(nums).most_common(k)]

print(top_k_frequent([1, 1, 1, 2, 2, 3], 2))
`,
    nextUrl: /\/learn\/py-484-k-closest/,
    cursorAfter: "484",
  },
  {
    micro: 484,
    id: "py-484-k-closest",
    title: "DSA K Closest",
    solution: `def k_closest(points, k):
    import heapq
    return heapq.nsmallest(k, points, key=lambda p: p[0] ** 2 + p[1] ** 2)

print(k_closest([[1, 3], [-2, 2]], 1))
`,
    nextUrl: /\/learn\/py-485-ugly-number-ii/,
    cursorAfter: "485",
  },
  {
    micro: 485,
    id: "py-485-ugly-number-ii",
    title: "DSA Ugly Number II",
    solution: `def nth_ugly_number(n):
    import heapq
    h = [1]
    seen = {1}
    x = 1
    for _ in range(n):
        x = heapq.heappop(h)
        for f in (2, 3, 5):
            y = x * f
            if y not in seen:
                seen.add(y)
                heapq.heappush(h, y)
    return x

print(nth_ugly_number(10))
`,
    nextUrl: /\/learn\/py-486-reorg-string/,
    cursorAfter: "486",
  },
  {
    micro: 486,
    id: "py-486-reorg-string",
    title: "DSA Reorg String",
    solution: `def reorganize_string(s):
    from collections import Counter
    import heapq
    h = [(-c, ch) for ch, c in Counter(s).items()]
    heapq.heapify(h)
    out = []
    prev = (0, "")
    while h:
        c, ch = heapq.heappop(h)
        out.append(ch)
        if prev[0] < 0:
            heapq.heappush(h, prev)
        prev = (c + 1, ch)
    res = "".join(out)
    return res if len(res) == len(s) else ""

print(reorganize_string("aab"))
`,
    nextUrl: /\/workspace/,
    cursorAfter: "487",
  }
];

test("declares the contiguous learn-route family", () => {
  for (const step of FAMILY) {
    expect(step.id).toMatch(/^py-(?:481|482|483|484|485|486)-/);
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

test.describe("micro-steps 481–486 · heaps III", () => {
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
      if (nextMicro <= 564) {
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
