import assert from "node:assert/strict";
import {
  EMPTY_PROFILE_SYNTHESIS,
  simulateAISynthesis,
} from "./profile-synthesis";

assert.deepEqual(simulateAISynthesis(""), EMPTY_PROFILE_SYNTHESIS);
assert.deepEqual(simulateAISynthesis("   "), EMPTY_PROFILE_SYNTHESIS);

const purpose = simulateAISynthesis(
  "Soy estudiante y quiero ayudar a mis padres.",
);
assert.equal(
  purpose.purpose,
  "Ayudar a su familia y ganar autonomía económica.",
);

const urgency = simulateAISynthesis("Necesito resultados rápido por urgencia.");
assert.equal(
  urgency.urgency,
  "Extrema - Necesita resultados inmediatos.",
);

const vision = simulateAISynthesis("De mi futuro profesional no sé qué camino seguir.");
assert.equal(
  vision.vision,
  "Exploratoria. Buscando definir un camino sólido.",
);

const stackCoursera = simulateAISynthesis("Hice un curso en Coursera.");
assert.equal(
  stackCoursera.stack,
  "Coursera. Primer contacto con entornos como Jupyter/Colab.",
);

const stackDeny = simulateAISynthesis("No conozco Jupyter ni Colab todavía.");
assert.equal(
  stackDeny.stack,
  "Coursera. Primer contacto con entornos como Jupyter/Colab.",
);

const none = simulateAISynthesis("Me gusta aprender cosas nuevas cada día.");
assert.deepEqual(none, EMPTY_PROFILE_SYNTHESIS);

console.log("profile-synthesis.selfcheck: OK");
