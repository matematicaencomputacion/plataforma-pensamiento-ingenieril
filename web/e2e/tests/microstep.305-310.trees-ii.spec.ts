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
    micro: 305,
    id: "py-305-max-path-sum",
    title: "DSA Max Path Sum",
    solution: `class TreeNode:
    def __init__(self, data):
        self.data = data
        self.left = None
        self.right = None

def max_path_sum(root):
    best = float('-inf')
    def dfs(node):
        nonlocal best
        if node is None:
            return 0
        left = max(dfs(node.left), 0)
        right = max(dfs(node.right), 0)
        best = max(best, node.data + left + right)
        return node.data + max(left, right)
    dfs(root)
    return best

root = TreeNode(-10)
root.left = TreeNode(9)
root.right = TreeNode(20)
root.right.left = TreeNode(15)
root.right.right = TreeNode(7)
print(max_path_sum(root))
`,
    nextUrl: /\/learn\/py-306-serialize-tree/,
    cursorAfter: "306",
  },
  {
    micro: 306,
    id: "py-306-serialize-tree",
    title: "DSA Serialize Tree",
    solution: `from collections import deque

class TreeNode:
    def __init__(self, data):
        self.data = data
        self.left = None
        self.right = None

def serialize(root):
    if root is None:
        return '#'
    out = []
    q = deque([root])
    while q:
        node = q.popleft()
        if node is None:
            out.append('#')
            continue
        out.append(str(node.data))
        q.append(node.left)
        q.append(node.right)
    return ','.join(out)

def deserialize(data):
    if data == '#':
        return None
    vals = data.split(',')
    root = TreeNode(int(vals[0]))
    q = deque([root])
    i = 1
    while q and i < len(vals):
        node = q.popleft()
        if vals[i] != '#':
            node.left = TreeNode(int(vals[i]))
            q.append(node.left)
        i += 1
        if i < len(vals) and vals[i] != '#':
            node.right = TreeNode(int(vals[i]))
            q.append(node.right)
        i += 1
    return root

root = TreeNode(1)
root.left = TreeNode(2)
root.right = TreeNode(3)
root.right.left = TreeNode(4)
root.right.right = TreeNode(5)
print(serialize(root))
`,
    nextUrl: /\/learn\/py-307-build-tree/,
    cursorAfter: "307",
  },
  {
    micro: 307,
    id: "py-307-build-tree",
    title: "DSA Build Tree",
    solution: `class TreeNode:
    def __init__(self, data):
        self.data = data
        self.left = None
        self.right = None

def build_tree(preorder, inorder):
    if not preorder:
        return None
    root = TreeNode(preorder[0])
    mid = inorder.index(preorder[0])
    root.left = build_tree(preorder[1:1 + mid], inorder[:mid])
    root.right = build_tree(preorder[1 + mid:], inorder[mid + 1:])
    return root

def preorder_walk(root):
    if root is None:
        return []
    return [root.data] + preorder_walk(root.left) + preorder_walk(root.right)

root = build_tree([3, 9, 20, 15, 7], [9, 3, 15, 20, 7])
print(preorder_walk(root))
`,
    nextUrl: /\/learn\/py-308-kth-bst/,
    cursorAfter: "308",
  },
  {
    micro: 308,
    id: "py-308-kth-bst",
    title: "DSA Kth BST",
    solution: `class TreeNode:
    def __init__(self, data):
        self.data = data
        self.left = None
        self.right = None

def kth_smallest(root, k):
    stack = []
    cur = root
    while True:
        while cur:
            stack.append(cur)
            cur = cur.left
        cur = stack.pop()
        k -= 1
        if k == 0:
            return cur.data
        cur = cur.right

root = TreeNode(3)
root.left = TreeNode(1)
root.right = TreeNode(4)
root.left.right = TreeNode(2)
print(kth_smallest(root, 1))
`,
    nextUrl: /\/learn\/py-309-level-order/,
    cursorAfter: "309",
  },
  {
    micro: 309,
    id: "py-309-level-order",
    title: "DSA Level Order",
    solution: `from collections import deque

class TreeNode:
    def __init__(self, data):
        self.data = data
        self.left = None
        self.right = None

def level_order(root):
    if root is None:
        return []
    out = []
    q = deque([root])
    while q:
        level = []
        for _ in range(len(q)):
            node = q.popleft()
            level.append(node.data)
            if node.left:
                q.append(node.left)
            if node.right:
                q.append(node.right)
        out.append(level)
    return out

root = TreeNode(3)
root.left = TreeNode(9)
root.right = TreeNode(20)
root.right.left = TreeNode(15)
root.right.right = TreeNode(7)
print(level_order(root))
`,
    nextUrl: /\/learn\/py-310-is-balanced/,
    cursorAfter: "310",
  },
  {
    micro: 310,
    id: "py-310-is-balanced",
    title: "DSA Is Balanced",
    solution: `class TreeNode:
    def __init__(self, data):
        self.data = data
        self.left = None
        self.right = None

def is_balanced(root):
    def height(node):
        if node is None:
            return 0
        lh = height(node.left)
        if lh < 0:
            return -1
        rh = height(node.right)
        if rh < 0:
            return -1
        if abs(lh - rh) > 1:
            return -1
        return 1 + max(lh, rh)
    return height(root) >= 0

root = TreeNode(3)
root.left = TreeNode(9)
root.right = TreeNode(20)
root.right.left = TreeNode(15)
root.right.right = TreeNode(7)
print(is_balanced(root))
`,
    nextUrl: /\/learn\/py-311-min-arrows/,
    cursorAfter: "311",
  },
];

test("declares the contiguous learn-route family", () => {
  for (const step of FAMILY) {
    expect(step.id).toMatch(/^py-30[5-9]-|^py-310-/);
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

test.describe("micro-steps 305–310 · trees II", () => {
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
      if (nextMicro <= 594) {
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
