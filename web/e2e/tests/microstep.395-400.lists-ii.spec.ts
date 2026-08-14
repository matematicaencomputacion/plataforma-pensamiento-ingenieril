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
    micro: 395,
    id: "py-395-reverse-list",
    title: "DSA Reverse List",
    solution: `def reverse_list(head):
    return head[::-1]

print(reverse_list([1, 2, 3, 4, 5]))
`,
    nextUrl: /\/learn\/py-396-merge-two-lists/,
    cursorAfter: "396",
  },
  {
    micro: 396,
    id: "py-396-merge-two-lists",
    title: "DSA Merge Two Lists",
    solution: `import heapq

def merge_two_lists(l1, l2):
    return list(heapq.merge(l1, l2))

print(merge_two_lists([1, 2, 4], [1, 3, 4]))
`,
    nextUrl: /\/learn\/py-397-has-cycle/,
    cursorAfter: "397",
  },
  {
    micro: 397,
    id: "py-397-has-cycle",
    title: "DSA Has Cycle",
    solution: `def has_cycle(vals, pos):
    n = len(vals)
    nxt = list(range(1, n)) + [-1]
    if pos >= 0:
        nxt[-1] = pos
    slow = fast = 0
    while fast != -1 and nxt[fast] != -1:
        slow = nxt[slow]
        fast = nxt[nxt[fast]]
        if slow == fast:
            return True
    return False

print(has_cycle([3, 2, 0, -4], 1))
`,
    nextUrl: /\/learn\/py-398-remove-nth/,
    cursorAfter: "398",
  },
  {
    micro: 398,
    id: "py-398-remove-nth",
    title: "DSA Remove Nth",
    solution: `def remove_nth_from_end(head, n):
    del head[-n]
    return head

print(remove_nth_from_end([1, 2, 3, 4, 5], 2))
`,
    nextUrl: /\/learn\/py-399-palindrome-list/,
    cursorAfter: "399",
  },
  {
    micro: 399,
    id: "py-399-palindrome-list",
    title: "DSA Palindrome List",
    solution: `def is_palindrome(head):
    return head == head[::-1]

print(is_palindrome([1, 2, 2, 1]))
`,
    nextUrl: /\/learn\/py-400-add-two-numbers/,
    cursorAfter: "400",
  },
  {
    micro: 400,
    id: "py-400-add-two-numbers",
    title: "DSA Add Two Numbers",
    solution: `def add_two_numbers(l1, l2):
    i = carry = 0
    out = []
    while i < len(l1) or i < len(l2) or carry:
        s = carry + (l1[i] if i < len(l1) else 0) + (l2[i] if i < len(l2) else 0)
        out.append(s % 10)
        carry = s // 10
        i += 1
    return out

print(add_two_numbers([2, 4, 3], [5, 6, 4]))
`,
    nextUrl: /\/learn\/py-401-sort-colors/,
    cursorAfter: "401",
  }
];

test("declares the contiguous learn-route family", () => {
  for (const step of FAMILY) {
    expect(step.id).toMatch(/^py-(?:395|396|397|398|399|400)-/);
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

test.describe("micro-steps 395–400 · linked lists II", () => {
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
      if (nextMicro <= 570) {
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
