import { expect, test } from "@playwright/test";

const FAMILY = [
  ["py-215-min-window", /\/learn\/py-216-char-replace/],
  ["py-216-char-replace", /\/learn\/py-217-find-anagrams/],
  ["py-217-find-anagrams", /\/learn\/py-218-decode-string/],
  ["py-218-decode-string", /\/learn\/py-219-str-compress/],
  ["py-219-str-compress", /\/learn\/py-220-multiply-strings/],
  ["py-220-multiply-strings", /\/learn\/py-221-insert-interval/],
] as const;

test.describe("micro-steps 215–220 · ventanas y strings", () => {
  test("declares the contiguous learn-route family", () => {
    for (const [id, nextUrl] of FAMILY) {
      expect(id).toMatch(/^py-2(1[5-9]|20)-/);
      expect(nextUrl).toBeInstanceOf(RegExp);
    }
  });
});
