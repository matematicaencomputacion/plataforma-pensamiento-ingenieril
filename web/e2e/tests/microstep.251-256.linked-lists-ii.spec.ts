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
    micro: 251,
    id: "py-251-copy-random",
    title: "DSA Copy Random List",
    solution: `class ListNode:
    def __init__(self, data=0, next=None, random=None):
        self.data = data
        self.next = next
        self.random = random

def walk(head):
    vals, rands = [], []
    while head:
        vals.append(head.data)
        rands.append(head.random.data if head.random else None)
        head = head.next
    return vals, rands

def copy_random_list(head):
    if not head:
        return None
    mapping = {}
    cur = head
    while cur:
        mapping[cur] = ListNode(cur.data)
        cur = cur.next
    cur = head
    while cur:
        mapping[cur].next = mapping.get(cur.next)
        mapping[cur].random = mapping.get(cur.random)
        cur = cur.next
    return mapping[head]

a = ListNode(1)
b = ListNode(2)
a.next = b
a.random = b
b.random = a
copy = copy_random_list(a)
print(walk(copy)[0])
`,
    nextUrl: /\/learn\/py-252-sort-list/,
    cursorAfter: "252",
  },
  {
    micro: 252,
    id: "py-252-sort-list",
    title: "DSA Sort List",
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

def merge(a, b):
    dummy = ListNode(0)
    cur = dummy
    while a and b:
        if a.data <= b.data:
            cur.next = a
            a = a.next
        else:
            cur.next = b
            b = b.next
        cur = cur.next
    cur.next = a or b
    return dummy.next

def sort_list(head):
    if not head or not head.next:
        return head
    slow, fast = head, head.next
    while fast and fast.next:
        slow = slow.next
        fast = fast.next.next
    mid = slow.next
    slow.next = None
    return merge(sort_list(head), sort_list(mid))

head = ListNode(4, ListNode(2, ListNode(1, ListNode(3))))
print(to_list(sort_list(head)))
`,
    nextUrl: /\/learn\/py-253-merge-two-lists/,
    cursorAfter: "253",
  },
  {
    micro: 253,
    id: "py-253-merge-two-lists",
    title: "DSA Merge Two Lists",
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

def merge_two_lists(l1, l2):
    dummy = ListNode(0)
    cur = dummy
    while l1 and l2:
        if l1.data <= l2.data:
            cur.next = l1
            l1 = l1.next
        else:
            cur.next = l2
            l2 = l2.next
        cur = cur.next
    cur.next = l1 or l2
    return dummy.next

l1 = ListNode(1, ListNode(2, ListNode(4)))
l2 = ListNode(1, ListNode(3, ListNode(4)))
print(to_list(merge_two_lists(l1, l2)))
`,
    nextUrl: /\/learn\/py-254-intersection/,
    cursorAfter: "254",
  },
  {
    micro: 254,
    id: "py-254-intersection",
    title: "DSA List Intersection",
    solution: `class ListNode:
    def __init__(self, data=0, next=None):
        self.data = data
        self.next = next

def get_intersection_node(head_a, head_b):
    a, b = head_a, head_b
    while a is not b:
        a = a.next if a else head_b
        b = b.next if b else head_a
    return a

shared = ListNode(3, ListNode(4))
head_a = ListNode(1, ListNode(2, shared))
head_b = ListNode(5, shared)
print(get_intersection_node(head_a, head_b).data)
`,
    nextUrl: /\/learn\/py-255-cycle-start/,
    cursorAfter: "255",
  },
  {
    micro: 255,
    id: "py-255-cycle-start",
    title: "DSA Cycle Start",
    solution: `class ListNode:
    def __init__(self, data=0, next=None):
        self.data = data
        self.next = next

def detect_cycle_start(head):
    slow = fast = head
    while fast and fast.next:
        slow = slow.next
        fast = fast.next.next
        if slow is fast:
            slow = head
            while slow is not fast:
                slow = slow.next
                fast = fast.next
            return slow
    return None

n1 = ListNode(1)
n2 = ListNode(2)
n3 = ListNode(3)
n1.next = n2
n2.next = n3
n3.next = n2
print(detect_cycle_start(n1).data)
`,
    nextUrl: /\/learn\/py-256-remove-dupes-ii/,
    cursorAfter: "256",
  },
  {
    micro: 256,
    id: "py-256-remove-dupes-ii",
    title: "DSA Remove Dupes II",
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

def delete_duplicates(head):
    dummy = ListNode(0, head)
    prev = dummy
    while prev.next:
        if prev.next.next and prev.next.data == prev.next.next.data:
            val = prev.next.data
            while prev.next and prev.next.data == val:
                prev.next = prev.next.next
        else:
            prev = prev.next
    return dummy.next

head = ListNode(1, ListNode(1, ListNode(1, ListNode(2, ListNode(3, ListNode(3))))))
print(to_list(delete_duplicates(head)))
`,
    nextUrl: /\/learn\/py-257-remove-k-digits/,
    cursorAfter: "257",
  },
];

test("declares the contiguous learn-route family", () => {
  for (const step of FAMILY) {
    expect(step.id).toMatch(/^py-25[1-6]-/);
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

test.describe("micro-steps 251–256 · listas enlazadas II", () => {
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
      if (nextMicro <= 492) {
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
