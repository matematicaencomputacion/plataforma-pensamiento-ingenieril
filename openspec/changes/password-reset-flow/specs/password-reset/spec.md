## ADDED Requirements

### Requirement: Request password recovery
The system SHALL accept `POST /api/auth/forgot-password` with `{ "email" }` and
ALWAYS respond HTTP 200 with a generic message that does not disclose whether
the email is registered. When the email belongs to an existing user, the system
MUST create a single-use reset token with a bounded TTL and store only a
one-way hash of that token. When reset-token exposure is enabled for non-prod
DX, the response MAY include the plaintext `resetToken` if and only if a user
matched.

#### Scenario: Forgot known email (exposure on)
- **GIVEN** a registered user and reset-token exposure enabled
- **WHEN** a client posts that email to `POST /api/auth/forgot-password`
- **THEN** the system responds 200 with a generic `message` and a non-empty `resetToken`

#### Scenario: Forgot unknown email
- **WHEN** a client posts an unregistered email
- **THEN** the system responds 200 with the same style of generic `message` and WITHOUT `resetToken`

#### Scenario: Forgot with exposure disabled
- **GIVEN** a registered user and exposure disabled
- **WHEN** a client requests forgot-password
- **THEN** the system responds 200 without `resetToken`

### Requirement: Reset password with valid token
The system SHALL accept `POST /api/auth/reset-password` with `{ "token", "password" }`,
validate that the token hash exists, is unused, and is not expired, enforce the
password policy (minimum 8 characters), replace the stored password hash, mark
the token as used, and return HTTP 200 with `{ user, token }` like login.

#### Scenario: Successful reset
- **GIVEN** a valid unused reset token
- **WHEN** a client submits that token and a valid new password
- **THEN** the system updates the password, invalidates the token, and returns 200 with session credentials
- **AND** subsequent login with the old password fails while the new password succeeds

#### Scenario: Expired or unknown token
- **WHEN** a client submits an unknown, used, or expired token
- **THEN** the system responds HTTP 400 with a clear error and does not change any password

#### Scenario: Weak new password
- **WHEN** a client submits a valid token but a password shorter than 8 characters
- **THEN** the system responds HTTP 400 and does not consume the token

### Requirement: Leptos recovery UI
The Leptos CSR shell SHALL expose `/forgot-password` (email request) linked from
login, and `/reset-password` (new password, token from query string or input).
Both views MUST call the Go endpoints above and surface errors via accessible alerts.

#### Scenario: Navigate from login
- **WHEN** a learner opens `/login`
- **THEN** they can reach `/forgot-password` via an explicit link

#### Scenario: Complete reset in UI
- **GIVEN** a `resetToken` from the forgot response (dev exposure)
- **WHEN** the learner submits a new password on `/reset-password`
- **THEN** they land authenticated on `/workspace`
