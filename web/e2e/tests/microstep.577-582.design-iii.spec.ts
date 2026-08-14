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
    micro: 577,
    id: "py-577-min-stack",
    title: "DSA Min Stack",
    solution: `class MinStack:
    def __init__(self):
        self.st = []
        self.mn = []
    def push(self, val):
        self.st.append(val)
        self.mn.append(val if not self.mn else min(val, self.mn[-1]))
    def pop(self):
        self.st.pop(); self.mn.pop()
    def top(self):
        return self.st[-1]
    def get_min(self):
        return self.mn[-1]

s = MinStack()
s.push(-2); s.push(0); s.push(-3)
print([s.get_min(), s.top()])
s.pop()
print([s.get_min(), s.top()])
`,
    nextUrl: /\/learn\/py-578-my-queue/,
    cursorAfter: "578",
  },
  {
    micro: 578,
    id: "py-578-my-queue",
    title: "DSA Stack Queue",
    solution: `class MyQueue:
    def __init__(self):
        self.a, self.b = [], []
    def push(self, x):
        self.a.append(x)
    def pop(self):
        self.peek()
        return self.b.pop()
    def peek(self):
        if not self.b:
            while self.a:
                self.b.append(self.a.pop())
        return self.b[-1]
    def empty(self):
        return not self.a and not self.b

q = MyQueue()
q.push(1); q.push(2)
print([q.peek(), q.pop(), q.empty()])
`,
    nextUrl: /\/learn\/py-579-my-stack/,
    cursorAfter: "579",
  },
  {
    micro: 579,
    id: "py-579-my-stack",
    title: "DSA Queue Stack",
    solution: `class MyStack:
    def __init__(self):
        from collections import deque
        self.q = deque()
    def push(self, x):
        self.q.append(x)
        for _ in range(len(self.q) - 1):
            self.q.append(self.q.popleft())
    def pop(self):
        return self.q.popleft()
    def top(self):
        return self.q[0]
    def empty(self):
        return not self.q

s = MyStack()
s.push(1); s.push(2)
print([s.top(), s.pop(), s.empty()])
`,
    nextUrl: /\/learn\/py-580-lru-cache/,
    cursorAfter: "580",
  },
  {
    micro: 580,
    id: "py-580-lru-cache",
    title: "DSA LRU Cache",
    solution: `class LRUCache:
    def __init__(self, cap):
        from collections import OrderedDict
        self.cap = cap
        self.d = OrderedDict()
    def get(self, key):
        if key not in self.d:
            return -1
        self.d.move_to_end(key)
        return self.d[key]
    def put(self, key, value):
        if key in self.d:
            self.d.move_to_end(key)
        self.d[key] = value
        if len(self.d) > self.cap:
            self.d.popitem(last=False)

c = LRUCache(2)
c.put(1, 1); c.put(2, 2)
print([c.get(1)])
c.put(3, 3)
print([c.get(2), c.get(3)])
`,
    nextUrl: /\/learn\/py-581-hash-map/,
    cursorAfter: "581",
  },
  {
    micro: 581,
    id: "py-581-hash-map",
    title: "DSA Hash Map",
    solution: `class MyHashMap:
    def __init__(self):
        self.d = {}
    def put(self, key, value):
        self.d[key] = value
    def get(self, key):
        return self.d.get(key, -1)
    def remove(self, key):
        self.d.pop(key, None)

m = MyHashMap()
m.put(1, 1); m.put(2, 2)
print([m.get(1), m.get(3)])
m.put(2, 1)
print([m.get(2)])
m.remove(2)
print([m.get(2)])
`,
    nextUrl: /\/learn\/py-582-hash-set/,
    cursorAfter: "582",
  },
  {
    micro: 582,
    id: "py-582-hash-set",
    title: "DSA Hash Set",
    solution: `class MyHashSet:
    def __init__(self):
        self.s = set()
    def add(self, key):
        self.s.add(key)
    def remove(self, key):
        self.s.discard(key)
    def contains(self, key):
        return key in self.s

h = MyHashSet()
h.add(1); h.add(2)
print([h.contains(1), h.contains(3)])
h.add(2)
print([h.contains(2)])
h.remove(2)
print([h.contains(2)])
`,
    nextUrl: /\/workspace/,
    cursorAfter: "583",
  }
];

test("declares the contiguous learn-route family", () => {
  for (const step of FAMILY) {
    expect(step.id).toMatch(/^py-(?:577|578|579|580|581|582)-/);
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

test.describe("micro-steps 577–582 · design III", () => {
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
