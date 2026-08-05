import assert from "node:assert/strict";
import {
  composeSpeechNotes,
  voiceErrorMessage,
} from "./speech-recognition";

assert.equal(composeSpeechNotes("", "hola", ""), "hola");
assert.equal(composeSpeechNotes("base", " mundo", ""), "base mundo");
assert.equal(composeSpeechNotes("base ", "mundo", ""), "base mundo");
assert.equal(
  composeSpeechNotes("ya tipado", " soy estudiante", " rápido"),
  "ya tipado soy estudiante rápido",
);
assert.equal(voiceErrorMessage("not-allowed").length > 0, true);
assert.equal(voiceErrorMessage("aborted"), "");

console.log("speech-recognition.selfcheck: OK");
