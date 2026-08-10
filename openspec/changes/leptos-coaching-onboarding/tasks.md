## 1. OpenSpec & routing shell

- [x] 1.1 Crear change `leptos-coaching-onboarding` (proposal/design/spec/tasks)
- [x] 1.2 Ruta Leptos `/onboarding` + guard de sesión + página shell (prompts + notes)
- [x] 1.3 CTA en Workspace hub → `/onboarding`
- [x] 1.4 Estilos `.onboarding*` coherentes con el design system
- [x] 1.5 E2E smoke: auth → workspace → onboarding title visible
- [x] 1.6 `make harness` verde + PR TED atómico (shell)

## 2. Synthesize (rebanada siguiente)

- [ ] 2.1 Cliente Wasm `POST /api/learner/profile/synthesize`
- [ ] 2.2 Estados analyzing/error + transición a `reviewing`
- [ ] 2.3 Profile builder (4 campos) editable en review
- [ ] 2.4 Tests unitarios contratos JSON + E2E analyze con LLM mock

## 3. Persist & continue

- [ ] 3.1 GET/PUT `/api/user/profile` + dirty check
- [ ] 3.2 Estado `saved` + CTA continuar (href placeholder Paso 2)
- [ ] 3.3 E2E persist smoke (Bearer)

## 4. Hardening

- [ ] 4.1 A11y/aria + network copy (paridad Rebanada 2)
- [ ] 4.2 SpeechRecognition opcional (solo browser)
- [ ] 4.3 Sync prompts desde seed / docs
