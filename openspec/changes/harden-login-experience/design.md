## Context

See `proposal.md`. The Go service currently delegates every login directly to the auth use case. The Leptos CSR shell stores a bearer token locally, redirects all protected-route failures to bare `/login`, and clears in-memory state on any `/api/me` error. Cloud Run terminates requests behind Google infrastructure, so client identity cannot rely blindly on the container peer address.

## Goals / Non-Goals

**Goals:**

- Add deterministic, concurrency-safe abuse protection at the HTTP boundary with a strict memory ceiling.
- Centralize validation and encoding of post-login destinations.
- Model session restoration as explicit states so route guards can distinguish anonymous from temporarily unavailable.

**Non-Goals:**

- Distributed counters, durable lockouts or account-level security policy.
- Cookie/session transport migration.

## Decisions

### Bounded in-memory fixed-window limiter

The handler receives a typed limiter port and production injects a mutex-protected fixed-window implementation. The key combines server-derived peer identity with a SHA-256 digest of normalized email; raw email and password never enter limiter state or logs. Success resets the key. Expired entries are removed before insertion and, at capacity, unknown keys fail closed until the earliest active entry expires. Existing blocked keys are never evicted by attacker-controlled key churn. A fixed window is simple and testable; token bucket and Redis were rejected for this first increment because they add infrastructure not required by the contract.

The default handler constructor keeps a no-op limiter for isolated legacy tests; production composition explicitly injects the bounded limiter.

### Server-controlled network identity

Client identity is always derived from `RemoteAddr`; forwarding headers are ignored. Google Cloud Load Balancing preserves caller-supplied `X-Forwarded-For` prefixes, so accepting a convenient header position without a separately configured trusted-hop boundary would let an attacker rotate identities. Behind Cloud Run, the peer may be shared and the normalized-email digest therefore carries most of the key separation. This is a deliberate fail-closed trade-off until a verified edge header or distributed limiter is introduced.

### Strict allow-list semantics for `return_to`

A pure frontend helper accepts only absolute-path references beginning with exactly one `/`, rejects backslashes, control characters and authentication routes, and percent-encodes the value when building the login URL. Guards supply their current canonical path including query. Successful login re-validates the query value before navigation.

### Explicit session restoration status

`SessionCtx` adds loading, authenticated, anonymous and temporarily unavailable states. A retry method reuses the stored/in-memory token. Unauthorized responses clear storage and become anonymous; network, 5xx and invalid-response failures retain the token and become temporarily unavailable. A global recovery notice provides the retry action, while route guards redirect only when the token is absent.

## Risks / Trade-offs

- [Counters are per Cloud Run instance] → Keep the limiter port replaceable by a distributed adapter and document this first-layer limitation.
- [Cloud Run peers can be shared] → Combine peer identity with normalized email and prefer a conservative account-scoped budget over trusting spoofable forwarding input.
- [Capacity exhaustion can reject previously unseen keys] → Fail closed only until the earliest active window expires; never evict a blocked key.
- [A retained invalid token may briefly hold the UI] → Only non-auth failures retain it; a later `401/403` clears it deterministically.

## Migration Plan

1. Deploy additive frontend state and backend limiter together; no data migration is required.
2. Observe `429` rates and session-retry behavior through existing platform logs and tests.
3. Roll back by reverting the change; no persisted limiter/session schema remains.
