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
    micro: 625,
    id: "py-625-implement-trie",
    title: "DSA Trie III · Implement Trie",
    solution: "class Trie:\n    def __init__(self):\n        self.children = {}\n        self.end = False\n\n    def insert(self, word):\n        node = self\n        for ch in word:\n            node = node.children.setdefault(ch, Trie())\n        node.end = True\n\n    def search(self, word):\n        node = self\n        for ch in word:\n            if ch not in node.children:\n                return False\n            node = node.children[ch]\n        return node.end\n\n    def starts_with(self, prefix):\n        node = self\n        for ch in prefix:\n            if ch not in node.children:\n                return False\n            node = node.children[ch]\n        return True\n\nt = Trie(); t.insert('apple'); print([t.search('apple'), t.search('app'), t.starts_with('app')])\n",
    nextUrl: /\/learn\/py-626-replace-words/,
    cursorAfter: "626",
  },
  {
    micro: 626,
    id: "py-626-replace-words",
    title: "DSA Trie III · Replace Words",
    solution: "def replace_words(dictionary, sentence):\n    trie = {}\n    for word in dictionary:\n        node = trie\n        for ch in word:\n            node = node.setdefault(ch, {})\n        node['#'] = True\n    def root_of(word):\n        node = trie\n        acc = []\n        for ch in word:\n            if '#' in node:\n                return ''.join(acc)\n            if ch not in node:\n                return word\n            acc.append(ch)\n            node = node[ch]\n        return ''.join(acc) if '#' in node else word\n    return ' '.join(root_of(w) for w in sentence.split())\n\nprint(replace_words(['cat', 'bat', 'rat'], 'the cattle was rattled by the battery'))\n",
    nextUrl: /\/learn\/py-627-longest-word-dict/,
    cursorAfter: "627",
  },
  {
    micro: 627,
    id: "py-627-longest-word-dict",
    title: "DSA Trie III · Longest Word",
    solution: "def longest_word(words):\n    seen = set(words)\n    best = ''\n    for word in sorted(words):\n        if all(word[:i] in seen for i in range(1, len(word))) and (len(word) > len(best) or (len(word) == len(best) and word < best)):\n            best = word\n    return best\n\nprint(longest_word(['w', 'wo', 'wor', 'worl', 'world']))\n",
    nextUrl: /\/learn\/py-628-map-sum/,
    cursorAfter: "628",
  },
  {
    micro: 628,
    id: "py-628-map-sum",
    title: "DSA Trie III · Map Sum",
    solution: "class MapSum:\n    def __init__(self):\n        self.vals = {}\n        self.trie = {}\n\n    def insert(self, key, val):\n        delta = val - self.vals.get(key, 0)\n        self.vals[key] = val\n        node = self.trie\n        for ch in key:\n            node = node.setdefault(ch, {'$': 0})\n            node['$'] = node.get('$', 0) + delta\n\n    def sum(self, prefix):\n        node = self.trie\n        for ch in prefix:\n            if ch not in node:\n                return 0\n            node = node[ch]\n        return node.get('$', 0)\n\nm = MapSum(); m.insert('apple', 3); m.insert('app', 2); print(m.sum('ap'))\n",
    nextUrl: /\/learn\/py-629-count-prefix-pairs/,
    cursorAfter: "629",
  },
  {
    micro: 629,
    id: "py-629-count-prefix-pairs",
    title: "DSA Trie III · Prefix Pairs",
    solution: "def count_prefix_suffix_pairs(words):\n    total = 0\n    n = len(words)\n    for i in range(n):\n        for j in range(i + 1, n):\n            a, b = words[i], words[j]\n            if (b.startswith(a) and b.endswith(a)) or (a.startswith(b) and a.endswith(b)):\n                total += 1\n    return total\n\nprint(count_prefix_suffix_pairs(['a', 'aba', 'ababa', 'aa']))\n",
    nextUrl: /\/learn\/py-630-max-xor-two/,
    cursorAfter: "630",
  },
  {
    micro: 630,
    id: "py-630-max-xor-two",
    title: "DSA Trie III · Max XOR",
    solution: "def find_maximum_xor(nums):\n    trie = {}\n    for x in nums:\n        node = trie\n        for b in range(31, -1, -1):\n            bit = (x >> b) & 1\n            node = node.setdefault(bit, {})\n    best = 0\n    for x in nums:\n        node = trie\n        acc = 0\n        for b in range(31, -1, -1):\n            bit = (x >> b) & 1\n            want = 1 - bit\n            if want in node:\n                acc |= 1 << b\n                node = node[want]\n            else:\n                node = node[bit]\n        best = max(best, acc)\n    return best\n\nprint(find_maximum_xor([3, 10, 5, 25, 2, 8]))\n",
    nextUrl: /\/learn\/py-631-insert-interval/,
    cursorAfter: "631",
  },
];

test("declares the contiguous learn-route family", () => {
  for (const step of FAMILY) {
    expect(step.id).toMatch(/^py-(?:625|626|627|628|629|630)-/);
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

test.describe("micro-steps 625–630 · trie iii", () => {
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
      if (nextMicro <= 630) {
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

      if (step.micro < 630) {
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
