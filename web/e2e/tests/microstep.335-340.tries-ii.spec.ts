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
    micro: 335,
    id: "py-335-implement-trie",
    title: "DSA Implement Trie",
    solution: `class Trie:
    def __init__(self):
        self.root = {}

    def insert(self, word):
        node = self.root
        for ch in word:
            node = node.setdefault(ch, {})
        node["$"] = True

    def search(self, word):
        node = self.root
        for ch in word:
            if ch not in node:
                return False
            node = node[ch]
        return bool(node.get("$"))

    def starts_with(self, prefix):
        node = self.root
        for ch in prefix:
            if ch not in node:
                return False
            node = node[ch]
        return True

t = Trie()
t.insert("apple")
print([t.search("apple"), t.search("app"), t.starts_with("app")])
`,
    nextUrl: /\/learn\/py-336-word-dict/,
    cursorAfter: "336",
  },
  {
    micro: 336,
    id: "py-336-word-dict",
    title: "DSA Word Dictionary",
    solution: `class WordDictionary:
    def __init__(self):
        self.root = {}

    def add_word(self, word):
        node = self.root
        for ch in word:
            node = node.setdefault(ch, {})
        node["$"] = True

    def search(self, word):
        def dfs(i, node):
            if i == len(word):
                return bool(node.get("$"))
            ch = word[i]
            if ch == ".":
                return any(dfs(i + 1, node[k]) for k in node if k != "$")
            if ch not in node:
                return False
            return dfs(i + 1, node[ch])
        return dfs(0, self.root)

w = WordDictionary()
for x in ("bad", "dad", "mad"):
    w.add_word(x)
print([w.search("pad"), w.search("bad"), w.search(".ad"), w.search("b..")])
`,
    nextUrl: /\/learn\/py-337-replace-words/,
    cursorAfter: "337",
  },
  {
    micro: 337,
    id: "py-337-replace-words",
    title: "DSA Replace Words",
    solution: `def replace_words(dictionary, sentence):
    root = {}
    for w in dictionary:
        node = root
        for ch in w:
            node = node.setdefault(ch, {})
        node["$"] = True

    def repl(word):
        node = root
        pref = []
        for ch in word:
            if ch not in node:
                return word
            node = node[ch]
            pref.append(ch)
            if node.get("$"):
                return "".join(pref)
        return word

    return " ".join(repl(w) for w in sentence.split())

print(replace_words(["cat", "bat", "rat"], "the cattle was rattled by the battery"))
`,
    nextUrl: /\/learn\/py-338-map-sum/,
    cursorAfter: "338",
  },
  {
    micro: 338,
    id: "py-338-map-sum",
    title: "DSA Map Sum",
    solution: `class MapSum:
    def __init__(self):
        self.root = {}
        self.vals = {}

    def insert(self, key, val):
        delta = val - self.vals.get(key, 0)
        self.vals[key] = val
        node = self.root
        for ch in key:
            node = node.setdefault(ch, {"#": 0})
            node["#"] = node.get("#", 0) + delta

    def sum(self, prefix):
        node = self.root
        for ch in prefix:
            if ch not in node:
                return 0
            node = node[ch]
        return node.get("#", 0)

m = MapSum()
m.insert("apple", 3)
a = m.sum("ap")
m.insert("app", 2)
b = m.sum("ap")
print([a, b])
`,
    nextUrl: /\/learn\/py-339-longest-word/,
    cursorAfter: "339",
  },
  {
    micro: 339,
    id: "py-339-longest-word",
    title: "DSA Longest Word",
    solution: `def longest_word(words):
    words = sorted(words)
    seen = {""}
    best = ""
    for w in words:
        if w[:-1] in seen:
            seen.add(w)
            if len(w) > len(best):
                best = w
    return best

print(longest_word(["w", "wo", "wor", "worl", "world"]))
`,
    nextUrl: /\/learn\/py-340-stream-checker/,
    cursorAfter: "340",
  },
  {
    micro: 340,
    id: "py-340-stream-checker",
    title: "DSA Stream Checker",
    solution: `class StreamChecker:
    def __init__(self, words):
        self.root = {}
        self.buf = []
        for w in words:
            node = self.root
            for ch in reversed(w):
                node = node.setdefault(ch, {})
            node["$"] = True

    def query(self, letter):
        self.buf.append(letter)
        node = self.root
        for ch in reversed(self.buf):
            if ch not in node:
                return False
            node = node[ch]
            if node.get("$"):
                return True
        return False

s = StreamChecker(["cd", "f", "kl"])
print([s.query(ch) for ch in "cdaf"])
`,
    nextUrl: /\/learn\/py-341-range-sum/,
    cursorAfter: "341",
  },
];

test("declares the contiguous learn-route family", () => {
  for (const step of FAMILY) {
    expect(step.id).toMatch(/^py-33[5-9]-|^py-340-/);
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

test.describe("micro-steps 335–340 · tries II", () => {
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
      if (nextMicro <= 510) {
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
