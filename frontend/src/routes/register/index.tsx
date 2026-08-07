import { component$, useSignal } from "@builder.io/qwik";
import type { DocumentHead } from "@builder.io/qwik-city";
import { Link, useNavigate } from "@builder.io/qwik-city";
import {
  BrandName,
  BRAND_NAME_PLAIN,
} from "../../components/brand-name/brand-name";
import { registerUser, storeToken } from "../../lib/auth";

export default component$(() => {
  const nav = useNavigate();
  const email = useSignal("");
  const password = useSignal("");
  const error = useSignal("");
  const busy = useSignal(false);

  return (
    <main class="auth-page">
      <div class="auth-page__card">
        <Link class="auth-page__brand" href="/" aria-label={BRAND_NAME_PLAIN}>
          <BrandName />
        </Link>
        <h1 class="auth-page__title">Crear cuenta</h1>
        <p class="auth-page__lead">
          Registrate con correo y contraseña (mínimo 8 caracteres).
        </p>
        <form
          class="auth-form"
          preventdefault:submit
          onSubmit$={async () => {
            error.value = "";
            busy.value = true;
            try {
              const result = await registerUser(email.value, password.value);
              storeToken(result.token);
              await nav("/workspace");
            } catch (e) {
              error.value =
                e instanceof Error ? e.message : "No se pudo crear la cuenta";
            } finally {
              busy.value = false;
            }
          }}
        >
          <label class="auth-form__label" for="register-email">
            Correo
          </label>
          <input
            id="register-email"
            class="auth-form__input"
            type="email"
            autocomplete="email"
            required
            value={email.value}
            onInput$={(_, el) => {
              email.value = el.value;
            }}
          />
          <label class="auth-form__label" for="register-password">
            Contraseña
          </label>
          <input
            id="register-password"
            class="auth-form__input"
            type="password"
            autocomplete="new-password"
            required
            minLength={8}
            value={password.value}
            onInput$={(_, el) => {
              password.value = el.value;
            }}
          />
          {error.value && (
            <p class="auth-form__error" role="alert">
              {error.value}
            </p>
          )}
          <button
            type="submit"
            class="auth-form__submit"
            disabled={busy.value}
          >
            {busy.value ? "Creando…" : "Crear cuenta"}
          </button>
        </form>
        <p class="auth-page__switch">
          ¿Ya tenés cuenta?{" "}
          <Link href="/login">Iniciar sesión</Link>
        </p>
      </div>
    </main>
  );
});

export const head: DocumentHead = {
  title: `Crear cuenta · ${BRAND_NAME_PLAIN}`,
};
