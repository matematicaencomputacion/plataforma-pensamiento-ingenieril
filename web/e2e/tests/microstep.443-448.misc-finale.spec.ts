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
    micro: 443,
    id: "py-443-valid-paren",
    title: "DSA Valid Paren",
    solution: `def is_valid(s):
    m = {")": "(", "]": "[", "}": "{"}
    st = []
    for ch in s:
        if ch in m:
            if not st or st[-1] != m[ch]:
                return False
            st.pop()
        else:
            st.append(ch)
    return not st

print(is_valid("()[]{}"))
`,
    nextUrl: /\/learn\/py-444-min-stack-ops/,
    cursorAfter: "444",
  },
  {
    micro: 444,
    id: "py-444-min-stack-ops",
    title: "DSA Eval RPN",
    solution: `def eval_rpn(tokens):
    st = []
    for t in tokens:
        if t in "+-*/":
            b, a = st.pop(), st.pop()
            if t == "+":
                st.append(a + b)
            elif t == "-":
                st.append(a - b)
            elif t == "*":
                st.append(a * b)
            else:
                st.append(int(a / b))
        else:
            st.append(int(t))
    return st[0]

print(eval_rpn(["2", "1", "+", "3", "*"]))
`,
    nextUrl: /\/learn\/py-445-majority-elem/,
    cursorAfter: "445",
  },
  {
    micro: 445,
    id: "py-445-majority-elem",
    title: "DSA Majority Elem",
    solution: `def majority_element(nums):
    cand = None
    cnt = 0
    for x in nums:
        if cnt == 0:
            cand = x
        cnt += 1 if x == cand else -1
    return cand

print(majority_element([3, 2, 3]))
`,
    nextUrl: /\/learn\/py-446-pascal-row/,
    cursorAfter: "446",
  },
  {
    micro: 446,
    id: "py-446-pascal-row",
    title: "DSA Pascal Row",
    solution: `def get_row(row_index):
    row = [1]
    for _ in range(row_index):
        row = [1] + [row[i] + row[i + 1] for i in range(len(row) - 1)] + [1]
    return row

print(get_row(3))
`,
    nextUrl: /\/learn\/py-447-move-zeroes/,
    cursorAfter: "447",
  },
  {
    micro: 447,
    id: "py-447-move-zeroes",
    title: "DSA Move Zeroes",
    solution: `def move_zeroes(nums):
    w = 0
    for x in nums:
        if x != 0:
            nums[w] = x
            w += 1
    for i in range(w, len(nums)):
        nums[i] = 0
    return nums

print(move_zeroes([0, 1, 0, 3, 12]))
`,
    nextUrl: /\/learn\/py-448-plus-one/,
    cursorAfter: "448",
  },
  {
    micro: 448,
    id: "py-448-plus-one",
    title: "DSA Plus One",
    solution: `def plus_one(digits):
    for i in range(len(digits) - 1, -1, -1):
        if digits[i] < 9:
            digits[i] += 1
            return digits
        digits[i] = 0
    return [1] + digits

print(plus_one([1, 2, 3]))
`,
    nextUrl: /\/learn\/py-449-roman-to-int/,
    cursorAfter: "449",
  }
];

test("declares the contiguous learn-route family", () => {
  for (const step of FAMILY) {
    expect(step.id).toMatch(/^py-(?:443|444|445|446|447|448)-/);
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

test.describe("micro-steps 443–448 · misc finale", () => {
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
      if (nextMicro <= 504) {
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
