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
    micro: 679,
    id: "py-679-reverse-between",
    title: "DSA Lists IV · Reverse Between",
    solution: "class ListNode:\n    def __init__(self, val=0, next=None):\n        self.val = val\n        self.next = next\n\ndef to_list(head):\n    out = []\n    while head:\n        out.append(head.val)\n        head = head.next\n    return out\n\ndef from_list(vals):\n    dummy = ListNode(0)\n    cur = dummy\n    for v in vals:\n        cur.next = ListNode(v)\n        cur = cur.next\n    return dummy.next\n\ndef reverse_between(head, left, right):\n    dummy = ListNode(0, head)\n    pred = dummy\n    for _ in range(left - 1):\n        pred = pred.next\n    cur = pred.next\n    for _ in range(right - left):\n        nxt = cur.next\n        cur.next = nxt.next\n        nxt.next = pred.next\n        pred.next = nxt\n    return dummy.next\n\nprint(to_list(reverse_between(from_list([1, 2, 3, 4, 5]), 2, 4)))\n",
    nextUrl: /\/learn\/py-680-odd-even-list/,
    cursorAfter: "680",
  },
  {
    micro: 680,
    id: "py-680-odd-even-list",
    title: "DSA Lists IV · Odd Even",
    solution: "class ListNode:\n    def __init__(self, val=0, next=None):\n        self.val = val\n        self.next = next\n\ndef to_list(head):\n    out = []\n    while head:\n        out.append(head.val)\n        head = head.next\n    return out\n\ndef from_list(vals):\n    dummy = ListNode(0)\n    cur = dummy\n    for v in vals:\n        cur.next = ListNode(v)\n        cur = cur.next\n    return dummy.next\n\ndef odd_even_list(head):\n    if not head or not head.next:\n        return head\n    odd, even, even_head = head, head.next, head.next\n    while even and even.next:\n        odd.next = even.next\n        odd = odd.next\n        even.next = odd.next\n        even = even.next\n    odd.next = even_head\n    return head\n\nprint(to_list(odd_even_list(from_list([1, 2, 3, 4, 5]))))\n",
    nextUrl: /\/learn\/py-681-partition-list/,
    cursorAfter: "681",
  },
  {
    micro: 681,
    id: "py-681-partition-list",
    title: "DSA Lists IV · Partition",
    solution: "class ListNode:\n    def __init__(self, val=0, next=None):\n        self.val = val\n        self.next = next\n\ndef to_list(head):\n    out = []\n    while head:\n        out.append(head.val)\n        head = head.next\n    return out\n\ndef from_list(vals):\n    dummy = ListNode(0)\n    cur = dummy\n    for v in vals:\n        cur.next = ListNode(v)\n        cur = cur.next\n    return dummy.next\n\ndef partition(head, x):\n    before = before_h = ListNode(0)\n    after = after_h = ListNode(0)\n    while head:\n        if head.val < x:\n            before.next = head\n            before = before.next\n        else:\n            after.next = head\n            after = after.next\n        head = head.next\n    after.next = None\n    before.next = after_h.next\n    return before_h.next\n\nprint(to_list(partition(from_list([1, 4, 3, 2, 5, 2]), 3)))\n",
    nextUrl: /\/learn\/py-682-split-list-parts/,
    cursorAfter: "682",
  },
  {
    micro: 682,
    id: "py-682-split-list-parts",
    title: "DSA Lists IV · Split Parts",
    solution: "class ListNode:\n    def __init__(self, val=0, next=None):\n        self.val = val\n        self.next = next\n\ndef to_list(head):\n    out = []\n    while head:\n        out.append(head.val)\n        head = head.next\n    return out\n\ndef from_list(vals):\n    dummy = ListNode(0)\n    cur = dummy\n    for v in vals:\n        cur.next = ListNode(v)\n        cur = cur.next\n    return dummy.next\n\ndef split_list_to_parts(head, k):\n    n, cur = 0, head\n    while cur:\n        n += 1\n        cur = cur.next\n    q, r = divmod(n, k)\n    parts = []\n    cur = head\n    for i in range(k):\n        parts.append(cur)\n        size = q + (1 if i < r else 0)\n        for _ in range(max(0, size - 1)):\n            if cur:\n                cur = cur.next\n        if cur:\n            nxt = cur.next\n            cur.next = None\n            cur = nxt\n    return parts\n\nprint([to_list(p) for p in split_list_to_parts(from_list([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 3)])\n",
    nextUrl: /\/learn\/py-683-add-two-numbers-ii/,
    cursorAfter: "683",
  },
  {
    micro: 683,
    id: "py-683-add-two-numbers-ii",
    title: "DSA Lists IV · Add Two II",
    solution: "class ListNode:\n    def __init__(self, val=0, next=None):\n        self.val = val\n        self.next = next\n\ndef to_list(head):\n    out = []\n    while head:\n        out.append(head.val)\n        head = head.next\n    return out\n\ndef from_list(vals):\n    dummy = ListNode(0)\n    cur = dummy\n    for v in vals:\n        cur.next = ListNode(v)\n        cur = cur.next\n    return dummy.next\n\ndef add_two_numbers(l1, l2):\n    s1, s2 = [], []\n    while l1:\n        s1.append(l1.val)\n        l1 = l1.next\n    while l2:\n        s2.append(l2.val)\n        l2 = l2.next\n    carry, dummy = 0, None\n    while s1 or s2 or carry:\n        carry += (s1.pop() if s1 else 0) + (s2.pop() if s2 else 0)\n        node = ListNode(carry % 10)\n        node.next = dummy\n        dummy = node\n        carry //= 10\n    return dummy\n\nprint(to_list(add_two_numbers(from_list([7, 2, 4, 3]), from_list([5, 6, 4]))))\n",
    nextUrl: /\/learn\/py-684-remove-zero-sum/,
    cursorAfter: "684",
  },
  {
    micro: 684,
    id: "py-684-remove-zero-sum",
    title: "DSA Lists IV · Zero Sum",
    solution: "class ListNode:\n    def __init__(self, val=0, next=None):\n        self.val = val\n        self.next = next\n\ndef to_list(head):\n    out = []\n    while head:\n        out.append(head.val)\n        head = head.next\n    return out\n\ndef from_list(vals):\n    dummy = ListNode(0)\n    cur = dummy\n    for v in vals:\n        cur.next = ListNode(v)\n        cur = cur.next\n    return dummy.next\n\ndef remove_zero_sum_sublists(head):\n    dummy = ListNode(0, head)\n    seen = {}\n    s, cur = 0, dummy\n    while cur:\n        s += cur.val\n        seen[s] = cur\n        cur = cur.next\n    s, cur = 0, dummy\n    while cur:\n        s += cur.val\n        cur.next = seen[s].next\n        cur = cur.next\n    return dummy.next\n\nprint(to_list(remove_zero_sum_sublists(from_list([1, 2, -3, 3, 1]))))\n",
    nextUrl: /\/workspace/,
    cursorAfter: "685",
  },
];

test("declares the contiguous learn-route family", () => {
  for (const step of FAMILY) {
    expect(step.id).toMatch(/^py-(?:679|680|681|682|683|684)-/);
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

test.describe("micro-steps 679–684 · lists iv", () => {
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
      if (nextMicro <= 684) {
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

      if (step.micro < 684) {
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
