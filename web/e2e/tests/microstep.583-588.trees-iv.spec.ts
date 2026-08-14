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
    micro: 583,
    id: "py-583-invert-tree",
    title: "DSA Invert Tree",
    solution: `class TreeNode:
    def __init__(self, data):
        self.data = data
        self.left = None
        self.right = None

def invert_tree(root):
    if root:
        root.left, root.right = invert_tree(root.right), invert_tree(root.left)
    return root

root = TreeNode(4)
root.left = TreeNode(2); root.right = TreeNode(7)
root.left.left = TreeNode(1); root.left.right = TreeNode(3)
root.right.left = TreeNode(6); root.right.right = TreeNode(9)
invert_tree(root)
print(root.left.data, root.right.data)
`,
    nextUrl: /\/learn\/py-584-same-tree/,
    cursorAfter: "584",
  },
  {
    micro: 584,
    id: "py-584-same-tree",
    title: "DSA Same Tree",
    solution: `class TreeNode:
    def __init__(self, data):
        self.data = data
        self.left = None
        self.right = None

def is_same_tree(p, q):
    if not p or not q:
        return p is q
    return p.data == q.data and is_same_tree(p.left, q.left) and is_same_tree(p.right, q.right)

a = TreeNode(1); a.left = TreeNode(2); a.right = TreeNode(3)
b = TreeNode(1); b.left = TreeNode(2); b.right = TreeNode(3)
print(is_same_tree(a, b))
`,
    nextUrl: /\/learn\/py-585-diameter-bt/,
    cursorAfter: "585",
  },
  {
    micro: 585,
    id: "py-585-diameter-bt",
    title: "DSA Diameter BT",
    solution: `class TreeNode:
    def __init__(self, data):
        self.data = data
        self.left = None
        self.right = None

def diameter_of_binary_tree(root):
    best = 0
    def depth(n):
        nonlocal best
        if not n:
            return 0
        l, r = depth(n.left), depth(n.right)
        best = max(best, l + r)
        return 1 + max(l, r)
    depth(root)
    return best

root = TreeNode(1)
root.left = TreeNode(2); root.right = TreeNode(3)
root.left.left = TreeNode(4); root.left.right = TreeNode(5)
print(diameter_of_binary_tree(root))
`,
    nextUrl: /\/learn\/py-586-path-sum/,
    cursorAfter: "586",
  },
  {
    micro: 586,
    id: "py-586-path-sum",
    title: "DSA Path Sum",
    solution: `class TreeNode:
    def __init__(self, data):
        self.data = data
        self.left = None
        self.right = None

def has_path_sum(root, target):
    if not root:
        return False
    if not root.left and not root.right:
        return root.data == target
    return has_path_sum(root.left, target - root.data) or has_path_sum(root.right, target - root.data)

root = TreeNode(5)
root.left = TreeNode(4); root.right = TreeNode(8)
root.left.left = TreeNode(11); root.left.left.left = TreeNode(7); root.left.left.right = TreeNode(2)
root.right.left = TreeNode(13); root.right.right = TreeNode(4); root.right.right.right = TreeNode(1)
print(has_path_sum(root, 22))
`,
    nextUrl: /\/learn\/py-587-sorted-array-bst/,
    cursorAfter: "587",
  },
  {
    micro: 587,
    id: "py-587-sorted-array-bst",
    title: "DSA Array to BST",
    solution: `class TreeNode:
    def __init__(self, data):
        self.data = data
        self.left = None
        self.right = None

def sorted_array_to_bst(nums):
    if not nums:
        return None
    m = len(nums) // 2
    root = TreeNode(nums[m])
    root.left = sorted_array_to_bst(nums[:m])
    root.right = sorted_array_to_bst(nums[m + 1:])
    return root

r = sorted_array_to_bst([-10, -3, 0, 5, 9])
print(r.data, r.left.data, r.right.data)
`,
    nextUrl: /\/learn\/py-588-kth-small-bst/,
    cursorAfter: "588",
  },
  {
    micro: 588,
    id: "py-588-kth-small-bst",
    title: "DSA Kth Small BST",
    solution: `class TreeNode:
    def __init__(self, data):
        self.data = data
        self.left = None
        self.right = None

def kth_smallest(root, k):
    st = []
    while True:
        while root:
            st.append(root)
            root = root.left
        root = st.pop()
        k -= 1
        if k == 0:
            return root.data
        root = root.right

root = TreeNode(3)
root.left = TreeNode(1); root.right = TreeNode(4)
root.left.right = TreeNode(2)
print(kth_smallest(root, 1))
`,
    nextUrl: /\/workspace/,
    cursorAfter: "589",
  }
];

test("declares the contiguous learn-route family", () => {
  for (const step of FAMILY) {
    expect(step.id).toMatch(/^py-(?:583|584|585|586|587|588)-/);
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

test.describe("micro-steps 583–588 · trees IV", () => {
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
