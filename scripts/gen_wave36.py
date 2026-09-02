"""Generate Wave 36: deterministic safe-delivery exercises."""

from gen_wave35 import exercise, emit_refs, emit_rust


CASES = [
    # 1. Release contracts
    ("release-manifiesto", "release · manifiesto", "Construir un manifiesto de entrega.", "resultado = {'version': '2.1.0', 'artefactos': ['api', 'web']}\nprint(resultado)", {"version": "2.1.0", "artefactos": ["api", "web"]}, "contracts"),
    ("release-ordenar", "release · ordenar", "Ordenar artefactos reproduciblemente.", "artefactos = ['web', 'api', 'worker']\nresultado = sorted(artefactos)\nprint(resultado)", ["api", "web", "worker"], "contracts"),
    ("release-requeridos", "release · requeridos", "Validar campos obligatorios.", "manifiesto = {'version': '2.1.0', 'revision': 'abc'}\nresultado = {'version', 'revision'} <= manifiesto.keys()\nprint(resultado)", True, "contracts"),
    ("release-diferencia", "release · diferencia", "Detectar artefactos cambiados.", "anterior = {'api': 'a1', 'web': 'w1'}; nuevo = {'api': 'a2', 'web': 'w1'}\nresultado = sorted(k for k in nuevo if nuevo[k] != anterior.get(k))\nprint(resultado)", ["api"], "contracts"),
    ("release-aprobaciones", "release · aprobaciones", "Comprobar aprobaciones requeridas.", "requeridas = {'qa', 'producto'}; recibidas = {'producto', 'qa', 'ops'}\nresultado = requeridas <= recibidas\nprint(resultado)", True, "contracts"),
    ("release-suite", "release · suite", "Validar un contrato completo de entrega.", "m = {'version': '3.0.0', 'revision': 'c7', 'artefactos': ['web', 'api'], 'aprobaciones': ['qa', 'producto']}\nresultado = {'valido': {'version', 'revision', 'artefactos'} <= m.keys(), 'artefactos': sorted(m['artefactos']), 'aprobado': {'qa', 'producto'} <= set(m['aprobaciones'])}\nprint(resultado)", {"valido": True, "artefactos": ["api", "web"], "aprobado": True}, "contracts"),

    # 2. Semantic versions
    ("version-parsear", "versión · parsear", "Convertir una versión en componentes.", "resultado = tuple(map(int, '2.4.1'.split('.')))\nprint(resultado)", (2, 4, 1), "versions"),
    ("version-comparar", "versión · comparar", "Comparar versiones por componentes.", "actual = (2, 3, 9); candidata = (2, 4, 0)\nresultado = candidata > actual\nprint(resultado)", True, "versions"),
    ("version-mayor", "versión · mayor", "Clasificar un cambio mayor.", "antes = (2, 8, 1); despues = (3, 0, 0)\nresultado = despues[0] > antes[0]\nprint(resultado)", True, "versions"),
    ("version-menor", "versión · menor", "Calcular la siguiente versión menor.", "mayor, menor, parche = (2, 4, 7)\nresultado = (mayor, menor + 1, 0)\nprint(resultado)", (2, 5, 0), "versions"),
    ("version-seleccionar", "versión · seleccionar", "Elegir la versión máxima compatible.", "versiones = [(1, 9, 0), (2, 0, 0), (1, 10, 2)]; mayor = 1\nresultado = max(v for v in versiones if v[0] == mayor)\nprint(resultado)", (1, 10, 2), "versions"),
    ("version-suite", "versión · suite", "Ordenar y clasificar una evolución semántica.", "textos = ['2.1.0', '1.9.4', '2.0.3']; versiones = [tuple(map(int, x.split('.'))) for x in textos]\norden = sorted(versiones); anterior, nueva = orden[-2:]\nresultado = {'orden': orden, 'tipo': 'menor' if nueva[0] == anterior[0] and nueva[1] > anterior[1] else 'otro'}\nprint(resultado)", {"orden": [(1, 9, 4), (2, 0, 3), (2, 1, 0)], "tipo": "menor"}, "versions"),

    # 3. Compatibility
    ("compatibilidad-campos", "compatibilidad · campos", "Aceptar campos adicionales.", "requeridos = {'id', 'nombre'}; recibido = {'id': 1, 'nombre': 'Ada', 'extra': True}\nresultado = requeridos <= recibido.keys()\nprint(resultado)", True, "compatibility"),
    ("compatibilidad-default", "compatibilidad · default", "Completar un campo opcional.", "entrada = {'id': 7}\nresultado = {**entrada, 'modo': entrada.get('modo', 'estable')}\nprint(resultado)", {"id": 7, "modo": "estable"}, "compatibility"),
    ("compatibilidad-lector", "compatibilidad · lector", "Elegir lectores compatibles.", "lectores = {'v1': {1}, 'v2': {1, 2}, 'v3': {2, 3}}; formato = 2\nresultado = sorted(k for k, formatos in lectores.items() if formato in formatos)\nprint(resultado)", ["v2", "v3"], "compatibility"),
    ("compatibilidad-matriz", "compatibilidad · matriz", "Construir una matriz de compatibilidad.", "productores = {'p1': 1, 'p2': 2}; consumidores = {'c1': {1, 2}, 'c2': {2}}\nresultado = {p: sorted(c for c, soporta in consumidores.items() if v in soporta) for p, v in sorted(productores.items())}\nprint(resultado)", {"p1": ["c1"], "p2": ["c1", "c2"]}, "compatibility"),
    ("compatibilidad-bloqueos", "compatibilidad · bloqueos", "Detectar consumidores incompatibles.", "version = 3; soporte = {'a': {2, 3}, 'b': {1, 2}, 'c': {3}}\nresultado = sorted(k for k, versiones in soporte.items() if version not in versiones)\nprint(resultado)", ["b"], "compatibility"),
    ("compatibilidad-suite", "compatibilidad · suite", "Validar productores y consumidores antes de entregar.", "formatos = {'api': 2, 'eventos': 3}; soporte = {'web': {1, 2}, 'worker': {2, 3}}\nresultado = {origen: sorted(c for c, versiones in soporte.items() if version in versiones) for origen, version in sorted(formatos.items())}\nprint(resultado)", {"api": ["web", "worker"], "eventos": ["worker"]}, "compatibility"),

    # 4. Feature flags
    ("flag-activa", "flags · activa", "Consultar una bandera explícita.", "flags = {'nuevo_editor': True}\nresultado = flags.get('nuevo_editor', False)\nprint(resultado)", True, "flags"),
    ("flag-cohorte", "flags · cohorte", "Habilitar una cohorte permitida.", "permitidas = {'beta', 'staff'}; cohorte = 'beta'\nresultado = cohorte in permitidas\nprint(resultado)", True, "flags"),
    ("flag-porcentaje", "flags · porcentaje", "Asignar exposición por bucket estable.", "bucket, porcentaje = 17, 20\nresultado = bucket < porcentaje\nprint(resultado)", True, "flags"),
    ("flag-prioridad", "flags · prioridad", "Resolver reglas por prioridad.", "reglas = [('global', False), ('beta', True)]; cohorte = 'beta'\nresultado = next(valor for nombre, valor in reversed(reglas) if nombre in {'global', cohorte})\nprint(resultado)", True, "flags"),
    ("flag-resumen", "flags · resumen", "Resumir exposición de usuarios.", "usuarios = [('a', 5), ('b', 35), ('c', 19)]; porcentaje = 20\nresultado = [u for u, bucket in usuarios if bucket < porcentaje]\nprint(resultado)", ["a", "c"], "flags"),
    ("flag-suite", "flags · suite", "Combinar estado, cohorte y porcentaje.", "config = {'activa': True, 'cohortes': {'beta'}, 'porcentaje': 25}; usuarios = [('ana', 'beta', 80), ('leo', 'general', 10), ('sol', 'general', 40)]\nresultado = [u for u, c, b in usuarios if config['activa'] and (c in config['cohortes'] or b < config['porcentaje'])]\nprint(resultado)", ["ana", "leo"], "flags"),

    # 5. Canary rollout
    ("canary-muestra", "canary · muestra", "Separar muestras canary y estable.", "peticiones = [('a', 'canary'), ('b', 'estable'), ('c', 'canary')]\nresultado = [x for x, grupo in peticiones if grupo == 'canary']\nprint(resultado)", ["a", "c"], "canary"),
    ("canary-error", "canary · error", "Calcular error canary en puntos base.", "resultados = [True, True, False, True]\nresultado = sum(not ok for ok in resultados) * 10000 // len(resultados)\nprint(resultado)", 2500, "canary"),
    ("canary-comparar", "canary · comparar", "Comparar canary con la base.", "error_canary, error_base, tolerancia = 120, 100, 30\nresultado = error_canary <= error_base + tolerancia\nprint(resultado)", True, "canary"),
    ("canary-fases", "canary · fases", "Elegir la siguiente fase gradual.", "fases = [5, 20, 50, 100]; actual = 20\nresultado = fases[fases.index(actual) + 1]\nprint(resultado)", 50, "canary"),
    ("canary-detener", "canary · detener", "Detener ante una señal incumplida.", "senales = {'errores': True, 'latencia': False}\nresultado = any(senales.values())\nprint(resultado)", True, "canary"),
    ("canary-suite", "canary · suite", "Evaluar señales y decidir promoción.", "canary = {'errores': 2, 'total': 100, 'latencia': 110}; limites = {'error_bp': 300, 'latencia': 120}\nmetricas = {'error_bp': canary['errores'] * 10000 // canary['total'], 'latencia': canary['latencia']}; fallos = sorted(k for k in metricas if metricas[k] > limites[k])\nresultado = {'metricas': metricas, 'promover': not fallos, 'fallos': fallos}\nprint(resultado)", {"metricas": {"error_bp": 200, "latencia": 110}, "promover": True, "fallos": []}, "canary"),

    # 6. Expand/contract migrations
    ("migracion-expandir", "migración · expandir", "Agregar un campo compatible.", "fila = {'nombre': 'Ada'}\nresultado = {**fila, 'nombre_nuevo': fila['nombre']}\nprint(resultado)", {"nombre": "Ada", "nombre_nuevo": "Ada"}, "migrations"),
    ("migracion-dual", "migración · dual", "Leer campo nuevo con fallback.", "fila = {'nombre_viejo': 'Lin'}\nresultado = fila.get('nombre_nuevo', fila['nombre_viejo'])\nprint(resultado)", "Lin", "migrations"),
    ("migracion-rellenar", "migración · rellenar", "Rellenar filas preservando orden.", "filas = [{'id': 2, 'viejo': 'b'}, {'id': 1, 'viejo': 'a'}]\nresultado = [{**f, 'nuevo': f['viejo']} for f in filas]\nprint(resultado)", [{"id": 2, "viejo": "b", "nuevo": "b"}, {"id": 1, "viejo": "a", "nuevo": "a"}], "migrations"),
    ("migracion-verificar", "migración · verificar", "Verificar equivalencia de columnas.", "filas = [{'viejo': 'a', 'nuevo': 'a'}, {'viejo': 'b', 'nuevo': 'b'}]\nresultado = all(f['viejo'] == f['nuevo'] for f in filas)\nprint(resultado)", True, "migrations"),
    ("migracion-contraer", "migración · contraer", "Retirar el campo antiguo.", "fila = {'id': 1, 'viejo': 'a', 'nuevo': 'a'}\nresultado = {k: v for k, v in fila.items() if k != 'viejo'}\nprint(resultado)", {"id": 1, "nuevo": "a"}, "migrations"),
    ("migracion-suite", "migración · suite", "Ejecutar expand, backfill, verificación y contract.", "filas = [{'id': 1, 'viejo': 'a'}, {'id': 2, 'viejo': 'b'}]; expandidas = [{**f, 'nuevo': f['viejo']} for f in filas]\nvalida = all(f['viejo'] == f['nuevo'] for f in expandidas); finales = [{k: v for k, v in f.items() if k != 'viejo'} for f in expandidas]\nresultado = {'valida': valida, 'filas': finales}\nprint(resultado)", {"valida": True, "filas": [{"id": 1, "nuevo": "a"}, {"id": 2, "nuevo": "b"}]}, "migrations"),

    # 7. Verification gates
    ("gate-resultados", "verificación · resultados", "Comprobar todos los gates.", "gates = {'unit': True, 'e2e': True, 'smoke': True}\nresultado = all(gates.values())\nprint(resultado)", True, "verification"),
    ("gate-fallidos", "verificación · fallidos", "Listar gates fallidos.", "gates = {'unit': True, 'e2e': False, 'smoke': True}\nresultado = sorted(k for k, ok in gates.items() if not ok)\nprint(resultado)", ["e2e"], "verification"),
    ("gate-orden", "verificación · orden", "Ordenar gates por dependencia.", "dependencias = {'unit': set(), 'build': {'unit'}, 'smoke': {'build'}}; hechos = set(); orden = []\nwhile len(orden) < len(dependencias):\n    listo = min(k for k, req in dependencias.items() if k not in hechos and req <= hechos); hechos.add(listo); orden.append(listo)\nresultado = orden\nprint(resultado)", ["unit", "build", "smoke"], "verification"),
    ("gate-evidencia", "verificación · evidencia", "Asociar gates con revisiones.", "ejecuciones = [('unit', 'abc', True), ('e2e', 'abc', True), ('smoke', 'def', True)]; revision = 'abc'\nresultado = sorted(nombre for nombre, sha, ok in ejecuciones if sha == revision and ok)\nprint(resultado)", ["e2e", "unit"], "verification"),
    ("gate-cobertura", "verificación · cobertura", "Detectar evidencia ausente.", "requeridos = {'unit', 'e2e', 'smoke'}; presentes = {'unit', 'smoke'}\nresultado = sorted(requeridos - presentes)\nprint(resultado)", ["e2e"], "verification"),
    ("gate-suite", "verificación · suite", "Validar evidencia completa del mismo commit.", "revision = 'r7'; runs = [('unit', 'r7', True), ('e2e', 'r7', True), ('smoke', 'r7', True), ('viejo', 'r6', True)]; requeridos = {'unit', 'e2e', 'smoke'}\npasados = {n for n, sha, ok in runs if sha == revision and ok}\nresultado = {'completo': requeridos <= pasados, 'faltantes': sorted(requeridos - pasados)}\nprint(resultado)", {"completo": True, "faltantes": []}, "verification"),

    # 8. Rollback
    ("rollback-version", "rollback · versión", "Elegir la versión previa.", "historial = ['1.0.0', '1.1.0', '1.2.0']\nresultado = historial[-2]\nprint(resultado)", "1.1.0", "rollback"),
    ("rollback-cambios", "rollback · cambios", "Invertir cambios en orden reverso.", "cambios = [('poner', 'a', 1), ('poner', 'b', 2)]\ninversas = [('borrar', clave) for _, clave, _ in reversed(cambios)]\nresultado = inversas\nprint(resultado)", [("borrar", "b"), ("borrar", "a")], "rollback"),
    ("rollback-seguro", "rollback · seguro", "Comprobar compatibilidad de reversión.", "lectores_previos = {1, 2}; formato_actual = 2\nresultado = formato_actual in lectores_previos\nprint(resultado)", True, "rollback"),
    ("rollback-disparador", "rollback · disparador", "Activar rollback ante gates críticos.", "fallos = {'latencia': False, 'errores': True}; criticos = {'errores'}\nresultado = any(fallos.get(k, False) for k in criticos)\nprint(resultado)", True, "rollback"),
    ("rollback-plan", "rollback · plan", "Construir un plan de reversión.", "componentes = ['api', 'web']; anterior = {'api': 'a1', 'web': 'w1'}\nresultado = [('restaurar', c, anterior[c]) for c in reversed(componentes)]\nprint(resultado)", [("restaurar", "web", "w1"), ("restaurar", "api", "a1")], "rollback"),
    ("rollback-suite", "rollback · suite", "Evaluar disparador y producir plan compatible.", "senales = {'errores': 8, 'limite': 5}; actual = {'api': 'a2', 'web': 'w2'}; previo = {'api': 'a1', 'web': 'w1'}\nactivar = senales['errores'] > senales['limite']; plan = [('restaurar', k, previo[k]) for k in sorted(actual)] if activar else []\nresultado = {'activar': activar, 'plan': plan}\nprint(resultado)", {"activar": True, "plan": [("restaurar", "api", "a1"), ("restaurar", "web", "w1")]}, "rollback"),

    # 9. Promotion
    ("promocion-ambientes", "promoción · ambientes", "Ordenar ambientes de entrega.", "resultado = list(enumerate(['dev', 'staging', 'prod'], 1))\nprint(resultado)", [(1, "dev"), (2, "staging"), (3, "prod")], "promotion"),
    ("promocion-siguiente", "promoción · siguiente", "Elegir el siguiente ambiente.", "ambientes = ['dev', 'staging', 'prod']; actual = 'dev'\nresultado = ambientes[ambientes.index(actual) + 1]\nprint(resultado)", "staging", "promotion"),
    ("promocion-misma-revision", "promoción · revisión", "Verificar la misma revisión entre ambientes.", "revisiones = {'dev': 'abc', 'staging': 'abc'}\nresultado = len(set(revisiones.values())) == 1\nprint(resultado)", True, "promotion"),
    ("promocion-gates", "promoción · gates", "Exigir gates del ambiente origen.", "gates = {'dev': {'unit': True}, 'staging': {'e2e': True, 'smoke': True}}\nresultado = all(gates['staging'].values())\nprint(resultado)", True, "promotion"),
    ("promocion-historial", "promoción · historial", "Registrar promociones ordenadas.", "eventos = [(2, 'staging'), (1, 'dev'), (3, 'prod')]\nresultado = [ambiente for _, ambiente in sorted(eventos)]\nprint(resultado)", ["dev", "staging", "prod"], "promotion"),
    ("promocion-suite", "promoción · suite", "Promover la misma revisión solo con gates verdes.", "revision = 'c9'; estados = [('dev', 'c9', True), ('staging', 'c9', True)]; orden = ['dev', 'staging', 'prod']\nactual = estados[-1]; siguiente = orden[orden.index(actual[0]) + 1]\nresultado = {'revision': revision, 'siguiente': siguiente, 'permitida': actual[1] == revision and actual[2]}\nprint(resultado)", {"revision": "c9", "siguiente": "prod", "permitida": True}, "promotion"),

    # 10. Safe-delivery capstone
    ("entrega-contrato", "capstone · contrato", "Validar versión y artefactos.", "m = {'version': (2, 1, 0), 'artefactos': {'api', 'web'}}\nresultado = m['version'] > (2, 0, 0) and m['artefactos'] == {'api', 'web'}\nprint(resultado)", True, "capstone"),
    ("entrega-compatibilidad", "capstone · compatibilidad", "Verificar consumidores de la entrega.", "formato = 2; consumidores = {'web': {1, 2}, 'worker': {2}}\nresultado = all(formato in soporta for soporta in consumidores.values())\nprint(resultado)", True, "capstone"),
    ("entrega-exposicion", "capstone · exposición", "Seleccionar cohorte canary estable.", "usuarios = [('ana', 7), ('leo', 42), ('sol', 18)]; porcentaje = 20\nresultado = [u for u, bucket in usuarios if bucket < porcentaje]\nprint(resultado)", ["ana", "sol"], "capstone"),
    ("entrega-verificacion", "capstone · verificación", "Evaluar gates sobre la misma revisión.", "sha = 'z8'; runs = [('unit', 'z8', True), ('e2e', 'z8', True), ('smoke', 'z8', True)]\nresultado = all(s == sha and ok for _, s, ok in runs)\nprint(resultado)", True, "capstone"),
    ("entrega-decision", "capstone · decisión", "Elegir promoción o rollback.", "compatible, gates, error_bp, limite = True, True, 180, 200\nresultado = 'promover' if compatible and gates and error_bp <= limite else 'rollback'\nprint(resultado)", "promover", "capstone"),
    ("ola36-suite", "ola 36 · suite", "Cerrar la ola con contrato, canary, gates y reversión.", "release = {'version': (3, 1, 0), 'revision': 'r9'}; soporte = {'web': {3}, 'worker': {2, 3}}; canary = [True, True, False, True, True]; limite_bp = 2500\ncompatible = all(3 in versiones for versiones in soporte.values()); error_bp = sum(not ok for ok in canary) * 10000 // len(canary); gates = {'unit': True, 'e2e': True, 'smoke': True}; accion = 'promover' if compatible and all(gates.values()) and error_bp <= limite_bp else 'rollback'\nresultado = {'revision': release['revision'], 'compatible': compatible, 'error_bp': error_bp, 'accion': accion}\nprint(resultado)", {"revision": "r9", "compatible": True, "error_bp": 2000, "accion": "promover"}, "capstone"),
]

RAW = CASES


def build_raw(entries=CASES):
    assert len(entries) == 60
    return [exercise(3101 + index, *case) for index, case in enumerate(entries)]


if __name__ == "__main__":
    print(emit_rust(build_raw()))
