import {
  component$,
  useSignal,
  useVisibleTask$,
  $,
} from "@builder.io/qwik";
import { Link, useNavigate } from "@builder.io/qwik-city";
import {
  clearToken,
  fetchMe,
  getStoredToken,
  logoutRemote,
  type AuthUser,
} from "../../lib/auth";
import { BrandName } from "../brand-name/brand-name";

export type SessionBarProps = {
  /** Si true, enlaces de auth/producto en estilo lander. */
  variant?: "landing" | "app";
};

/**
 * Barra de sesión: CTAs login/registro o email + Salir.
 * Solo hidrata el usuario en el cliente (token en localStorage).
 */
export const SessionBar = component$((props: SessionBarProps) => {
  const user = useSignal<AuthUser | null>(null);
  const ready = useSignal(false);
  const nav = useNavigate();
  const variant = props.variant ?? "app";

  // eslint-disable-next-line qwik/no-use-visible-task
  useVisibleTask$(async () => {
    const token = getStoredToken();
    if (!token) {
      ready.value = true;
      return;
    }
    try {
      user.value = await fetchMe(token);
    } catch {
      clearToken();
      user.value = null;
    } finally {
      ready.value = true;
    }
  });

  const logout$ = $(async () => {
    const token = getStoredToken();
    await logoutRemote(token);
    clearToken();
    user.value = null;
    await nav("/");
  });

  return (
    <nav
      class={`session-bar session-bar--${variant}`}
      aria-label="Cuenta"
    >
      {!ready.value ? (
        <span class="session-bar__muted">…</span>
      ) : user.value ? (
        <>
          <span class="session-bar__email">{user.value.email}</span>
          <button
            type="button"
            class="session-bar__btn session-bar__btn--ghost"
            onClick$={logout$}
          >
            Salir
          </button>
        </>
      ) : (
        <>
          <Link class="session-bar__btn session-bar__btn--ghost" href="/login">
            Iniciar sesión
          </Link>
          <Link class="session-bar__btn session-bar__btn--primary" href="/register">
            Crear cuenta
          </Link>
        </>
      )}
    </nav>
  );
});

/** Marca + session bar para headers de app. */
export const AppHeaderChrome = component$(() => {
  return (
    <div class="app-chrome">
      <Link class="app-chrome__brand" href="/" aria-label="IngenierIA — inicio">
        <BrandName />
      </Link>
      <SessionBar variant="app" />
    </div>
  );
});
