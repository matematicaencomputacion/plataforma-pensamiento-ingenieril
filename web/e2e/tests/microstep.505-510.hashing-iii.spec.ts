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
    micro: 505,
    id: "py-505-group-anagrams",
    title: "DSA Group Anagrams",
    solution: `def group_anagrams(strs):
    from collections import defaultdict
    d = defaultdict(list)
    for s in strs:
        d[tuple(sorted(s))].append(s)
    return list(d.values())

print(sorted([sorted(g) for g in group_anagrams(["eat", "tea", "tan", "ate", "nat", "bat"])]))
`,
    nextUrl: /\/learn\/py-506-longest-consec/,
    cursorAfter: "506",
  },
  {
    micro: 506,
    id: "py-506-longest-consec",
    title: "DSA Longest Consec",
    solution: `def longest_consecutive(nums):
    s = set(nums)
    best = 0
    for x in s:
        if x - 1 not in s:
            y = x
            while y in s:
                y += 1
            best = max(best, y - x)
    return best

print(longest_consecutive([100, 4, 200, 1, 3, 2]))
`,
    nextUrl: /\/learn\/py-507-word-pattern/,
    cursorAfter: "507",
  },
  {
    micro: 507,
    id: "py-507-word-pattern",
    title: "DSA Word Pattern",
    solution: `def word_pattern(pattern, s):
    words = s.split()
    if len(pattern) != len(words):
        return False
    p2w, w2p = {}, {}
    for p, w in zip(pattern, words):
        if p2w.get(p, w) != w or w2p.get(w, p) != p:
            return False
        p2w[p] = w; w2p[w] = p
    return True

print(word_pattern("abba", "dog cat cat dog"))
`,
    nextUrl: /\/learn\/py-508-isomorphic/,
    cursorAfter: "508",
  },
  {
    micro: 508,
    id: "py-508-isomorphic",
    title: "DSA Isomorphic",
    solution: `def is_isomorphic(s, t):
    if len(s) != len(t):
        return False
    a, b = {}, {}
    for x, y in zip(s, t):
        if a.get(x, y) != y or b.get(y, x) != x:
            return False
        a[x] = y; b[y] = x
    return True

print(is_isomorphic("egg", "add"))
`,
    nextUrl: /\/learn\/py-509-find-diff/,
    cursorAfter: "509",
  },
  {
    micro: 509,
    id: "py-509-find-diff",
    title: "DSA Find Diff",
    solution: `def find_the_difference(s, t):
    from collections import Counter
    return (Counter(t) - Counter(s)).most_common(1)[0][0]

print(find_the_difference("abcd", "abcde"))
`,
    nextUrl: /\/learn\/py-510-ransom-note/,
    cursorAfter: "510",
  },
  {
    micro: 510,
    id: "py-510-ransom-note",
    title: "DSA Ransom Note",
    solution: `def can_construct(ransom_note, magazine):
    from collections import Counter
    return not (Counter(ransom_note) - Counter(magazine))

print(can_construct("aa", "aab"))
`,
    nextUrl: /\/workspace/,
    cursorAfter: "511",
  }
];

test("declares the contiguous learn-route family", () => {
  for (const step of FAMILY) {
    expect(step.id).toMatch(/^py-(?:505|506|507|508|509|510)-/);
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

test.describe("micro-steps 505–510 · hashing III", () => {
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
