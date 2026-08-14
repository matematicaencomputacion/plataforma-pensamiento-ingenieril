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
    micro: 125,
    id: "py-125-greedy-coin",
    title: "DSA Greedy Coin",
    solution: `def greedy_coin(coins, amount):
    coins = sorted(coins, reverse=True)
    count = 0
    for c in coins:
        count += amount // c
        amount %= c
    return count
print(greedy_coin([25, 10, 5, 1], 63))
`,
    nextUrl: /\/learn\/py-126-activity-select/,
    cursorAfter: "126",
  },
  {
    micro: 126,
    id: "py-126-activity-select",
    title: "DSA Activity Selection",
    solution: `def activity_select(intervals):
    intervals = sorted(intervals, key=lambda x: x[1])
    picked = []
    end = -1
    for s, e in intervals:
        if s >= end:
            picked.append((s, e))
            end = e
    return picked
print(activity_select([(1, 4), (3, 5), (0, 6), (5, 7), (8, 9), (5, 9)]))
`,
    nextUrl: /\/learn\/py-127-tsp-nearest/,
    cursorAfter: "127",
  },
  {
    micro: 127,
    id: "py-127-tsp-nearest",
    title: "DSA TSP Nearest Neighbor",
    solution: `distances = [[0, 2, 9, 10], [1, 0, 6, 4], [15, 7, 0, 8], [6, 3, 12, 0]]
def nearest_neighbor_tsp(distances):
    n = len(distances)
    visited = [False] * n
    route = [0]
    visited[0] = True
    total = 0
    for _ in range(1, n):
        last = route[-1]
        nearest = min((i for i in range(n) if not visited[i]), key=lambda i: distances[last][i])
        total += distances[last][nearest]
        route.append(nearest)
        visited[nearest] = True
    total += distances[route[-1]][0]
    route.append(0)
    return route, total
route, total = nearest_neighbor_tsp(distances)
print(total)
`,
    nextUrl: /\/learn\/py-128-lcs/,
    cursorAfter: "128",
  },
  {
    micro: 128,
    id: "py-128-lcs",
    title: "DSA LCS",
    solution: `def lcs(a, b):
    m, n = len(a), len(b)
    dp = [[0] * (n + 1) for _ in range(m + 1)]
    for i in range(1, m + 1):
        for j in range(1, n + 1):
            if a[i - 1] == b[j - 1]:
                dp[i][j] = dp[i - 1][j - 1] + 1
            else:
                dp[i][j] = max(dp[i - 1][j], dp[i][j - 1])
    return dp[m][n]
print(lcs('ABCBDAB', 'BDCABA'))
`,
    nextUrl: /\/learn\/py-129-coin-change-dp/,
    cursorAfter: "129",
  },
  {
    micro: 129,
    id: "py-129-coin-change-dp",
    title: "DSA Coin Change DP",
    solution: `def coin_change(coins, amount):
    dp = [0] + [float('inf')] * amount
    for a in range(1, amount + 1):
        for c in coins:
            if c <= a:
                dp[a] = min(dp[a], dp[a - c] + 1)
    return int(dp[amount]) if dp[amount] != float('inf') else -1
print(coin_change([1, 3, 4], 6))
`,
    nextUrl: /\/learn\/py-130-floyd-warshall/,
    cursorAfter: "130",
  },
  {
    micro: 130,
    id: "py-130-floyd-warshall",
    title: "DSA Floyd-Warshall",
    solution: `INF = 999
graph = [[0, 3, INF, 7], [8, 0, 2, INF], [5, INF, 0, 1], [2, INF, INF, 0]]
def floyd(graph):
    n = len(graph)
    dist = [row[:] for row in graph]
    for k in range(n):
        for i in range(n):
            for j in range(n):
                if dist[i][k] + dist[k][j] < dist[i][j]:
                    dist[i][j] = dist[i][k] + dist[k][j]
    return dist
print(floyd(graph)[0])
`,
    nextUrl: /\/learn\/py-131-two-pointers/,
    cursorAfter: "131",
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

test.describe("micro-steps 125–130 · Greedy / TSP / LCS / Coin DP / Floyd", () => {
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
      if (nextMicro <= 480) {
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
