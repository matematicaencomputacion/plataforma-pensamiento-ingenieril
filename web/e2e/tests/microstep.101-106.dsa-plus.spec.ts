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
    micro: 101,
    id: "py-101-linked-delete",
    title: "DSA Linked List Delete",
    solution: `class Node:
    def __init__(self, data):
        self.data = data
        self.next = None
def deleteNext(node):
    if node.next is not None:
        node.next = node.next.next
node1 = Node(7)
node2 = Node(11)
node3 = Node(3)
node1.next = node2
node2.next = node3
deleteNext(node1)
print(node1.next.data)
`,
    nextUrl: /\/learn\/py-102-linked-insert/,
    cursorAfter: "102",
  },
  {
    micro: 102,
    id: "py-102-linked-insert",
    title: "DSA Linked List Insert",
    solution: `class Node:
    def __init__(self, data):
        self.data = data
        self.next = None
def insertAfter(node, newNode):
    newNode.next = node.next
    node.next = newNode
node1 = Node(7)
node2 = Node(3)
node1.next = node2
insertAfter(node1, Node(97))
print(node1.next.data)
`,
    nextUrl: /\/learn\/py-103-merge-sort/,
    cursorAfter: "103",
  },
  {
    micro: 103,
    id: "py-103-merge-sort",
    title: "DSA Merge Sort",
    solution: `def merge(left, right):
    result = []
    i = j = 0
    while i < len(left) and j < len(right):
        if left[i] < right[j]:
            result.append(left[i])
            i += 1
        else:
            result.append(right[j])
            j += 1
    result.extend(left[i:])
    result.extend(right[j:])
    return result
def mergeSort(arr):
    if len(arr) <= 1:
        return arr
    mid = len(arr) // 2
    return merge(mergeSort(arr[:mid]), mergeSort(arr[mid:]))
mylist = [3, 7, 6, -10, 15, 23.5, 55, -13]
print(mergeSort(mylist))
`,
    nextUrl: /\/learn\/py-104-counting-sort/,
    cursorAfter: "104",
  },
  {
    micro: 104,
    id: "py-104-counting-sort",
    title: "DSA Counting Sort",
    solution: `def countingSort(arr):
    if not arr:
        return []
    size = max(arr) + 1
    count = [0] * size
    for x in arr:
        count[x] += 1
    out = []
    for value, freq in enumerate(count):
        out.extend([value] * freq)
    return out
mylist = [4, 2, 2, 8, 3, 3, 1]
print(countingSort(mylist))
`,
    nextUrl: /\/learn\/py-105-tree-node/,
    cursorAfter: "105",
  },
  {
    micro: 105,
    id: "py-105-tree-node",
    title: "DSA Binary Tree Node",
    solution: `class TreeNode:
    def __init__(self, data):
        self.data = data
        self.left = None
        self.right = None
root = TreeNode(1)
root.left = TreeNode(2)
root.right = TreeNode(3)
print(root.left.data)
print(root.right.data)
`,
    nextUrl: /\/learn\/py-106-tree-preorder/,
    cursorAfter: "106",
  },
  {
    micro: 106,
    id: "py-106-tree-preorder",
    title: "DSA Tree Preorder",
    solution: `class TreeNode:
    def __init__(self, data):
        self.data = data
        self.left = None
        self.right = None
def preorder(node):
    if node is None:
        return
    print(node.data)
    preorder(node.left)
    preorder(node.right)
root = TreeNode(1)
root.left = TreeNode(2)
root.right = TreeNode(3)
preorder(root)
`,
    nextUrl: /\/learn\/py-107-tree-inorder/,
    cursorAfter: "107",
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

test.describe("micro-steps 101–106 · Linked ops / Merge-Counting Sort / Trees", () => {
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
