## ADDED Requirements

### Requirement: Sintetizar perfil de aprendiz desde texto libre
El sistema SHALL aceptar el relato de onboarding del alumno y devolver una síntesis estructurada con las claves `purpose`, `urgency`, `vision` y `stack` en español, usando un clasificador LLM (xAI Grok, API OpenAI-compatible) o un mock configurable.

#### Scenario: Clasificación exitosa vía API
- **WHEN** el cliente envía `POST /api/learner/profile/synthesize` con `raw_notes` de longitud ≥ 12 y el proveedor LLM responde JSON válido
- **THEN** el sistema responde HTTP 200 con las cuatro claves de síntesis (strings; vacías solo si el modelo no detectó señal)

#### Scenario: Texto insuficiente
- **WHEN** el cliente envía `raw_notes` vacías o con menos de 12 caracteres no-espacio
- **THEN** el sistema responde HTTP 400 y no invoca al proveedor LLM

#### Scenario: Fallo del proveedor
- **WHEN** el proveedor LLM no está disponible o devuelve contenido no parseable
- **THEN** el sistema responde HTTP 502 con mensaje genérico seguro (sin filtrar secretos) y el frontend permanece en `drafting` con error visible

### Requirement: Modo mock para desarrollo offline
El sistema SHALL permitir `LEARNER_PROFILE_LLM=mock` para clasificar con el motor por keywords sin llamar a xAI.

#### Scenario: Mock activo
- **WHEN** `LEARNER_PROFILE_LLM=mock` y el cliente solicita síntesis
- **THEN** el sistema usa el clasificador local por keywords y no realiza llamadas de red a xAI

### Requirement: Onboarding usa síntesis bajo demanda
El frontend de onboarding SHALL invocar la API de síntesis solo al confirmar “Enviar para análisis”, y solo entonces transiciónar a `reviewing` si la respuesta es exitosa.

#### Scenario: Enviar para análisis
- **WHEN** el alumno en estado `drafting` confirma “Enviar para análisis” con texto suficiente
- **THEN** la UI muestra estado de carga, llama a `/api/learner/profile/synthesize`, puebla las tarjetas con la respuesta y pasa a `reviewing`
