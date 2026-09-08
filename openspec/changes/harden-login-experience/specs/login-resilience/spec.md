## Purpose

Garantiza que el acceso autenticado sea resistente a abuso, conserve la intención de navegación y no expulse al alumno por indisponibilidades temporales.

## ADDED Requirements

### Requirement: Login throttles repeated attempts
The system SHALL limit repeated login attempts using both a server-derived client identity and normalized email, MUST keep limiter memory bounded, and MUST NOT reveal whether the account exists.

#### Scenario: Attempt budget exhausted
- **WHEN** a client exceeds the configured login attempt budget for one normalized email
- **THEN** the API responds with status `429`, a `Retry-After` header rounded up to the remaining blocking interval, and a generic retry message

#### Scenario: Successful login resets the budget
- **WHEN** a valid login succeeds before the blocking window expires
- **THEN** the attempt budget for that client and email is reset

#### Scenario: Limiter receives unbounded distinct keys
- **WHEN** distinct client and email combinations exceed the configured limiter capacity
- **THEN** the limiter rejects previously unseen keys until capacity expires, retains no more than its configured maximum and does not evict an active blocked key

#### Scenario: Forwarding headers are attacker controlled
- **WHEN** a login request supplies `X-Forwarded-For` or another forwarding header
- **THEN** the limiter ignores it and derives network identity only from the server-controlled peer address

### Requirement: Authentication preserves a safe internal destination
The web application SHALL return an authenticated user to the requested internal route and MUST reject external, protocol-relative, malformed or authentication-loop destinations.

#### Scenario: Protected route requires login
- **WHEN** an unauthenticated user opens a protected route
- **THEN** the application navigates to login carrying that internal route as `return_to`

#### Scenario: Safe destination after successful login
- **WHEN** login succeeds with a valid internal `return_to`
- **THEN** the application navigates to that destination instead of the default workspace

#### Scenario: Unsafe destination is supplied
- **WHEN** login receives an external, protocol-relative, malformed or authentication-page `return_to`
- **THEN** the application ignores it and navigates to `/workspace`

### Requirement: Transient session restoration is recoverable
The web application SHALL distinguish temporary `/api/me` failures from rejected credentials, retain the stored token during temporary failures and offer an explicit retry.

#### Scenario: Session API is temporarily unavailable
- **WHEN** a stored token exists and `/api/me` fails without status `401` or `403`
- **THEN** the application keeps the token, shows a recoverable session state and does not redirect to login

#### Scenario: Session retry succeeds
- **WHEN** the user retries restoration and `/api/me` succeeds
- **THEN** the application restores the authenticated user without requiring credentials again

#### Scenario: Stored token is rejected
- **WHEN** `/api/me` responds with status `401` or `403`
- **THEN** the application clears authentication state and permits the route guard to redirect to login
