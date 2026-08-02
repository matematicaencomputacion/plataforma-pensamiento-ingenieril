# core-learning-engine

Motor de Aprendizaje Dual y Perfil Cognitivo.

## Artefactos

| Archivo | Rol |
|---------|-----|
| `proposal.md` | Problema, objetivo, alcance, capacidades e impacto |
| `specs/learning-tracks/spec.md` | Delta: enunciados por track |
| `specs/cognitive-profile/spec.md` | Delta: perfil cognitivo en evaluación |
| `design.md` | Decisiones técnicas (Go + Qwik) |
| `tasks.md` | Checklist ejecutable |

## Flujo

1. Revisar/ajustar proposal + specs.
2. Validar: `openspec validate core-learning-engine`
3. Implementar con `/opsx:apply` siguiendo `tasks.md`.
4. Archivar con `/opsx:archive` cuando el change esté completo.
