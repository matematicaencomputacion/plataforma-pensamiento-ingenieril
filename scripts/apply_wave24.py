"""Aplica la Ola 24 (micro-steps 2381-2440) a los fuentes Rust del curriculum.

Modifica `web/src/curriculum.rs` y `web/src/concepts/mod.rs`.

Uso:  python3 scripts/apply_wave24.py
Pre-requisitos: los archivos generados se reusan vía import de gen_wave24.
"""

import os
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
CURR = os.path.join(ROOT, "web", "src", "curriculum.rs")
CONCEPTS = os.path.join(ROOT, "web", "src", "concepts", "mod.rs")

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import gen_wave24  # noqa: E402


def apply_curriculum():
    src = open(CURR, encoding="utf-8").read()
    steps = gen_wave24.build_raw(gen_wave24.RAW)
    block = gen_wave24.emit_rust(steps)
    refs = gen_wave24.emit_refs(steps)

    # 1) Encadenar 2380 -> 2381
    old_next = "    next: None, show_type_chips: false, micro_step: 2380,\n};"
    assert src.count(old_next) == 1, "marker py-2380 next:None not unique"
    new_next = "    next: Some(\"py-2381-map-lambda\"), show_type_chips: false, micro_step: 2380,\n};"
    src = src.replace(old_next, new_next)

    # 2) Insertar el bloque de 60 pasos antes de CODING_STEPS
    marker = "pub const CODING_STEPS: &[&CodingStep] = &["
    assert src.count(marker) == 1, "CODING_STEPS marker not unique"
    src = src.replace(marker, block + "\n" + marker)

    # 3) Agregar las 60 refs al array
    old_ref = "    &PY2380_PIPE_CHECK,\n];"
    assert src.count(old_ref) == 1, "PY2380 ref marker not unique"
    src = src.replace(old_ref, "    &PY2380_PIPE_CHECK,\n" + refs + "\n];")

    # 4) Actualizar chain test: 2380 deja de ser el fin del rail
    old_seg = (
        "            } else {\n"
        '                assert_eq!(step.next, None, "step 2380 is the end of the rail");\n'
        "            }"
    )
    assert src.count(old_seg) == 1, "wave23 chain else branch not unique"
    new_seg = (
        "            } else {\n"
        '                assert_eq!(step.next, Some("py-2381-map-lambda"), "step 2380 chains to wave24");\n'
        "            }"
    )
    src = src.replace(old_seg, new_seg)

    # 5) Agregar test de cadena de la ola 24
    w24_test = '''
    #[test]
    fn py2381_to_py2440_pipeline_chain() {
        let bridge = coding_step_by_micro_step(2380).expect("py-2380");
        assert_eq!(bridge.next, Some("py-2381-map-lambda"));

        for n in 2381..=2440 {
            let step = coding_step_by_micro_step(n).expect("wave24 chain step");
            assert_eq!(step.micro_step, n);
            assert!(
                step.id.starts_with(&format!("py-{n}-")),
                "step {n} id '{}' should start with py-{n}-",
                step.id
            );
            if n < 2440 {
                let next_step = coding_step_by_micro_step(n + 1).expect("next chain step");
                assert_eq!(
                    step.next,
                    Some(next_step.id),
                    "step {n} should chain to {}",
                    next_step.id
                );
            } else {
                assert_eq!(step.next, None, "step 2440 is the end of the rail");
            }
        }
    }
'''
    anchor = "\n    #[test]\n    fn micro_step_unlocked_uses_cursor() {"
    assert src.count(anchor) == 1, "micro_step_unlocked test anchor not unique"
    src = src.replace(anchor, w24_test + anchor)

    open(CURR, "w", encoding="utf-8").write(src)
    print("curriculum.rs updated")


# Tags (lenses) por micro-step para STEP_PARTITIONS (partición ids):
#   group -> (range_start, tags)
PARTITION_ROWS = [
    (2381, 2386, [3]),    # map/filter lambdas: paradigms
    (2387, 2392, [3, 2]), # funciones como datos/callbacks: paradigms + scope-legb
    (2393, 2398, [3, 4]), # encadenar generadores: paradigms + ecosystem
    (2399, 2404, [3, 1]), # reduce/folding: paradigms + data-model
    (2405, 2410, [5, 1]), # pipelines logs: domains + data-model
    (2411, 2416, [3, 2]), # filter predicados: paradigms + scope-legb
    (2417, 2422, [5, 3]), # mapped/serial ETL: domains + paradigms
    (2423, 2428, [3, 4]), # early termination: paradigms + ecosystem
    (2429, 2434, [3, 1]), # agregación streaming: paradigms + data-model
    (2435, 2440, [5, 3]), # pipeline de scoring: domains + paradigms
]


def partition_rows_block():
    rows = []
    for lo, hi, tags in PARTITION_ROWS:
        fmt = ", ".join(str(t) for t in tags)
        for n in range(lo, hi + 1):
            rows.append("    (%d, &[%s])," % (n, fmt))
    return "\n".join(rows)


def apply_concepts():
    src = open(CONCEPTS, encoding="utf-8").read()

    # 1) Insertar 60 filas de STEP_PARTITIONS tras la fila (2380, ...)
    old_row = "    (2380, &[5, 4]),\n];"
    assert src.count(old_row) == 1, "2380 STEP_PARTITIONS row marker not unique"
    new_rows = "    (2380, &[5, 4]),\n" + partition_rows_block() + "\n];"
    src = src.replace(old_row, new_rows)

    # 2) Renombrar WAVE23_FROZEN_BEYOND_2380 -> WAVE24_FROZEN_BEYOND_2440 (+ doc)
    old_const = (
        "    /// Frozen `(micro_step, tags)` pairs with `micro_step > 2380` (Wave 23 ceiling).\n"
        "    const WAVE23_FROZEN_BEYOND_2380: &[(i32, &[u8])] = &[];"
    )
    assert src.count(old_const) == 1, "WAVE23 const block not unique"
    new_const = (
        "    /// Frozen `(micro_step, tags)` pairs with `micro_step > 2440` (Wave 24 ceiling).\n"
        "    const WAVE24_FROZEN_BEYOND_2440: &[(i32, &[u8])] = &[];"
    )
    src = src.replace(old_const, new_const)

    # 3) Renombrar y actualizar el freeze test
    old_test = (
        "    #[test]\n"
        "    fn wave23_freeze_rows_beyond_2380() {\n"
        "        let current: Vec<(i32, &[u8])> = STEP_PARTITIONS\n"
        "            .iter()\n"
        "            .copied()\n"
        "            .filter(|(n, _)| *n > 2380)\n"
        "            .collect();\n"
        "        assert_eq!(\n"
        "            current.as_slice(),\n"
        "            WAVE23_FROZEN_BEYOND_2380,\n"
        '            "do not add or remove rows > 2380"\n'
        "        );\n"
        "    }"
    )
    assert src.count(old_test) == 1, "wave23 freeze test not unique"
    new_test = (
        "    #[test]\n"
        "    fn wave24_freeze_rows_beyond_2440() {\n"
        "        let current: Vec<(i32, &[u8])> = STEP_PARTITIONS\n"
        "            .iter()\n"
        "            .copied()\n"
        "            .filter(|(n, _)| *n > 2440)\n"
        "            .collect();\n"
        "        assert_eq!(\n"
        "            current.as_slice(),\n"
        "            WAVE24_FROZEN_BEYOND_2440,\n"
        '            "do not add or remove rows > 2440"\n'
        "        );\n"
        "    }"
    )
    src = src.replace(old_test, new_test)

    # 4) Actualizar test partition: extender asserts de ola 24 y correr la frontera
    old_part = '        assert_eq!(partitions_for_micro_step(2380), &[5, 4]); // Wave 23: pipeline\n'
    assert src.count(old_part) == 1, "partition 2380 assert not unique"
    new_part = (
        '        assert_eq!(partitions_for_micro_step(2380), &[5, 4]); // Wave 23: pipeline\n'
        '        assert_eq!(partitions_for_micro_step(2381), &[3]); // Wave 24: map/filter = paradigms\n'
        '        assert_eq!(partitions_for_micro_step(2387), &[3, 2]); // Wave 24: callbacks = paradigms + scope-legb\n'
        '        assert_eq!(partitions_for_micro_step(2393), &[3, 4]); // Wave 24: generadores = paradigms + ecosystem\n'
        '        assert_eq!(partitions_for_micro_step(2399), &[3, 1]); // Wave 24: reduce = paradigms + data-model\n'
        '        assert_eq!(partitions_for_micro_step(2405), &[5, 1]); // Wave 24: logs = domains + data-model\n'
        '        assert_eq!(partitions_for_micro_step(2423), &[3, 4]); // Wave 24: early = paradigms + ecosystem\n'
        '        assert_eq!(partitions_for_micro_step(2435), &[5, 3]); // Wave 24: scoring = domains + paradigms\n'
        '        assert_eq!(partitions_for_micro_step(2440), &[5, 3]); // Wave 24: ranking\n'
        '        assert!(partitions_for_micro_step(2441).is_empty()); // frontier beyond Wave 24\n'
    )
    src = src.replace(old_part, new_part)

    open(CONCEPTS, "w", encoding="utf-8").write(src)
    print("concepts/mod.rs updated")


if __name__ == "__main__":
    apply_curriculum()
    apply_concepts()
    print("done")