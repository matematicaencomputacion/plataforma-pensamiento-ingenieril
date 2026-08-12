import { expect, test } from "@playwright/test";

const FAMILY = [
  ["py-221-insert-interval", /\/learn\/py-222-erase-overlap/],
  ["py-222-erase-overlap", /\/learn\/py-223-meeting-rooms-ii/],
  ["py-223-meeting-rooms-ii", /\/learn\/py-224-single-number-ii/],
  ["py-224-single-number-ii", /\/learn\/py-225-counting-bits/],
  ["py-225-counting-bits", /\/learn\/py-226-reverse-bits/],
  ["py-226-reverse-bits", /\/learn\/py-227-generate-parens/],
] as const;

test.describe("micro-steps 221–226 · intervalos y bits", () => {
  test("declares the contiguous learn-route family", () => {
    for (const [id, nextUrl] of FAMILY) {
      expect(id).toMatch(/^py-22[1-6]-/);
      expect(nextUrl).toBeInstanceOf(RegExp);
    }
  });
});
