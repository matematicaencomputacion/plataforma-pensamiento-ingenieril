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
    micro: 88,
    id: "py-88-linear-in",
    title: "DSA Linear Search (in)",
    solution: `mylist = [3, 7, 2, 9, 5, 1, 8, 4, 6]
if 4 in mylist:
    print("Found!")
else:
    print("Not found!")
`,
    nextUrl: /\/learn\/py-89-linear-search/,
    cursorAfter: "89",
  },
  {
    micro: 89,
    id: "py-89-linear-search",
    title: "DSA Linear Search Index",
    solution: `def linearSearch(arr, targetVal):
    for i in range(len(arr)):
        if arr[i] == targetVal:
            return i
    return -1
mylist = [3, 7, 2, 9, 5, 1, 8, 4, 6]
x = 4
print(linearSearch(mylist, x))
`,
    nextUrl: /\/learn\/py-90-bubble-sort/,
    cursorAfter: "90",
  },
  {
    micro: 90,
    id: "py-90-bubble-sort",
    title: "DSA Bubble Sort",
    solution: `mylist = [64, 34, 25, 12, 22, 11, 90, 5]
n = len(mylist)
for i in range(n-1):
    for j in range(n-i-1):
        if mylist[j] > mylist[j+1]:
            mylist[j], mylist[j+1] = mylist[j+1], mylist[j]
print(mylist)
`,
    nextUrl: /\/learn\/py-91-binary-search/,
    cursorAfter: "91",
  },
  {
    micro: 91,
    id: "py-91-binary-search",
    title: "DSA Binary Search",
    solution: `def binarySearch(arr, targetVal):
    left = 0
    right = len(arr) - 1
    while left <= right:
        mid = (left + right) // 2
        if arr[mid] == targetVal:
            return mid
        if arr[mid] < targetVal:
            left = mid + 1
        else:
            right = mid - 1
    return -1
mylist = [1, 3, 5, 7, 9, 11, 13, 15]
x = 11
print(binarySearch(mylist, x))
`,
    nextUrl: /\/learn\/py-92-selection-sort/,
    cursorAfter: "92",
  },
  {
    micro: 92,
    id: "py-92-selection-sort",
    title: "DSA Selection Sort",
    solution: `mylist = [64, 34, 25, 12, 22, 11, 90, 5]
n = len(mylist)
for i in range(n):
    min_idx = i
    for j in range(i+1, n):
        if mylist[j] < mylist[min_idx]:
            min_idx = j
    mylist[i], mylist[min_idx] = mylist[min_idx], mylist[i]
print(mylist)
`,
    nextUrl: /\/learn\/py-93-insertion-sort/,
    cursorAfter: "93",
  },
  {
    micro: 93,
    id: "py-93-insertion-sort",
    title: "DSA Insertion Sort",
    solution: `mylist = [64, 34, 25, 12, 22, 11, 90, 5]
n = len(mylist)
for i in range(1, n):
    key = mylist[i]
    j = i - 1
    while j >= 0 and mylist[j] > key:
        mylist[j + 1] = mylist[j]
        j -= 1
    mylist[j + 1] = key
print(mylist)
`,
    nextUrl: /\/learn\/py-94-linked-node/,
    cursorAfter: "94",
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

test.describe("micro-steps 88–93 · DSA Search & Sort", () => {
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
      if (nextMicro <= 498) {
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
