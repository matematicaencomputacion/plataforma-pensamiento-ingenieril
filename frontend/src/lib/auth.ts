import { API_BASE_URL } from "./api";

export const AUTH_TOKEN_KEY = "ppi.auth.token";

export type AuthUser = {
  id: string;
  email: string;
};

export type AuthSuccess = {
  user: AuthUser;
  token: string;
};

type AuthErrorBody = {
  error?: string;
};

function isBrowser(): boolean {
  return typeof window !== "undefined" && typeof localStorage !== "undefined";
}

export function getStoredToken(): string | null {
  if (!isBrowser()) {
    return null;
  }
  return localStorage.getItem(AUTH_TOKEN_KEY);
}

export function storeToken(token: string): void {
  if (!isBrowser()) {
    return;
  }
  localStorage.setItem(AUTH_TOKEN_KEY, token);
}

export function clearToken(): void {
  if (!isBrowser()) {
    return;
  }
  localStorage.removeItem(AUTH_TOKEN_KEY);
}

async function readError(res: Response): Promise<string> {
  try {
    const body = (await res.json()) as AuthErrorBody;
    if (body.error) {
      return body.error;
    }
  } catch {
    /* ignore */
  }
  return `Error HTTP ${res.status}`;
}

export async function registerUser(
  email: string,
  password: string,
): Promise<AuthSuccess> {
  const res = await fetch(`${API_BASE_URL}/api/auth/register`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ email, password }),
  });
  if (!res.ok) {
    throw new Error(await readError(res));
  }
  return (await res.json()) as AuthSuccess;
}

export async function loginUser(
  email: string,
  password: string,
): Promise<AuthSuccess> {
  const res = await fetch(`${API_BASE_URL}/api/auth/login`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ email, password }),
  });
  if (!res.ok) {
    throw new Error(await readError(res));
  }
  return (await res.json()) as AuthSuccess;
}

export async function fetchMe(token: string): Promise<AuthUser> {
  const res = await fetch(`${API_BASE_URL}/api/me`, {
    headers: { Authorization: `Bearer ${token}` },
  });
  if (!res.ok) {
    throw new Error(await readError(res));
  }
  return (await res.json()) as AuthUser;
}

export async function logoutRemote(token: string | null): Promise<void> {
  try {
    await fetch(`${API_BASE_URL}/api/auth/logout`, {
      method: "POST",
      headers: token ? { Authorization: `Bearer ${token}` } : undefined,
    });
  } catch {
    /* cliente-stateless: igual limpiamos local */
  }
}

export async function logoutSession(): Promise<void> {
  const token = getStoredToken();
  await logoutRemote(token);
  clearToken();
}
