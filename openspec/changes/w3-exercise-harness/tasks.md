## 1. Semilla y tipos

- [x] 1.1 Copiar `docs/seeds/python-foundations-microsteps-v0.2.json` a `frontend/src/data/python-foundations-microsteps.json`
- [x] 1.2 Definir tipos TypeScript del seed (`metadata`, `steps`, `checks`, MCQ) en `frontend/src/lib/microsteps/`
- [x] 1.3 Implementar loader/normalizer que adapta placeholders `exec(open('solution.py'))` a payload `{ studentCode, testSource }`
- [x] 1.4 Tests unitarios del loader/normalizer (resolver por `id`, fallback a primer step, frontera Casting)

## 2. Ruta Exercise Workspace

- [x] 2.1 Crear rama `feat/w3-exercise-harness` desde main limpia (si aún no existe)
- [x] 2.2 Añadir ruta Qwik City `/exercise` con query `?step=<id>`
- [x] 2.3 Layout single-viewport: theory | editor | results + header (título, Hint, Solución, Salir)
- [x] 2.4 CTA “Micro-pasos Python” en el home hacia `/exercise?step=py-01-home` sin montar InteractiveStage en esa ruta
- [x] 2.5 Navegación Continuar → `next` solo si checks required pasan; estado completado en frontera Casting

## 3. Runner client-side

- [ ] 3.1 Editor MVP (textarea tipado o Monaco si ya está disponible) inicializado con `starter_code`
- [ ] 3.2 Lazy-load de Pyodide al entrar a `/exercise` o al primer Run, con UI “Preparando motor Python…”
- [ ] 3.3 Acción Run: ejecutar código del alumno y mostrar stdout/stderr en el panel de resultados
- [ ] 3.4 Acción Check: evaluar `testSource` en browser y reportar pass/fail; no llamar API Go de ejecución
- [ ] 3.5 Acciones Hint / Show Solution revelan `hint` y `solution_example` en sesión local

## 4. MCQ y pulido UX

- [ ] 4.1 Renderizar MCQ cuando exista `checks.mcq` o `content.mcq_bank` e integrarlo según `checks.mode`
- [ ] 4.2 Responsive: stack vertical theory → editor → checks en viewport estrecho
- [ ] 4.3 Estado local `useStore` (step, código, pass/fail, hint/solution visibles); sin persistencia backend

## 5. Validación y cierre

- [ ] 5.1 Smoke manual de los 10 steps (HOME → Casting) en el harness
- [ ] 5.2 `cd frontend && npm run build` y lint sin errores
- [ ] 5.3 PR `feat/w3-exercise-harness` con checklist Pre-CI (sin merge a main salvo pedido)
