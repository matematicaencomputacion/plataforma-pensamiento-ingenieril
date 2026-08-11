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
    micro: 94,
    id: "py-94-linked-node",
    title: "DSA Linked List Node",
    solution: `class Node:
    def __init__(self, data):
        self.data = data
        self.next = None
node1 = Node(7)
node2 = Node(11)
node1.next = node2
print(node1.next.data)
`,
    nextUrl: /\/learn\/py-95-linked-traverse/,
    cursorAfter: "95",
  },
  {
    micro: 95,
    id: "py-95-linked-traverse",
    title: "DSA Linked List Traverse",
    solution: `class Node:
    def __init__(self, data):
        self.data = data
        self.next = None
def traverse(head):
    current = head
    while current:
        print(current.data, end=" ")
        current = current.next
    print()
node1 = Node(7)
node2 = Node(11)
node3 = Node(3)
node1.next = node2
node2.next = node3
traverse(node1)
`,
    nextUrl: /\/learn\/py-96-linked-lowest/,
    cursorAfter: "96",
  },
  {
    micro: 96,
    id: "py-96-linked-lowest",
    title: "DSA Linked List Lowest",
    solution: `class Node:
    def __init__(self, data):
        self.data = data
        self.next = None
def findLowestValue(head):
    minValue = head.data
    current = head.next
    while current:
        if current.data < minValue:
            minValue = current.data
        current = current.next
    return minValue
node1 = Node(7)
node2 = Node(11)
node3 = Node(3)
node4 = Node(2)
node5 = Node(9)
node1.next = node2
node2.next = node3
node3.next = node4
node4.next = node5
print(findLowestValue(node1))
`,
    nextUrl: /\/learn\/py-97-recursion/,
    cursorAfter: "97",
  },
  {
    micro: 97,
    id: "py-97-recursion",
    title: "Python Recursion (factorial)",
    solution: `def factorial(n):
    if n == 1:
        return 1
    else:
        return n * factorial(n - 1)
print(factorial(5))
`,
    nextUrl: /\/learn\/py-98-fibonacci/,
    cursorAfter: "98",
  },
  {
    micro: 98,
    id: "py-98-fibonacci",
    title: "Python Recursion (Fibonacci)",
    solution: `def fib(n):
    if n == 0:
        return 0
    if n == 1:
        return 1
    return fib(n - 1) + fib(n - 2)
print(fib(7))
`,
    nextUrl: /\/learn\/py-99-quicksort/,
    cursorAfter: "99",
  },
  {
    micro: 99,
    id: "py-99-quicksort",
    title: "DSA Quicksort",
    solution: `def partition(array, low, high):
    pivot = array[high]
    i = low - 1
    for j in range(low, high):
        if array[j] <= pivot:
            i += 1
            array[i], array[j] = array[j], array[i]
    array[i+1], array[high] = array[high], array[i+1]
    return i+1
def quicksort(array, low=0, high=None):
    if high is None:
        high = len(array) - 1
    if low < high:
        pivot_index = partition(array, low, high)
        quicksort(array, low, pivot_index-1)
        quicksort(array, pivot_index+1, high)
mylist = [64, 34, 25, 5, 22, 11, 90, 12]
quicksort(mylist)
print(mylist)
`,
    nextUrl: /\/learn\/py-100-hash-count/,
    cursorAfter: "100",
  },
  {
    micro: 100,
    id: "py-100-hash-count",
    title: "DSA Hash Tables (count)",
    solution: `mylist = ["apple", "banana", "apple", "cherry", "banana", "apple"]
counts = {}
for x in mylist:
    counts[x] = counts.get(x, 0) + 1
print(counts["apple"])
`,
    nextUrl: /\/workspace/,
    cursorAfter: "101",
  },
];

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

test.describe("micro-steps 94–100 · Linked Lists / Recursion / Quicksort / Hash", () => {
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
      if (step.micro < 100) {
        await expect(
          page.locator(`#workspace-microstep-link-${step.micro + 1}`),
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
