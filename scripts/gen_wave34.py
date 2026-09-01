"""Generate Wave 34: deterministic consistency and recovery exercises."""

from gen_wave33 import exercise, emit_refs, emit_rust


CASES = [
    # 1. Schema evolution
    ("esquema-campos", "esquema · campos", "Listar campos requeridos de una versión.", "esquemas = {1: ['id'], 2: ['id', 'nombre']}\nresultado = esquemas[2]\nprint(resultado)", ["id", "nombre"], "schemas"),
    ("esquema-default", "esquema · default", "Completar un campo nuevo con valor por defecto.", "registro = {'id': 7}\nresultado = {**registro, 'activo': registro.get('activo', True)}\nprint(resultado)", {"id": 7, "activo": True}, "schemas"),
    ("esquema-renombrar", "esquema · renombrar", "Migrar un nombre de campo.", "registro = {'nombre_viejo': 'Ada'}\nresultado = {'nombre': registro['nombre_viejo']}\nprint(resultado)", {"nombre": "Ada"}, "schemas"),
    ("esquema-compatible", "esquema · compatibilidad", "Verificar lectura compatible hacia atrás.", "requeridos = {'id'}\nregistro = {'id': 4, 'extra': 'x'}\nresultado = requeridos <= registro.keys()\nprint(resultado)", True, "schemas"),
    ("esquema-migrar-lote", "esquema · lote", "Migrar un lote preservando orden.", "filas = [{'id': 2}, {'id': 1, 'activo': False}]\nresultado = [{**f, 'activo': f.get('activo', True)} for f in filas]\nprint(resultado)", [{"id": 2, "activo": True}, {"id": 1, "activo": False}], "schemas"),
    ("esquema-suite", "esquema · suite", "Validar versión, migrar y proyectar campos canónicos.", "entrada = {'version': 1, 'id': 3, 'nombre_viejo': 'Lin'}\ncanonico = {'version': 2, 'id': entrada['id'], 'nombre': entrada['nombre_viejo'], 'activo': True}\nresultado = canonico\nprint(resultado)", {"version": 2, "id": 3, "nombre": "Lin", "activo": True}, "schemas"),
    # 2. Total ordering
    ("orden-secuencia", "orden · secuencia", "Ordenar por número de secuencia.", "eventos = [(3, 'c'), (1, 'a'), (2, 'b')]\nresultado = sorted(eventos)\nprint(resultado)", [(1, "a"), (2, "b"), (3, "c")], "ordering"),
    ("orden-desempate", "orden · desempate", "Desempatar por origen de forma estable.", "eventos = [(2, 'b'), (2, 'a'), (1, 'z')]\nresultado = sorted(eventos, key=lambda e: (e[0], e[1]))\nprint(resultado)", [(1, "z"), (2, "a"), (2, "b")], "ordering"),
    ("orden-monotono", "orden · monotonicidad", "Detectar una secuencia monótona.", "secuencia = [2, 4, 7, 9]\nresultado = all(a < b for a, b in zip(secuencia, secuencia[1:]))\nprint(resultado)", True, "ordering"),
    ("orden-inversiones", "orden · inversiones", "Contar pares fuera de orden.", "valores = [3, 1, 2]\nresultado = sum(valores[i] > valores[j] for i in range(len(valores)) for j in range(i + 1, len(valores)))\nprint(resultado)", 2, "ordering"),
    ("orden-merge", "orden · merge", "Unir dos secuencias ordenadas.", "a = [(1, 'a'), (4, 'd')]; b = [(2, 'b'), (3, 'c')]\nresultado = sorted(a + b)\nprint(resultado)", [(1, "a"), (2, "b"), (3, "c"), (4, "d")], "ordering"),
    ("orden-suite", "orden · suite", "Ordenar, deduplicar y detectar huecos.", "eventos = [(3, 'c'), (1, 'a'), (3, 'c'), (5, 'e')]\nordenados = sorted(set(eventos)); numeros = [n for n, _ in ordenados]\nresultado = {'eventos': ordenados, 'huecos': [n for n in range(numeros[0], numeros[-1] + 1) if n not in numeros]}\nprint(resultado)", {"eventos": [(1, "a"), (3, "c"), (5, "e")], "huecos": [2, 4]}, "ordering"),
    # 3. Transactional outbox
    ("outbox-registrar", "outbox · registrar", "Registrar cambio y evento juntos.", "estado = {'saldo': 5}; outbox = []\nestado['saldo'] += 2; outbox.append(('saldo', 7))\nresultado = (estado, outbox)\nprint(resultado)", ({"saldo": 7}, [("saldo", 7)]), "outbox"),
    ("outbox-pendientes", "outbox · pendientes", "Seleccionar eventos no publicados.", "eventos = [('e1', True), ('e2', False), ('e3', False)]\nresultado = [e for e, publicado in eventos if not publicado]\nprint(resultado)", ["e2", "e3"], "outbox"),
    ("outbox-marcar", "outbox · marcar", "Marcar publicación por id.", "eventos = {'e1': False, 'e2': False}\neventos['e1'] = True\nresultado = eventos\nprint(resultado)", {"e1": True, "e2": False}, "outbox"),
    ("outbox-orden", "outbox · orden", "Publicar respetando secuencia.", "eventos = [(3, 'e3'), (1, 'e1'), (2, 'e2')]\nresultado = [e for _, e in sorted(eventos)]\nprint(resultado)", ["e1", "e2", "e3"], "outbox"),
    ("outbox-reintento", "outbox · reintento", "Conservar pendientes tras un intento parcial.", "pendientes = ['e1', 'e2', 'e3']; enviados = {'e1', 'e3'}\nresultado = [e for e in pendientes if e not in enviados]\nprint(resultado)", ["e2"], "outbox"),
    ("outbox-suite", "outbox · suite", "Aplicar comandos y drenar outbox idempotente.", "comandos = [('c1', 2), ('c2', 3), ('c1', 2)]\nvistos, total, outbox = set(), 0, []\nfor cid, valor in comandos:\n    if cid not in vistos: vistos.add(cid); total += valor; outbox.append((len(outbox) + 1, cid, total))\nresultado = {'total': total, 'publicados': [cid for _, cid, _ in outbox]}\nprint(resultado)", {"total": 5, "publicados": ["c1", "c2"]}, "outbox"),
    # 4. Sagas
    ("saga-pasos", "saga · pasos", "Enumerar pasos de una transacción larga.", "pasos = ['reservar', 'cobrar', 'enviar']\nresultado = list(enumerate(pasos, 1))\nprint(resultado)", [(1, "reservar"), (2, "cobrar"), (3, "enviar")], "sagas"),
    ("saga-compensar", "saga · compensar", "Invertir pasos completados al fallar.", "completados = ['reservar', 'cobrar']\ncompensa = {'reservar': 'liberar', 'cobrar': 'reembolsar'}\nresultado = [compensa[p] for p in reversed(completados)]\nprint(resultado)", ["reembolsar", "liberar"], "sagas"),
    ("saga-estado", "saga · estado", "Calcular estado según pasos completados.", "esperados = {'reservar', 'cobrar', 'enviar'}; completados = {'reservar', 'cobrar'}\nresultado = 'completa' if completados == esperados else 'pendiente'\nprint(resultado)", "pendiente", "sagas"),
    ("saga-fallo", "saga · fallo", "Localizar el primer paso fallido.", "resultados = [('reservar', True), ('cobrar', False), ('enviar', True)]\nresultado = next((paso for paso, ok in resultados if not ok), None)\nprint(resultado)", "cobrar", "sagas"),
    ("saga-plan", "saga · plan", "Construir plan de compensación parcial.", "hechos = ['crear', 'reservar', 'cobrar']; fallo = 'cobrar'\ncompensa = {'crear': 'borrar', 'reservar': 'liberar', 'cobrar': 'reembolsar'}\nresultado = [compensa[p] for p in reversed(hechos[:hechos.index(fallo)])]\nprint(resultado)", ["liberar", "borrar"], "sagas"),
    ("saga-suite", "saga · suite", "Ejecutar hasta fallo y producir compensaciones.", "pasos = [('reservar', True), ('cobrar', True), ('enviar', False)]\ncompensa = {'reservar': 'liberar', 'cobrar': 'reembolsar'}; hechos = []\nfor paso, ok in pasos:\n    if not ok: break\n    hechos.append(paso)\nresultado = {'hechos': hechos, 'compensar': [compensa[p] for p in reversed(hechos)]}\nprint(resultado)", {"hechos": ["reservar", "cobrar"], "compensar": ["reembolsar", "liberar"]}, "sagas"),
    # 5. Logical leases
    ("lease-vigente", "lease · vigencia", "Validar un lease con tick lógico.", "lease = {'hasta': 8}; tick = 6\nresultado = tick < lease['hasta']\nprint(resultado)", True, "leases"),
    ("lease-vencer", "lease · vencer", "Detectar lease vencido.", "hasta, tick = 5, 5\nresultado = tick >= hasta\nprint(resultado)", True, "leases"),
    ("lease-renovar", "lease · renovar", "Renovar desde el tick actual.", "tick, duracion = 7, 4\nresultado = {'desde': tick, 'hasta': tick + duracion}\nprint(resultado)", {"desde": 7, "hasta": 11}, "leases"),
    ("lease-dueno", "lease · dueño", "Aceptar operación solo del dueño vigente.", "lease = {'dueno': 'a', 'hasta': 9}; actor, tick = 'a', 8\nresultado = actor == lease['dueno'] and tick < lease['hasta']\nprint(resultado)", True, "leases"),
    ("lease-transferir", "lease · transferir", "Transferir un lease vencido.", "lease = {'dueno': 'a', 'hasta': 4}; tick = 5\nresultado = {'dueno': 'b', 'hasta': tick + 3} if tick >= lease['hasta'] else lease\nprint(resultado)", {"dueno": "b", "hasta": 8}, "leases"),
    ("lease-suite", "lease · suite", "Procesar renovaciones y rechazos con ticks.", "lease = {'dueno': 'a', 'hasta': 3}; acciones = [(2, 'a'), (3, 'b'), (5, 'b')]\naceptadas = []\nfor tick, actor in acciones:\n    if tick >= lease['hasta']: lease = {'dueno': actor, 'hasta': tick + 2}\n    if actor == lease['dueno']: aceptadas.append((tick, actor))\nresultado = {'lease': lease, 'aceptadas': aceptadas}\nprint(resultado)", {"lease": {"dueno": "b", "hasta": 7}, "aceptadas": [(2, "a"), (3, "b"), (5, "b")]}, "leases"),
    # 6. Quorum
    ("quorum-mayoria", "quorum · mayoría", "Calcular mayoría mínima.", "replicas = 5\nresultado = replicas // 2 + 1\nprint(resultado)", 3, "quorum"),
    ("quorum-alcanzado", "quorum · alcanzado", "Decidir si hay quorum.", "votos = {'a', 'b', 'c'}; requerido = 3\nresultado = len(votos) >= requerido\nprint(resultado)", True, "quorum"),
    ("quorum-valor", "quorum · valor", "Elegir valor con más votos.", "votos = ['x', 'y', 'x', 'x', 'y']\nresultado = max(sorted(set(votos)), key=votos.count)\nprint(resultado)", "x", "quorum"),
    ("quorum-version", "quorum · versión", "Elegir la versión máxima confirmada por mayoría.", "acks = {1: {'a', 'b', 'c'}, 2: {'a', 'b'}, 3: {'a', 'b', 'c'}}\nresultado = max(v for v, nodos in acks.items() if len(nodos) >= 3)\nprint(resultado)", 3, "quorum"),
    ("quorum-faltantes", "quorum · faltantes", "Calcular votos faltantes.", "actuales, requerido = 2, 4\nresultado = max(0, requerido - actuales)\nprint(resultado)", 2, "quorum"),
    ("quorum-suite", "quorum · suite", "Resolver una elección determinista por término y votos.", "candidatos = {'a': (2, {'a', 'b'}), 'b': (3, {'b', 'c', 'd'}), 'c': (3, {'a', 'c'})}\nvalidos = [(term, nombre) for nombre, (term, votos) in candidatos.items() if len(votos) >= 3]\nresultado = max(validos)[1]\nprint(resultado)", "b", "quorum"),
    # 7. Log compaction
    ("compactar-ultimo", "compactación · último", "Conservar el último valor por clave.", "log = [('a', 1), ('b', 2), ('a', 3)]\nresultado = {k: v for k, v in log}\nprint(resultado)", {"a": 3, "b": 2}, "compaction"),
    ("compactar-tumbas", "compactación · tumbas", "Eliminar claves con tombstone.", "log = [('a', 1), ('b', 2), ('a', None)]\nestado = {}\nfor k, v in log:\n    if v is None: estado.pop(k, None)\n    else: estado[k] = v\nresultado = estado\nprint(resultado)", {"b": 2}, "compaction"),
    ("compactar-offset", "compactación · offset", "Conservar offset junto al valor final.", "log = [(1, 'a', 2), (3, 'a', 5), (2, 'b', 4)]\nresultado = {k: (off, val) for off, k, val in sorted(log)}\nprint(resultado)", {"a": (3, 5), "b": (2, 4)}, "compaction"),
    ("compactar-claves", "compactación · claves", "Ordenar claves compactadas.", "estado = {'z': 1, 'a': 3, 'm': 2}\nresultado = [(k, estado[k]) for k in sorted(estado)]\nprint(resultado)", [("a", 3), ("m", 2), ("z", 1)], "compaction"),
    ("compactar-ahorro", "compactación · ahorro", "Medir entradas eliminadas.", "log = [('a', 1), ('a', 2), ('b', 1), ('a', 3)]\nresultado = len(log) - len({k for k, _ in log})\nprint(resultado)", 2, "compaction"),
    ("compactar-suite", "compactación · suite", "Compactar respetando tombstones y offsets.", "log = [(1, 'a', 2), (2, 'b', 4), (3, 'a', None), (4, 'b', 7)]\nestado = {}\nfor off, clave, valor in log:\n    if valor is None: estado.pop(clave, None)\n    else: estado[clave] = (off, valor)\nresultado = {'estado': estado, 'hasta': max(off for off, _, _ in log)}\nprint(resultado)", {"estado": {"b": (4, 7)}, "hasta": 4}, "compaction"),
    # 8. Audit trail
    ("auditar-entrada", "auditoría · entrada", "Construir una entrada de auditoría.", "resultado = {'actor': 'ana', 'accion': 'editar', 'recurso': 'r1'}\nprint(resultado)", {"actor": "ana", "accion": "editar", "recurso": "r1"}, "audit"),
    ("auditar-filtrar", "auditoría · filtrar", "Filtrar acciones por actor.", "log = [('ana', 'crear'), ('leo', 'leer'), ('ana', 'editar')]\nresultado = [accion for actor, accion in log if actor == 'ana']\nprint(resultado)", ["crear", "editar"], "audit"),
    ("auditar-secuencia", "auditoría · secuencia", "Validar ids consecutivos.", "ids = [10, 11, 12]\nresultado = ids == list(range(ids[0], ids[-1] + 1))\nprint(resultado)", True, "audit"),
    ("auditar-cambios", "auditoría · cambios", "Calcular campos modificados.", "antes = {'a': 1, 'b': 2}; despues = {'a': 3, 'b': 2}\nresultado = {k: (antes[k], despues[k]) for k in sorted(antes) if antes[k] != despues[k]}\nprint(resultado)", {"a": (1, 3)}, "audit"),
    ("auditar-resumen", "auditoría · resumen", "Contar acciones por tipo.", "acciones = ['crear', 'leer', 'crear', 'editar']\nresultado = {a: acciones.count(a) for a in sorted(set(acciones))}\nprint(resultado)", {"crear": 2, "editar": 1, "leer": 1}, "audit"),
    ("auditar-suite", "auditoría · suite", "Validar secuencia y resumir actores y acciones.", "log = [(1, 'ana', 'crear'), (2, 'leo', 'leer'), (3, 'ana', 'editar')]\nresultado = {'continua': [i for i, _, _ in log] == [1, 2, 3], 'actores': sorted({a for _, a, _ in log}), 'acciones': [x for _, _, x in log]}\nprint(resultado)", {"continua": True, "actores": ["ana", "leo"], "acciones": ["crear", "leer", "editar"]}, "audit"),
    # 9. Repair
    ("reparar-diferencias", "reparación · diferencias", "Detectar claves divergentes.", "esperado = {'a': 2, 'b': 3}; actual = {'a': 2, 'b': 1}\nresultado = [k for k in sorted(esperado) if esperado[k] != actual.get(k)]\nprint(resultado)", ["b"], "repair"),
    ("reparar-faltantes", "reparación · faltantes", "Detectar claves ausentes.", "esperado = {'a', 'b', 'c'}; actual = {'a', 'c'}\nresultado = sorted(esperado - actual)\nprint(resultado)", ["b"], "repair"),
    ("reparar-plan", "reparación · plan", "Crear operaciones de reparación.", "esperado = {'a': 2, 'b': 3}; actual = {'a': 1, 'c': 4}\nresultado = [('poner', k, esperado[k]) for k in sorted(esperado) if actual.get(k) != esperado[k]] + [('borrar', k) for k in sorted(actual.keys() - esperado.keys())]\nprint(resultado)", [("poner", "a", 2), ("poner", "b", 3), ("borrar", "c")], "repair"),
    ("reparar-aplicar", "reparación · aplicar", "Aplicar un plan determinista.", "estado = {'a': 1, 'c': 4}; plan = [('poner', 'a', 2), ('poner', 'b', 3), ('borrar', 'c')]\nfor op in plan:\n    if op[0] == 'poner': estado[op[1]] = op[2]\n    else: estado.pop(op[1], None)\nresultado = estado\nprint(resultado)", {"a": 2, "b": 3}, "repair"),
    ("reparar-verificar", "reparación · verificar", "Verificar convergencia final.", "replicas = [{'a': 2}, {'a': 2}, {'a': 2}]\nresultado = all(r == replicas[0] for r in replicas[1:])\nprint(resultado)", True, "repair"),
    ("reparar-suite", "reparación · suite", "Comparar, reparar y verificar dos réplicas.", "fuente = {'a': 2, 'b': 3}; replica = {'a': 1, 'c': 4}\nfor clave in list(replica):\n    if clave not in fuente: del replica[clave]\nfor clave, valor in fuente.items(): replica[clave] = valor\nresultado = {'replica': replica, 'converge': replica == fuente}\nprint(resultado)", {"replica": {"a": 2, "b": 3}, "converge": True}, "repair"),
    # 10. Recovery capstone
    ("recuperar-migrar", "capstone · migrar", "Migrar registros antes de recuperar.", "fila = {'version': 1, 'id': 2}\nresultado = {**fila, 'version': 2, 'activo': True}\nprint(resultado)", {"version": 2, "id": 2, "activo": True}, "capstone"),
    ("recuperar-ordenar", "capstone · ordenar", "Ordenar eventos recuperados.", "eventos = [(3, 'c'), (1, 'a'), (2, 'b')]\nresultado = [e for _, e in sorted(eventos)]\nprint(resultado)", ["a", "b", "c"], "capstone"),
    ("recuperar-quorum", "capstone · quorum", "Elegir snapshot con quorum.", "snapshots = {2: {'a', 'b'}, 3: {'a', 'b', 'c'}}\nresultado = max(v for v, votos in snapshots.items() if len(votos) >= 3)\nprint(resultado)", 3, "capstone"),
    ("recuperar-compensar", "capstone · compensar", "Compensar pasos posteriores al snapshot.", "hechos = ['reservar', 'cobrar']; compensa = {'reservar': 'liberar', 'cobrar': 'reembolsar'}\nresultado = [compensa[p] for p in reversed(hechos)]\nprint(resultado)", ["reembolsar", "liberar"], "capstone"),
    ("recuperar-reparar", "capstone · reparar", "Reparar réplica desde estado canónico.", "canonico = {'x': 3, 'y': 4}; replica = {'x': 1}\nresultado = dict(canonico)\nprint(resultado)", {"x": 3, "y": 4}, "capstone"),
    ("ola34-suite", "ola 34 · suite", "Cerrar la ola con migración, outbox, quorum, auditoría y reparación.", "registro = {'version': 1, 'id': 7}; eventos = [(2, 'actualizar', 5), (1, 'crear', 2)]\nestado = {'id': registro['id'], 'version': 2, 'total': 0}; auditoria = []\nfor secuencia, accion, valor in sorted(eventos): estado['total'] += valor; auditoria.append((secuencia, accion))\nreplicas = [dict(estado), {'id': 7, 'version': 2, 'total': 6}]; canonico = max(replicas, key=lambda r: r['total'])\nresultado = {'estado': canonico, 'auditoria': auditoria, 'reparadas': [dict(canonico) for _ in replicas]}\nprint(resultado)", {"estado": {"id": 7, "version": 2, "total": 7}, "auditoria": [(1, "crear"), (2, "actualizar")], "reparadas": [{"id": 7, "version": 2, "total": 7}, {"id": 7, "version": 2, "total": 7}]}, "capstone"),
]

RAW = CASES


def build_raw(entries=CASES):
    assert len(entries) == 60
    return [exercise(2981 + index, *case) for index, case in enumerate(entries)]


if __name__ == "__main__":
    print(emit_rust(build_raw()))
