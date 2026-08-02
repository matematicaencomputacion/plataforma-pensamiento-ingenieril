## 1. Dominio y contratos

- [x] 1.1 Definir structs de dominio `Challenge`/`Level` (id, title, statement, trackType, evaluationPrompt) en `backend/internal/domain`
- [x] 1.2 Definir structs `CognitiveProfile` y `Skill` (id, status, lastReviewedAt) en `backend/internal/domain`
- [x] 1.3 Definir interfaces de repositorio para niveles y perfil en la capa adecuada (puertos)

## 2. Backend — datos y casos de uso

- [x] 2.1 Implementar repositorio in-memory/SQLite con seed: 1 micro_paso + 1 reto_ingenieril + perfil demo
- [x] 2.2 Caso de uso `GetCurrentLevel` y handler `GET /api/levels/current` (+ `GET /api/levels/{id}`)
- [x] 2.3 Extender `EvaluationService` para componer system prompt (track + perfil cognitivo + reglas JSON)
- [x] 2.4 Actualizar tests unitarios de evaluación/handlers con mocks HTTP y perfil seeded
- [x] 2.5 Registrar rutas en `main.go` y verificar CORS

## 3. Frontend — enunciado y evaluación

- [ ] 3.1 Fetch de nivel actual al cargar la vista del estudiante
- [ ] 3.2 Render condicional de tarjeta de enunciado (micro_paso vs reto_ingenieril)
- [ ] 3.3 Mantener flujo de evaluación + feedback del profesor con el `level_id` del nivel cargado
- [ ] 3.4 Estados de carga/error de enunciado con `useSignal`/`useStore`

## 4. Validación SDD

- [ ] 4.1 Cubrir escenarios de `learning-tracks` y `cognitive-profile` con pruebas automatizadas o checklist manual documentado
- [ ] 4.2 Ejecutar `openspec validate core-learning-engine` en verde
- [ ] 4.3 Commit convencional del change implementado (fuera de este scaffolding)
