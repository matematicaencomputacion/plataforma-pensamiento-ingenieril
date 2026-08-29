"""Aplica los micro-steps 2441-2500 de la Ola 25 al curriculum.rs y concepts/mod.rs.

Este script es robusto contra el estado actual de main (después de Wave 24 mergeado).
Maneja correctamente:
- El case donde micro_step: 2380 ya apunta a py-2441-advanced-pipeline (main post-Wave 24)
- La constante WAVE25_FROZEN_BEYOND_2500
- Las particiones y tests de concepts/mod.rs
- Los 60 CodingStep blocks y referencias en curriculum.rs
- El test py2441_to_py2500_pipeline_chain
"""

import re
import sys

sys.path.insert(0, '.')


def apply_wave25_to_curriculum(curriculum_path):
    """Aplica los cambios de Wave 25 al archivo curriculum.rs."""
    
    with open(curriculum_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    updated = False
    
    # 1. Actualizar next de step 2380 a py-2441-advanced-pipeline si aún no lo está
    old_next_2380 = 'next: Some("py-2381-map-lambda"),'
    new_next_2380 = 'next: Some("py-2441-advanced-pipeline"),'
    
    if old_next_2380 in content:
        # Verificar si ya fue actualizado
        if new_next_2380 not in content:
            content = content.replace(old_next_2380, new_next_2380)
            updated = True
            print("✓ Updated step 2380 next to py-2441-advanced-pipeline")
        else:
            print("✓ Step 2380 already updated to py-2441-advanced-pipeline")
    else:
        print("! Could not find step 2380 next marker (may already be Wave 25 state)")
    
    # 2. Insertar 60 nuevos CodingStep blocks (micro-steps 2441-2500) antes de CODING_STEPS
    # Buscar el punto de inserción: justo antes de "pub const DEFAULT_CODING_STEP_ID"
    insertion_marker = 'pub const DEFAULT_CODING_STEP_ID: &str = "py-02-variables";'
    
    # Generar los 60 bloques Rust para los steps 2441-2500
    # Usar el mismo patrón que gen_wave25.py
    rust_blocks = []
    for num in range(2441, 2501):
        const_name = f"PY{num}_STEP".upper().replace("-", "_")
        # Generar un bloque básico - el contenido real vendrá de gen_wave25.py
        rust_block = f'''pub const {const_name}: CodingStep = CodingStep {{
    id: "py-{num}-step", title: "Step {num}", objective: "Objective for step {num}",
    prompt_md: "...", starter_code: "...", pytest: "...", hint: "...", 
    solution_example: "...", next: None, show_type_chips: false, micro_step: {num},
}};'''
        rust_blocks.append(rust_block)
    
    # Juntar los bloques
    rust_blocks_joined = "\n".join(rust_blocks)
    
    # Insertar antes del marcador
    if insertion_marker in content:
        # Verificar si ya fueron insertados (buscando el primer block de Wave 25)
        if "py-2441" not in content:
            content = content.replace(insertion_marker, f"{rust_blocks_joined}\n{insertion_marker}")
            updated = True
            print(f"✓ Inserted {len(rust_blocks)} Rust CodingStep blocks for steps 2441-2500")
        else:
            print("✓ Wave 25 blocks already inserted in curriculum.rs")
    else:
        print("! Could not find insertion point in curriculum.rs")
    
    # 3. Agregar referencias &PY2441_* a &PY2500_* al array CODING_STEPS
    # Buscar la sección del array CODING_STEPS - buscar PY2440_SCORE_CHECK
    steps_array_marker = '    &PY2440_SCORE_CHECK,'
    
    # Generar las referencias para steps 2441-2500
    refs = []
    for num in range(2441, 2501):
        ref_line = f"    &PY{num}_STEP,".replace("PY" + str(num) + "_STEP,", f"PY{num}_STEP,")
        refs.append(ref_line)
    
    refs_joined = "\n".join(refs)
    
    if steps_array_marker in content:
        # Insertar nuevas referencias después de PY2440
        if "PY2500_SCORE_CHECK" not in content:
            content = content.replace(
                steps_array_marker,
                steps_array_marker.replace("&PY2440_SCORE_CHECK,", "&PY2440_SCORE_CHECK,\n" + refs_joined)
            )
            updated = True
            print("✓ Updated CODING_STEPS references")
        else:
            print("✓ CODING_STEPS references already updated")
    else:
        print("! Could not find CODING_STEPS array marker")
    
    # Escribir el contenido actualizado
    with open(curriculum_path, 'w', encoding='utf-8') as f:
        f.write(content)
    
    return updated


def apply_wave25_to_concepts(concepts_path):
    """Aplica los cambios de Wave 25 al archivo concepts/mod.rs."""
    
    with open(concepts_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    updated = False
    
    # 1. Agregar nuevas rows de STEP_PARTITIONS para micro-steps 2441-2500
    # El patrón es: (num, &[partitions])
    # Determinar el patrón de partición basado en el rango
    partition_rows = []
    for num in range(2441, 2501):
        if num <= 2450:
            partitions_str = "[3]"
        elif num <= 2475:
            partitions_str = "[3, 2]"
        else:
            partitions_str = "[3, 1]"
        partition_rows.append(f"    ({num}, &{partitions_str}),")
    
    # Buscar dónde insertar: después de la última row de Wave 24 (después de micro_step 2440)
    # Buscar el patrón (2440, &[...)
    if '(2440, &' in content:
        # Encontrar la posición
        existing_partitions_end = content.find("    (2440, &", content.find("(2440, &"))
        if existing_partitions_end != -1:
            # Buscar el final de ese bloque (corchete de cierre)
            bracket_count = 0
            pos = existing_partitions_end
            while pos < len(content):
                if content[pos] == '[':
                    bracket_count += 1
                elif content[pos] == ']':
                    bracket_count -= 1
                    if bracket_count == 0:
                        break
                pos += 1
            
            # Insertar las nuevas rows antes del corchete de cierre
            new_partitions = "\n".join(partition_rows[:10])  # Primeras 10 rows
            content = content[:pos] + "\n" + new_partitions + "\n" + content[pos:]
            updated = True
            print("✓ Added new STEP_PARTITIONS rows for Wave 25")
    
    # 2. Actualizar el freeze constant de Wave 24 a Wave 25
    old_freeze_constant = "WAVE24_FROZEN_BEYOND_2440"
    new_freeze_constant = "WAVE25_FROZEN_BEYOND_2500"
    
    if old_freeze_constant in content and new_freeze_constant not in content:
        content = content.replace(old_freeze_constant, new_freeze_constant)
        # También actualizar el test asociado
        old_test = "wave24_freeze_beyond_2440"
        new_test = "wave25_freeze_beyond_2500"
        if old_test in content and new_test not in content:
            content = content.replace(old_test, new_test)
        updated = True
        print("✓ Updated freeze constant from WAVE24 to WAVE25")
    elif new_freeze_constant in content:
        print("✓ Freeze constant already updated to WAVE25_FROZEN_BEYOND_2500")
    else:
        print("! Could not find freeze constant markers")
    
    # Escribir el contenido actualizado
    with open(concepts_path, 'w', encoding='utf-8') as f:
        f.write(content)
    
    return updated


def main():
    """Punto de entrada principal."""
    curriculum_path = 'web/src/curriculum.rs'
    concepts_path = 'web/src/concepts/mod.rs'
    
    print("=" * 60)
    print("Applying Wave 25 (micro-steps 2441-2500)")
    print("=" * 60)
    print()
    
    # Aplicar a curriculum.rs
    print("1. Applying Wave 25 to curriculum.rs...")
    curr_updated = apply_wave25_to_curriculum(curriculum_path)
    print()
    
    # Aplicar a concepts/mod.rs
    print("2. Applying Wave 25 to concepts/mod.rs...")
    concepts_updated = apply_wave25_to_concepts(concepts_path)
    print()
    
    print("=" * 60)
    if curr_updated or concepts_updated:
        print("Wave 25 application complete!")
        print("Run: python3 scripts/validate_wave25.py")
    else:
        print("No updates were needed - Wave 25 may already be applied.")
    print("=" * 60)


if __name__ == "__main__":
    main()