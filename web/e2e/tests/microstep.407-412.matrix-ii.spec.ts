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
    micro: 407,
    id: "py-407-spiral-order",
    title: "DSA Spiral Order",
    solution: `def spiral_order(matrix):
    out = []
    while matrix:
        out += matrix.pop(0)
        if matrix and matrix[0]:
            for row in matrix:
                out.append(row.pop())
        if matrix:
            out += matrix.pop()[::-1]
        if matrix and matrix[0]:
            for row in matrix[::-1]:
                out.append(row.pop(0))
    return out

print(spiral_order([[1, 2, 3], [4, 5, 6], [7, 8, 9]]))
`,
    nextUrl: /\/learn\/py-408-set-zeroes/,
    cursorAfter: "408",
  },
  {
    micro: 408,
    id: "py-408-set-zeroes",
    title: "DSA Set Zeroes",
    solution: `def set_zeroes(matrix):
    m, n = len(matrix), len(matrix[0])
    rows = {i for i in range(m) for j in range(n) if matrix[i][j] == 0}
    cols = {j for i in range(m) for j in range(n) if matrix[i][j] == 0}
    for i in range(m):
        for j in range(n):
            if i in rows or j in cols:
                matrix[i][j] = 0
    return matrix

print(set_zeroes([[1, 1, 1], [1, 0, 1], [1, 1, 1]]))
`,
    nextUrl: /\/learn\/py-409-rotate-image/,
    cursorAfter: "409",
  },
  {
    micro: 409,
    id: "py-409-rotate-image",
    title: "DSA Rotate Image",
    solution: `def rotate(matrix):
    n = len(matrix)
    for i in range(n):
        for j in range(i + 1, n):
            matrix[i][j], matrix[j][i] = matrix[j][i], matrix[i][j]
    for row in matrix:
        row.reverse()
    return matrix

print(rotate([[1, 2, 3], [4, 5, 6], [7, 8, 9]]))
`,
    nextUrl: /\/learn\/py-410-search-2d/,
    cursorAfter: "410",
  },
  {
    micro: 410,
    id: "py-410-search-2d",
    title: "DSA Search 2D",
    solution: `def search_matrix(matrix, target):
    if not matrix:
        return False
    r, c = 0, len(matrix[0]) - 1
    while r < len(matrix) and c >= 0:
        if matrix[r][c] == target:
            return True
        if matrix[r][c] > target:
            c -= 1
        else:
            r += 1
    return False

print(search_matrix([[1, 4, 7, 11, 15], [2, 5, 8, 12, 19], [3, 6, 9, 16, 22], [10, 13, 14, 17, 24], [18, 21, 23, 26, 30]], 5))
`,
    nextUrl: /\/learn\/py-411-game-of-life/,
    cursorAfter: "411",
  },
  {
    micro: 411,
    id: "py-411-game-of-life",
    title: "DSA Game Of Life",
    solution: `import copy

def game_of_life(board):
    m, n = len(board), len(board[0])
    nxt = copy.deepcopy(board)
    for i in range(m):
        for j in range(n):
            live = sum(board[x][y] for x in range(i - 1, i + 2) for y in range(j - 1, j + 2) if 0 <= x < m and 0 <= y < n) - board[i][j]
            if board[i][j] == 1 and (live < 2 or live > 3):
                nxt[i][j] = 0
            elif board[i][j] == 0 and live == 3:
                nxt[i][j] = 1
    for i in range(m):
        board[i] = nxt[i]
    return board

print(game_of_life([[0, 1, 0], [0, 0, 1], [1, 1, 1], [0, 0, 0]]))
`,
    nextUrl: /\/learn\/py-412-reshape-matrix/,
    cursorAfter: "412",
  },
  {
    micro: 412,
    id: "py-412-reshape-matrix",
    title: "DSA Reshape Matrix",
    solution: `def matrix_reshape(mat, r, c):
    flat = [x for row in mat for x in row]
    if len(flat) != r * c:
        return mat
    return [flat[i * c:(i + 1) * c] for i in range(r)]

print(matrix_reshape([[1, 2], [3, 4]], 1, 4))
`,
    nextUrl: /\/learn\/py-413-valid-anagram/,
    cursorAfter: "413",
  }
];

test("declares the contiguous learn-route family", () => {
  for (const step of FAMILY) {
    expect(step.id).toMatch(/^py-(?:407|408|409|410|411|412)-/);
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

test.describe("micro-steps 407–412 · matrix II", () => {
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
