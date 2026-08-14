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
    micro: 245,
    id: "py-245-remove-nth",
    title: "DSA Remove Nth Node",
    solution: `class ListNode:
    def __init__(self, data=0, next=None):
        self.data = data
        self.next = next

def to_list(head):
    out = []
    while head:
        out.append(head.data)
        head = head.next
    return out

def remove_nth_from_end(head, n):
    dummy = ListNode(0, head)
    fast = slow = dummy
    for _ in range(n):
        fast = fast.next
    while fast.next:
        fast = fast.next
        slow = slow.next
    slow.next = slow.next.next
    return dummy.next

head = ListNode(1, ListNode(2, ListNode(3, ListNode(4, ListNode(5)))))
print(to_list(remove_nth_from_end(head, 2)))
`,
    nextUrl: /\/learn\/py-246-reorder-list/,
    cursorAfter: "246",
  },
  {
    micro: 246,
    id: "py-246-reorder-list",
    title: "DSA Reorder List",
    solution: `class ListNode:
    def __init__(self, data=0, next=None):
        self.data = data
        self.next = next

def to_list(head):
    out = []
    while head:
        out.append(head.data)
        head = head.next
    return out

def reorder_list(head):
    if not head or not head.next:
        return
    slow = fast = head
    while fast.next and fast.next.next:
        slow = slow.next
        fast = fast.next.next
    second = slow.next
    slow.next = None
    prev = None
    while second:
        nxt = second.next
        second.next = prev
        prev = second
        second = nxt
    first, second = head, prev
    while second:
        t1, t2 = first.next, second.next
        first.next = second
        second.next = t1
        first, second = t1, t2

head = ListNode(1, ListNode(2, ListNode(3, ListNode(4, ListNode(5)))))
reorder_list(head)
print(to_list(head))
`,
    nextUrl: /\/learn\/py-247-add-two-lists/,
    cursorAfter: "247",
  },
  {
    micro: 247,
    id: "py-247-add-two-lists",
    title: "DSA Add Two Lists",
    solution: `class ListNode:
    def __init__(self, data=0, next=None):
        self.data = data
        self.next = next

def to_list(head):
    out = []
    while head:
        out.append(head.data)
        head = head.next
    return out

def add_two_numbers(l1, l2):
    dummy = ListNode(0)
    cur = dummy
    carry = 0
    while l1 or l2 or carry:
        total = carry
        if l1:
            total += l1.data
            l1 = l1.next
        if l2:
            total += l2.data
            l2 = l2.next
        cur.next = ListNode(total % 10)
        cur = cur.next
        carry = total // 10
    return dummy.next

l1 = ListNode(2, ListNode(4, ListNode(3)))
l2 = ListNode(5, ListNode(6, ListNode(4)))
print(to_list(add_two_numbers(l1, l2)))
`,
    nextUrl: /\/learn\/py-248-swap-pairs/,
    cursorAfter: "248",
  },
  {
    micro: 248,
    id: "py-248-swap-pairs",
    title: "DSA Swap Pairs",
    solution: `class ListNode:
    def __init__(self, data=0, next=None):
        self.data = data
        self.next = next

def to_list(head):
    out = []
    while head:
        out.append(head.data)
        head = head.next
    return out

def swap_pairs(head):
    dummy = ListNode(0, head)
    prev = dummy
    while prev.next and prev.next.next:
        a = prev.next
        b = a.next
        prev.next, a.next, b.next = b, b.next, a
        prev = a
    return dummy.next

head = ListNode(1, ListNode(2, ListNode(3, ListNode(4))))
print(to_list(swap_pairs(head)))
`,
    nextUrl: /\/learn\/py-249-rotate-list/,
    cursorAfter: "249",
  },
  {
    micro: 249,
    id: "py-249-rotate-list",
    title: "DSA Rotate List",
    solution: `class ListNode:
    def __init__(self, data=0, next=None):
        self.data = data
        self.next = next

def to_list(head):
    out = []
    while head:
        out.append(head.data)
        head = head.next
    return out

def rotate_right(head, k):
    if not head or not head.next or k == 0:
        return head
    n = 1
    tail = head
    while tail.next:
        tail = tail.next
        n += 1
    k %= n
    if k == 0:
        return head
    tail.next = head
    new_tail = head
    for _ in range(n - k - 1):
        new_tail = new_tail.next
    new_head = new_tail.next
    new_tail.next = None
    return new_head

head = ListNode(1, ListNode(2, ListNode(3, ListNode(4, ListNode(5)))))
print(to_list(rotate_right(head, 2)))
`,
    nextUrl: /\/learn\/py-250-palindrome-list/,
    cursorAfter: "250",
  },
  {
    micro: 250,
    id: "py-250-palindrome-list",
    title: "DSA Palindrome List",
    solution: `class ListNode:
    def __init__(self, data=0, next=None):
        self.data = data
        self.next = next

def is_palindrome_list(head):
    slow = fast = head
    while fast and fast.next:
        slow = slow.next
        fast = fast.next.next
    prev = None
    while slow:
        nxt = slow.next
        slow.next = prev
        prev = slow
        slow = nxt
    while prev:
        if prev.data != head.data:
            return False
        prev = prev.next
        head = head.next
    return True

head = ListNode(1, ListNode(2, ListNode(2, ListNode(1))))
print(is_palindrome_list(head))
`,
    nextUrl: /\/learn\/py-251-copy-random/,
    cursorAfter: "251",
  },
];

test("declares the contiguous learn-route family", () => {
  for (const step of FAMILY) {
    expect(step.id).toMatch(/^py-24[5-9]-|^py-250-/);
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

test.describe("micro-steps 245–250 · listas enlazadas", () => {
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
      if (nextMicro <= 486) {
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
