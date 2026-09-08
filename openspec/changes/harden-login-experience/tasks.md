## 1. Backend abuse protection

- [x] 1.1 Implement a bounded concurrency-safe login limiter and verify allow, block, expiry, reset and capacity unit tests pass
- [x] 1.2 Implement server-peer identity that ignores spoofable forwarding headers plus hashed normalized-email keys and verify both properties
- [x] 1.3 Integrate `429` and ceiling-rounded `Retry-After` into the login handler and verify handler tests cover blocking and success reset
- [x] 1.4 Wire configurable production defaults and verify configuration and backend suites pass

## 2. Safe login continuation

- [x] 2.1 Add pure validation and encoding helpers for internal `return_to` destinations and verify Rust unit tests cover safe, malicious and malformed inputs
- [x] 2.2 Update every protected guard and login navigation and verify E2E covers destination preservation and unsafe fallback

## 3. Recoverable session bootstrap

- [x] 3.1 Add explicit session restoration states and retry behavior and verify rejected tokens clear while transient failures retain the token
- [x] 3.2 Add an accessible recovery notice and verify E2E restores the session after an `/api/me` retry without credential entry

## 4. Integrated validation and delivery

- [x] 4.1 Run strict OpenSpec validation, Go tests, Rust tests, Wasm build and canonical Playwright journeys and record passing evidence
- [ ] 4.2 Commit, push and open a structured PR, then verify CI, Docker and E2E checks without merging
