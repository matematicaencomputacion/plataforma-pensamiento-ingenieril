import { component$ } from "@builder.io/qwik";
import type { DocumentHead } from "@builder.io/qwik-city";
import { Link } from "@builder.io/qwik-city";
import {
  BrandName,
  BRAND_NAME_PLAIN,
} from "../components/brand-name/brand-name";
import { SessionBar } from "../components/session-bar/session-bar";

/**
 * Portada de marca (/). Una composición: marca hero, headline, apoyo, CTAs.
 */
export default component$(() => {
  return (
    <main class="landing">
      <div class="landing__atmosphere" aria-hidden="true" />
      <header class="landing__top">
        <SessionBar variant="landing" />
      </header>
      <section class="landing__hero">
        <p class="landing__eyebrow">Pensamiento ingenieril</p>
        <h1 class="landing__brand" aria-label={BRAND_NAME_PLAIN}>
          <BrandName />
        </h1>
        <p class="landing__headline">
          Aprendé a programar resolviendo problemas reales, paso a paso.
        </p>
        <p class="landing__support">
          Abstracción → Diseño → Implementación → Pruebas. Tu progreso queda
          atado a tu cuenta.
        </p>
        <div class="landing__ctas">
          <Link class="landing__cta landing__cta--primary" href="/register">
            Crear cuenta
          </Link>
          <Link class="landing__cta landing__cta--secondary" href="/login">
            Iniciar sesión
          </Link>
        </div>
        <p class="landing__guest">
          <Link class="landing__guest-link" href="/exercise?step=py-01-home">
            Probar sin cuenta
          </Link>
        </p>
      </section>
    </main>
  );
});

export const head: DocumentHead = {
  title: BRAND_NAME_PLAIN,
  meta: [
    {
      name: "description",
      content:
        "IngenierIA — aprendé programación con pensamiento ingenieril. Creá tu cuenta y entrá al workspace.",
    },
  ],
};
