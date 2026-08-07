## ADDED Requirements

### Requirement: User registration with email and password
The system SHALL allow a new learner to register with a unique email address and a password that meets the minimum length policy (at least 8 characters). Passwords MUST be stored only as irreversible hashes. On success the API MUST return a public user representation (`id`, `email`) and an authentication token.

#### Scenario: Successful registration
- **WHEN** a client submits a previously unused email and a valid password to `POST /api/auth/register`
- **THEN** the system creates the user, returns HTTP 201 with `{ user, token }`, and does not return the password or password hash

#### Scenario: Duplicate email
- **WHEN** a client registers with an email that already exists
- **THEN** the system responds HTTP 409 and does not create a second account

#### Scenario: Invalid password
- **WHEN** a client registers with a password shorter than 8 characters
- **THEN** the system responds HTTP 400 and does not create a user

### Requirement: User login with email and password
The system SHALL authenticate registered users by email and password. On success it MUST return a public user representation and an authentication token. Failed attempts MUST use a generic error message that does not reveal whether the email exists.

#### Scenario: Successful login
- **WHEN** a client submits correct email and password to `POST /api/auth/login`
- **THEN** the system responds HTTP 200 with `{ user, token }`

#### Scenario: Invalid credentials
- **WHEN** a client submits a wrong password or unknown email to `POST /api/auth/login`
- **THEN** the system responds HTTP 401 with a generic “credenciales inválidas” (or equivalent) message

### Requirement: Authenticated current user
The system SHALL expose `GET /api/me` that returns the public user for a valid Bearer JWT and rejects missing or invalid tokens.

#### Scenario: Valid token
- **WHEN** a client calls `GET /api/me` with a valid `Authorization: Bearer` token
- **THEN** the system responds HTTP 200 with `{ id, email }`

#### Scenario: Missing or invalid token
- **WHEN** a client calls `GET /api/me` without a Bearer token or with an expired/invalid token
- **THEN** the system responds HTTP 401

### Requirement: Client session lifecycle
The Qwik frontend SHALL persist the auth token after successful register/login, attach it to authenticated API calls (`/api/me`), and clear it on logout so the user returns to an unauthenticated state.

#### Scenario: Logout
- **WHEN** an authenticated user chooses logout
- **THEN** the client discards the token and subsequent visits to protected account views behave as logged out

#### Scenario: Session restore
- **WHEN** a user with a stored valid token opens the app
- **THEN** the client can resolve the current user via `GET /api/me` without asking for credentials again
