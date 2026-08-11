import { expect, type APIRequestContext } from "@playwright/test";
import { e2eTimeout } from "./helpers";

/** Ordered coding rail micro-steps (level_id === micro_step). */
export const MICRO_PROGRESS_STEPS: ReadonlyArray<{
  level_id: number;
  step_id: string;
}> = [
  { level_id: 1, step_id: "py-02-variables" },
  { level_id: 2, step_id: "py-02-intro" },
  { level_id: 3, step_id: "py-03-get-started" },
  { level_id: 4, step_id: "py-04-syntax" },
  { level_id: 5, step_id: "py-05-output" },
  { level_id: 6, step_id: "py-06-comments" },
  { level_id: 7, step_id: "py-07-data-types" },
];

/** Complete steps 1..=throughLevelId so the next cell becomes current. */
export async function unlockThroughMicroStep(
  request: APIRequestContext,
  token: string,
  throughLevelId: number,
) {
  const steps = MICRO_PROGRESS_STEPS.filter((s) => s.level_id <= throughLevelId);
  for (const body of steps) {
    const res = await request.post("/api/progress/complete", {
      headers: { Authorization: `Bearer ${token}` },
      data: { ...body, passed: true },
      timeout: e2eTimeout,
    });
    expect(res.ok(), await res.text()).toBeTruthy();
  }
}
