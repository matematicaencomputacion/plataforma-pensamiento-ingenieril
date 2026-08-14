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
    micro: 451,
    id: "py-451-build-array-perm",
    title: "DSA Build Array",
    solution: `def build_array(nums):
    return [nums[x] for x in nums]

print(build_array([0, 2, 1, 5, 3, 4]))
`,
    nextUrl: /\/learn\/py-452-concat-array/,
    cursorAfter: "452",
  },
  {
    micro: 452,
    id: "py-452-concat-array",
    title: "DSA Concat Array",
    solution: `def get_concatenation(nums):
    return nums + nums

print(get_concatenation([1, 2, 1]))
`,
    nextUrl: /\/learn\/py-453-shuffle-array/,
    cursorAfter: "453",
  },
  {
    micro: 453,
    id: "py-453-shuffle-array",
    title: "DSA Shuffle Array",
    solution: `def shuffle(nums, n):
    out = []
    for i in range(n):
        out.append(nums[i])
        out.append(nums[i + n])
    return out

print(shuffle([2, 5, 1, 3, 4, 7], 3))
`,
    nextUrl: /\/learn\/py-454-kids-candies/,
    cursorAfter: "454",
  },
  {
    micro: 454,
    id: "py-454-kids-candies",
    title: "DSA Kids Candies",
    solution: `def kids_with_candies(candies, extra):
    m = max(candies)
    return [c + extra >= m for c in candies]

print(kids_with_candies([2, 3, 5, 1, 3], 3))
`,
    nextUrl: /\/learn\/py-455-good-pairs/,
    cursorAfter: "455",
  },
  {
    micro: 455,
    id: "py-455-good-pairs",
    title: "DSA Good Pairs",
    solution: `def num_identical_pairs(nums):
    from collections import Counter
    return sum(c * (c - 1) // 2 for c in Counter(nums).values())

print(num_identical_pairs([1, 2, 3, 1, 1, 3]))
`,
    nextUrl: /\/learn\/py-456-smaller-counts/,
    cursorAfter: "456",
  },
  {
    micro: 456,
    id: "py-456-smaller-counts",
    title: "DSA Smaller Counts",
    solution: `def smaller_numbers_than_current(nums):
    return [sum(1 for y in nums if y < x) for x in nums]

print(smaller_numbers_than_current([8, 1, 2, 2, 3]))
`,
    nextUrl: /\/learn\/py-457-defang-ip/,
    cursorAfter: "457",
  }
];

test("declares the contiguous learn-route family", () => {
  for (const step of FAMILY) {
    expect(step.id).toMatch(/^py-(?:451|452|453|454|455|456)-/);
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

test.describe("micro-steps 451–456 · arrays III", () => {
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
