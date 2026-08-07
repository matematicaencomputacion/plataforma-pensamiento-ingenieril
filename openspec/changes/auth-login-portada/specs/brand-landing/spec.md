## ADDED Requirements

### Requirement: Branded public landing as entry surface
The system SHALL present an unauthenticated visitor with a brand-first landing at `/` featuring the product name **IngenierIA** as a hero-level signal, one primary headline, one short supporting sentence, and a CTA group for registration and login. The first viewport MUST read as one branded composition (not a dashboard or exercise workspace).

#### Scenario: Anonymous visits root
- **WHEN** an unauthenticated user navigates to `/`
- **THEN** they see the landing (brand, headline, support line, register/login CTAs) and NOT the coding workspace editor as the primary surface

### Requirement: Clear paths into auth and the learning product
The landing SHALL provide primary CTAs to register and to log in. It MAY include a secondary CTA to try the learning harness without an account; if present, that path MUST be visually secondary to auth CTAs.

#### Scenario: Navigate to register
- **WHEN** the user activates the register CTA on the landing
- **THEN** they are taken to the registration view (`/register` or equivalent)

#### Scenario: Navigate to login
- **WHEN** the user activates the login CTA on the landing
- **THEN** they are taken to the login view (`/login` or equivalent)

### Requirement: Auth screens match brand
Login and registration screens SHALL reuse the product visual language (typography/colors already used by IngenierIA) and MUST collect email and password without third-party OAuth buttons in this milestone.

#### Scenario: Register form fields
- **WHEN** a user opens the registration view
- **THEN** they can enter email and password and submit to create an account

#### Scenario: Login form fields
- **WHEN** a user opens the login view
- **THEN** they can enter email and password and submit to authenticate

### Requirement: Post-auth destination
After successful login or registration, the frontend SHALL navigate the user into the learning product entry (micro-pasos harness or workspace route defined in design), not leave them on the landing without feedback.

#### Scenario: Redirect after login
- **WHEN** login succeeds
- **THEN** the user is redirected to the configured learning entry route
