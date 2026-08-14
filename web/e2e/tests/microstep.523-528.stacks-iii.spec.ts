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
    micro: 523,
    id: "py-523-daily-temps",
    title: "DSA Daily Temps",
    solution: `def daily_temperatures(temps):
    ans = [0] * len(temps)
    st = []
    for i, t in enumerate(temps):
        while st and temps[st[-1]] < t:
            j = st.pop()
            ans[j] = i - j
        st.append(i)
    return ans

print(daily_temperatures([73, 74, 75, 71, 69, 72, 76, 73]))
`,
    nextUrl: /\/learn\/py-524-next-greater/,
    cursorAfter: "524",
  },
  {
    micro: 524,
    id: "py-524-next-greater",
    title: "DSA Next Greater",
    solution: `def next_greater_elements(nums):
    n = len(nums)
    ans = [-1] * n
    st = []
    for i in range(2 * n):
        x = nums[i % n]
        while st and nums[st[-1]] < x:
            ans[st.pop()] = x
        if i < n:
            st.append(i)
    return ans

print(next_greater_elements([1, 2, 1]))
`,
    nextUrl: /\/learn\/py-525-eval-rpn/,
    cursorAfter: "525",
  },
  {
    micro: 525,
    id: "py-525-eval-rpn",
    title: "DSA Eval RPN",
    solution: `def eval_rpn(tokens):
    st = []
    for t in tokens:
        if t in "+-*/":
            b, a = st.pop(), st.pop()
            if t == "+": st.append(a + b)
            elif t == "-": st.append(a - b)
            elif t == "*": st.append(a * b)
            else: st.append(int(a / b))
        else:
            st.append(int(t))
    return st[0]

print(eval_rpn(["2", "1", "+", "3", "*"]))
`,
    nextUrl: /\/learn\/py-526-decode-string/,
    cursorAfter: "526",
  },
  {
    micro: 526,
    id: "py-526-decode-string",
    title: "DSA Decode String",
    solution: `def decode_string(s):
    st = []
    cur, num = "", 0
    for ch in s:
        if ch.isdigit():
            num = num * 10 + int(ch)
        elif ch == "[":
            st.append((cur, num))
            cur, num = "", 0
        elif ch == "]":
            prev, k = st.pop()
            cur = prev + cur * k
        else:
            cur += ch
    return cur

print(decode_string("3[a]2[bc]"))
`,
    nextUrl: /\/learn\/py-527-asteroid/,
    cursorAfter: "527",
  },
  {
    micro: 527,
    id: "py-527-asteroid",
    title: "DSA Asteroid",
    solution: `def asteroid_collision(asteroids):
    st = []
    for a in asteroids:
        while st and a < 0 < st[-1]:
            if st[-1] < -a:
                st.pop(); continue
            elif st[-1] == -a:
                st.pop()
            break
        else:
            st.append(a)
    return st

print(asteroid_collision([5, 10, -5]))
`,
    nextUrl: /\/learn\/py-528-remove-k-digits/,
    cursorAfter: "528",
  },
  {
    micro: 528,
    id: "py-528-remove-k-digits",
    title: "DSA Remove K Digits",
    solution: `def remove_k_digits(num, k):
    st = []
    for ch in num:
        while k and st and st[-1] > ch:
            st.pop(); k -= 1
        st.append(ch)
    if k:
        st = st[:-k]
    return "".join(st).lstrip("0") or "0"

print(remove_k_digits("1432219", 3))
`,
    nextUrl: /\/workspace/,
    cursorAfter: "529",
  }
];

test("declares the contiguous learn-route family", () => {
  for (const step of FAMILY) {
    expect(step.id).toMatch(/^py-(?:523|524|525|526|527|528)-/);
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

test.describe("micro-steps 523–528 · stacks III", () => {
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
      if (nextMicro <= 540) {
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
