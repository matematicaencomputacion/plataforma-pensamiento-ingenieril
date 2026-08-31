## Purpose

Define un catálogo curricular temporalmente estable y verificable para que navegación, progreso, particiones conceptuales y automatización compartan una única frontera canónica.

## ADDED Requirements

### Requirement: Catálogo canónico continuo hasta 2500
El sistema SHALL exponer exactamente los micro-pasos `1..=2500`, sin identificadores ni posiciones duplicadas, y MUST mantener una cadena navegable continua cuyo último paso no tenga sucesor.

#### Scenario: Catálogo completo
- **GIVEN** una compilación del frontend con el catálogo estabilizado
- **WHEN** se enumeran los micro-pasos disponibles
- **THEN** existen exactamente 2500 entradas con posiciones únicas de 1 a 2500

#### Scenario: Frontera del rail
- **GIVEN** el micro-paso 2500
- **WHEN** se consulta su sucesor
- **THEN** el sistema informa que no existe un paso siguiente

### Requirement: Contratos derivados alineados
Los tests unitarios, conceptuales y E2E MUST usar 2500 como frontera del catálogo y SHALL fallar si una expectativa activa referencia contenido posterior.

#### Scenario: Render del workspace
- **GIVEN** un usuario autenticado que abre el workspace
- **WHEN** el rail curricular termina de renderizar
- **THEN** la interfaz contiene exactamente 2500 celdas de micro-paso

#### Scenario: Particiones dentro de catálogo
- **GIVEN** una asociación entre un micro-paso y una partición conceptual
- **WHEN** se valida el índice conceptual
- **THEN** el micro-paso asociado existe dentro de `1..=2500`

### Requirement: Herramientas de contenido coherentes
Las herramientas activas de generación, aplicación y validación MUST corresponder únicamente a olas incluidas en la frontera canónica o estar explícitamente retiradas del árbol activo.

#### Scenario: Validación reproducible
- **GIVEN** un checkout limpio del change
- **WHEN** se ejecutan los validadores activos de las olas conservadas
- **THEN** no requieren módulos eliminados ni generan micro-pasos posteriores a 2500

### Requirement: Gates obligatorios de estabilización
El cambio MUST superar la validación OpenSpec, las suites unitarias de backend y frontend y los journeys de ADR 003 antes de considerarse entregable.

#### Scenario: Hito listo para PR
- **GIVEN** la implementación completa del rollback
- **WHEN** se ejecuta el pipeline de validación acordado
- **THEN** todos los gates finalizan correctamente o el cambio permanece incompleto con el bloqueo documentado
