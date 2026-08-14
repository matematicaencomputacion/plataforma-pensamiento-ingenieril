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
    micro: 637,
    id: "py-637-jump-game-ii",
    title: "DSA Greedy IV · Jump Game II",
    solution: "def jump(nums):\n    jumps = end = farthest = 0\n    for i in range(len(nums) - 1):\n        farthest = max(farthest, i + nums[i])\n        if i == end:\n            jumps += 1\n            end = farthest\n    return jumps\n\nprint(jump([2, 3, 1, 1, 4]))\n",
    nextUrl: /\/learn\/py-638-candy/,
    cursorAfter: "638",
  },
  {
    micro: 638,
    id: "py-638-candy",
    title: "DSA Greedy IV · Candy",
    solution: "def candy(ratings):\n    n = len(ratings)\n    give = [1] * n\n    for i in range(1, n):\n        if ratings[i] > ratings[i - 1]:\n            give[i] = give[i - 1] + 1\n    for i in range(n - 2, -1, -1):\n        if ratings[i] > ratings[i + 1]:\n            give[i] = max(give[i], give[i + 1] + 1)\n    return sum(give)\n\nprint(candy([1, 0, 2]))\n",
    nextUrl: /\/learn\/py-639-reconstruct-queue/,
    cursorAfter: "639",
  },
  {
    micro: 639,
    id: "py-639-reconstruct-queue",
    title: "DSA Greedy IV · Queue Reconstruction",
    solution: "def reconstruct_queue(people):\n    people.sort(key=lambda p: (-p[0], p[1]))\n    out = []\n    for person in people:\n        out.insert(person[1], person)\n    return out\n\nprint(reconstruct_queue([[7, 0], [4, 4], [7, 1], [5, 0], [6, 1], [5, 2]]))\n",
    nextUrl: /\/learn\/py-640-partition-labels/,
    cursorAfter: "640",
  },
  {
    micro: 640,
    id: "py-640-partition-labels",
    title: "DSA Greedy IV · Partition Labels",
    solution: "def partition_labels(s):\n    last = {ch: i for i, ch in enumerate(s)}\n    out = []\n    start = end = 0\n    for i, ch in enumerate(s):\n        end = max(end, last[ch])\n        if i == end:\n            out.append(end - start + 1)\n            start = i + 1\n    return out\n\nprint(partition_labels('ababcbacadefegdehijhklij'))\n",
    nextUrl: /\/learn\/py-641-task-scheduler/,
    cursorAfter: "641",
  },
  {
    micro: 641,
    id: "py-641-task-scheduler",
    title: "DSA Greedy IV · Task Scheduler",
    solution: "from collections import Counter\n\ndef least_interval(tasks, n):\n    counts = list(Counter(tasks).values())\n    peak = max(counts)\n    extra = counts.count(peak)\n    return max(len(tasks), (peak - 1) * (n + 1) + extra)\n\nprint(least_interval(['A', 'A', 'A', 'B', 'B', 'B'], 2))\n",
    nextUrl: /\/learn\/py-642-bag-of-tokens/,
    cursorAfter: "642",
  },
  {
    micro: 642,
    id: "py-642-bag-of-tokens",
    title: "DSA Greedy IV · Bag of Tokens",
    solution: "def bag_of_tokens_score(tokens, power):\n    tokens.sort()\n    lo, hi = 0, len(tokens) - 1\n    score = best = 0\n    while lo <= hi:\n        if power >= tokens[lo]:\n            power -= tokens[lo]\n            lo += 1\n            score += 1\n            best = max(best, score)\n        elif score:\n            power += tokens[hi]\n            hi -= 1\n            score -= 1\n        else:\n            break\n    return best\n\nprint(bag_of_tokens_score([100, 200, 300, 400], 200))\n",
    nextUrl: /\/workspace/,
    cursorAfter: "643",
  },
];

test("declares the contiguous learn-route family", () => {
  for (const step of FAMILY) {
    expect(step.id).toMatch(/^py-(?:637|638|639|640|641|642)-/);
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

test.describe("micro-steps 637–642 · greedy iv", () => {
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
      if (nextMicro <= 642) {
        await expect(
          page.locator(
            `#workspace-microsteps [data-microstep="${nextMicro}"]`,
          ),
        ).toHaveClass(/workspace__microstep--open|workspace__microstep--jumpable/);
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

      if (step.micro < 642) {
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
