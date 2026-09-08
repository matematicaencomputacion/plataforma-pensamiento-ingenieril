## 1. Backend abuse protection

- [ ] 1.1 Implement a bounded concurrency-safe login limiter and verify allow, block, expiry, reset and capacity unit tests pass
- [ ] 1.2 Implement trusted-proxy-aware client identity and hashed normalized-email keys and verify direct, trusted and malformed-header cases
- [ ] 1.3 Integrate `429` and ceiling-rounded `Retry-After` into the login handler and verify handler tests cover blocking and success reset
- [ ] 1.4 Wire configurable production defaults and the Cloud Run proxy flag and verify configuration and backend suites pass

## 2. Safe login continuation

- [ ] 2.1 Add pure validation and encoding helpers for internal `return_to` destinations and verify Rust unit tests cover safe, malicious and malformed inputs
- [ ] 2.2 Update every protected guard and login navigation and verify E2E covers destination preservation and unsafe fallback

## 3. Recoverable session bootstrap

- [ ] 3.1 Add explicit session restoration states and retry behavior and verify rejected tokens clear while transient failures retain the token
- [ ] 3.2 Add an accessible recovery notice and verify E2E restores the session after an `/api/me` retry without credential entry

## 4. Integrated validation and delivery

- [ ] 4.1 Run strict OpenSpec validation, Go tests, Rust tests, Wasm build and canonical Playwright journeys and record passing evidence
- [ ] 4.2 Commit, push and open a structured PR, then verify CI, Docker and E2E checks without merging
