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
    micro: 353,
    id: "py-353-num-provinces",
    title: "DSA Num Provinces",
    solution: `def find_circle_num(is_connected):
    n = len(is_connected)
    parent = list(range(n))

    def find(x):
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x

    for i in range(n):
        for j in range(i + 1, n):
            if is_connected[i][j]:
                a, b = find(i), find(j)
                if a != b:
                    parent[b] = a
    return len({find(i) for i in range(n)})

print(find_circle_num([[1, 1, 0], [1, 1, 0], [0, 0, 1]]))
`,
    nextUrl: /\/learn\/py-354-redundant-conn/,
    cursorAfter: "354",
  },
  {
    micro: 354,
    id: "py-354-redundant-conn",
    title: "DSA Redundant Conn",
    solution: `def find_redundant_connection(edges):
    parent = {}

    def find(x):
        parent.setdefault(x, x)
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x

    for a, b in edges:
        ra, rb = find(a), find(b)
        if ra == rb:
            return [a, b]
        parent[rb] = ra
    return []

print(find_redundant_connection([[1, 2], [1, 3], [2, 3]]))
`,
    nextUrl: /\/learn\/py-355-accounts-merge/,
    cursorAfter: "355",
  },
  {
    micro: 355,
    id: "py-355-accounts-merge",
    title: "DSA Accounts Merge",
    solution: `from collections import defaultdict

def accounts_merge(accounts):
    parent = {}

    def find(x):
        parent.setdefault(x, x)
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x

    email_name = {}
    for acc in accounts:
        name = acc[0]
        for e in acc[1:]:
            email_name[e] = name
            find(e)
            parent[find(e)] = find(acc[1])
    groups = defaultdict(list)
    for e in email_name:
        groups[find(e)].append(e)
    return [[email_name[r]] + sorted(emails) for r, emails in groups.items()]

print(sorted(accounts_merge([["John", "johnsmith@mail.com", "john_newyork@mail.com"], ["John", "johnsmith@mail.com", "john00@mail.com"], ["Mary", "mary@mail.com"], ["John", "johnnybravo@mail.com"]])))
`,
    nextUrl: /\/learn\/py-356-smallest-string/,
    cursorAfter: "356",
  },
  {
    micro: 356,
    id: "py-356-smallest-string",
    title: "DSA Smallest String",
    solution: `def smallest_equivalent_string(s1, s2, base_str):
    parent = {chr(c): chr(c) for c in range(ord("a"), ord("z") + 1)}

    def find(x):
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x

    for a, b in zip(s1, s2):
        ra, rb = find(a), find(b)
        if ra < rb:
            parent[rb] = ra
        else:
            parent[ra] = rb
    return "".join(find(ch) for ch in base_str)

print(smallest_equivalent_string("parker", "morris", "parser"))
`,
    nextUrl: /\/learn\/py-357-graph-valid-tree/,
    cursorAfter: "357",
  },
  {
    micro: 357,
    id: "py-357-graph-valid-tree",
    title: "DSA Graph Valid Tree",
    solution: `def valid_tree(n, edges):
    if len(edges) != n - 1:
        return False
    parent = list(range(n))

    def find(x):
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x

    for a, b in edges:
        ra, rb = find(a), find(b)
        if ra == rb:
            return False
        parent[rb] = ra
    return True

print(valid_tree(5, [[0, 1], [0, 2], [0, 3], [1, 4]]))
`,
    nextUrl: /\/learn\/py-358-earliest-friend/,
    cursorAfter: "358",
  },
  {
    micro: 358,
    id: "py-358-earliest-friend",
    title: "DSA Earliest Friend",
    solution: `def earliest_acq(logs, n):
    parent = list(range(n))
    comps = n

    def find(x):
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x

    for t, a, b in sorted(logs):
        ra, rb = find(a), find(b)
        if ra != rb:
            parent[rb] = ra
            comps -= 1
            if comps == 1:
                return t
    return -1

print(earliest_acq([[20190101, 0, 1], [20190104, 3, 4], [20190107, 2, 3], [20190211, 1, 5], [20190224, 2, 4], [20190301, 0, 3], [20190312, 1, 2], [20190322, 4, 5]], 6))
`,
    nextUrl: /\/learn\/py-359-search-rotated/,
    cursorAfter: "359",
  },
];

test("declares the contiguous learn-route family", () => {
  for (const step of FAMILY) {
    expect(step.id).toMatch(/^py-35[3-8]-/);
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

test.describe("micro-steps 353–358 · union-find II", () => {
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
      if (nextMicro <= 576) {
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
