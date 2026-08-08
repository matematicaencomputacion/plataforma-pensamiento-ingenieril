/**
 * Self-check dirty-checking helpers.
 * Ejecutar: cd frontend && npx --yes tsx src/components/exercise-workspace/learner-profile.selfcheck.ts
 */
import assert from "node:assert/strict";
import {
  isUserProfileEmpty,
  normalizeUserProfile,
  profilesEqual,
  snapshotUserProfile,
  synthesisToUserProfile,
} from "./learner-profile";

function main() {
  const a = normalizeUserProfile({
    lifePurpose: "  Hola  ",
    urgency: "ya",
    vision5Years: "staff",
    techStack: "python",
  });
  const b = synthesisToUserProfile({
    purpose: "Hola",
    urgency: "ya",
    vision: "staff",
    stack: "python",
  });
  assert.ok(profilesEqual(a, b));
  assert.equal(snapshotUserProfile(a), snapshotUserProfile(b));
  assert.ok(!isUserProfileEmpty(a));
  assert.ok(isUserProfileEmpty(normalizeUserProfile({})));
  console.log("learner-profile.selfcheck: OK");
}

main();
