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
    micro: 419,
    id: "py-419-min-stack",
    title: "DSA Min Stack",
    solution: `class MinStack:
    def __init__(self):
        self.st = []
        self.mn = []

    def push(self, x):
        self.st.append(x)
        self.mn.append(x if not self.mn else min(x, self.mn[-1]))

    def pop(self):
        self.st.pop()
        self.mn.pop()

    def top(self):
        return self.st[-1]

    def getMin(self):
        return self.mn[-1]

s = MinStack()
s.push(-2)
s.push(0)
s.push(-3)
a = s.getMin()
s.pop()
b = s.top()
c = s.getMin()
print([a, b, c])
`,
    nextUrl: /\/learn\/py-420-lru-cache/,
    cursorAfter: "420",
  },
  {
    micro: 420,
    id: "py-420-lru-cache",
    title: "DSA LRU Cache",
    solution: `from collections import OrderedDict

class LRUCache:
    def __init__(self, capacity):
        self.cap = capacity
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
c.put(1, 1)
c.put(2, 2)
a = c.get(1)
c.put(3, 3)
b = c.get(2)
c.put(4, 4)
d = c.get(1)
e = c.get(3)
f = c.get(4)
print([a, b, d, e, f])
`,
    nextUrl: /\/learn\/py-421-time-map/,
    cursorAfter: "421",
  },
  {
    micro: 421,
    id: "py-421-time-map",
    title: "DSA Time Map",
    solution: `import bisect
from collections import defaultdict

class TimeMap:
    def __init__(self):
        self.d = defaultdict(list)

    def set(self, key, value, timestamp):
        self.d[key].append((timestamp, value))

    def get(self, key, timestamp):
        arr = self.d[key]
        i = bisect.bisect_right(arr, (timestamp, chr(127))) - 1
        return arr[i][1] if i >= 0 else ""

t = TimeMap()
t.set("foo", "bar", 1)
a = t.get("foo", 1)
b = t.get("foo", 3)
t.set("foo", "bar2", 4)
c = t.get("foo", 4)
d = t.get("foo", 5)
print([a, b, c, d])
`,
    nextUrl: /\/learn\/py-422-randomized-set/,
    cursorAfter: "422",
  },
  {
    micro: 422,
    id: "py-422-randomized-set",
    title: "DSA Randomized Set",
    solution: `import random

class RandomizedSet:
    def __init__(self):
        self.arr = []
        self.idx = {}

    def insert(self, val):
        if val in self.idx:
            return False
        self.idx[val] = len(self.arr)
        self.arr.append(val)
        return True

    def remove(self, val):
        if val not in self.idx:
            return False
        i = self.idx[val]
        last = self.arr[-1]
        self.arr[i] = last
        self.idx[last] = i
        self.arr.pop()
        del self.idx[val]
        return True

    def getRandom(self):
        return random.choice(self.arr)

r = RandomizedSet()
print([r.insert(1), r.insert(2), r.remove(1), r.getRandom()])
`,
    nextUrl: /\/learn\/py-423-moving-avg/,
    cursorAfter: "423",
  },
  {
    micro: 423,
    id: "py-423-moving-avg",
    title: "DSA Moving Avg",
    solution: `from collections import deque

class MovingAverage:
    def __init__(self, size):
        self.size = size
        self.q = deque()
        self.s = 0

    def next(self, val):
        self.q.append(val)
        self.s += val
        if len(self.q) > self.size:
            self.s -= self.q.popleft()
        return self.s / len(self.q)

m = MovingAverage(3)
print([m.next(1), m.next(10), m.next(3), m.next(5)])
`,
    nextUrl: /\/learn\/py-424-browser-history/,
    cursorAfter: "424",
  },
  {
    micro: 424,
    id: "py-424-browser-history",
    title: "DSA Browser History",
    solution: `class BrowserHistory:
    def __init__(self, homepage):
        self.h = [homepage]
        self.i = 0

    def visit(self, url):
        self.h = self.h[: self.i + 1]
        self.h.append(url)
        self.i += 1

    def back(self, steps):
        self.i = max(0, self.i - steps)
        return self.h[self.i]

    def forward(self, steps):
        self.i = min(len(self.h) - 1, self.i + steps)
        return self.h[self.i]

b = BrowserHistory("leetcode.com")
b.visit("google.com")
b.visit("facebook.com")
b.visit("youtube.com")
a = b.back(1)
c = b.back(1)
d = b.forward(1)
b.visit("linkedin.com")
e = b.forward(2)
f = b.back(2)
g = b.back(7)
print([a, c, d, e, f, g])
`,
    nextUrl: /\/learn\/py-425-jump-game/,
    cursorAfter: "425",
  }
];

test("declares the contiguous learn-route family", () => {
  for (const step of FAMILY) {
    expect(step.id).toMatch(/^py-(?:419|420|421|422|423|424)-/);
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

test.describe("micro-steps 419–424 · design II", () => {
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
      if (nextMicro <= 600) {
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
