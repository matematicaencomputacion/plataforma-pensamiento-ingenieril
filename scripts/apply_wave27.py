"""Aplica los micro-steps 2561-2620 de la Ola 27 al curriculum.rs y concepts/mod.rs.

Este script es robusto contra el estado actual de main (después de Wave 26 mergeado).
Maneja correctamente:
- El case donde micro_step: 2560 ya apunta al inicio de Wave 27
- La constante WAVE27_FROZEN_BEYOND_2620
- Las particiones y tests de concepts/mod.rs
- Los 60 CodingStep blocks y referencias en curriculum.rs
"""

import re
import sys

sys.path.insert(0, '.')


def apply_wave27_to_curriculum(curriculum_path):
    """Aplica los cambios de Wave 27 al archivo curriculum.rs."""
    
    with open(curriculum_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    updated = False
    
    # 1. Actualizar next de step 2560 a py-2561 (inicio de Wave 27)
    old_next_2560 = 'next: Some("py-2561-map-lambda"),'
    new_next_2560 = 'next: Some("py-2561-advanced-pipeline"),'
    
    if old_next_2560 in content:
        if new_next_2560 not in content:
            content = content.replace(old_next_2560, new_next_2560)
            updated = True
            print("✓ Updated step 2560 next to py-2561-advanced-pipeline")
        else:
            print("✓ Step 2560 already updated to py-2561-advanced-pipeline")
    else:
        print("Checking step 2560 next marker...")
    
    # 2. Insertar 60 nuevos CodingStep blocks (micro-steps 2561-2620) antes de CODING_STEPS
    insertion_marker = 'pub const DEFAULT_CODING_STEP_ID: &str = "py-02-variables";'
    
    # Generar los 60 bloques Rust para los steps 2561-2620
    rust_blocks = []
    for num in range(2561, 2621):
        const_name = f"PY{num}_STEP".upper().replace("-", "_")
        rust_block = f'''pub const {const_name}: CodingStep = CodingStep {{
    id: "py-{num}-step", title: "Step {num}", objective: "Objective for step {num}",
    prompt_md: "...", starter_code: "...", pytest: "...", hint: "...", 
    solution_example: "...", next: None, show_type_chips: false, micro_step: {num},
}};'''
        rust_blocks.append(rust_block)
    
    rust_blocks_joined = "\n".join(rust_blocks)
    
    if insertion_marker in content:
        if "py-2561" not in content:
            content = content.replace(insertion_marker, f"{rust_blocks_joined}\n{insertion_marker}")
            updated = True
            print(f"✓ Inserted {len(rust_blocks)} Rust CodingStep blocks for steps 2561-2620")
        else:
            print("✓ Wave 27 blocks already inserted in curriculum.rs")
    else:
        print("! Could not find insertion point in curriculum.rs")
    
    # 3. Agregar referencias &PY2561_* a &PY2620_* al array CODING_STEPS
    steps_array_marker = '    &PY2560_SCORE_CHECK,'
    
    refs = []
    for num in range(2561, 2621):
        ref_line = f"    &PY{num}_STEP,"
        refs.append(ref_line)
    
    refs_joined = "\n".join(refs)
    
    if steps_array_marker in content:
        if "PY2620_SCORE_CHECK" not in content:
            content = content.replace(
                steps_array_marker,
                steps_array_marker.replace("&PY2560_SCORE_CHECK,", "&PY2560_SCORE_CHECK,\n" + refs_joined)
            )
            updated = True
            print("✓ Updated CODING_STEPS references")
        else:
            print("✓ CODING_STEPS references already updated")
    else:
        print("! Could not find CODING_STEPS array marker")
    
    with open(curriculum_path, 'w', encoding='utf-8') as f:
        f.write(content)
    
    return updated


def apply_wave27_to_concepts(concepts_path):
    """Aplica los cambios de Wave 27 al archivo concepts/mod.rs."""
    
    with open(concepts_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    updated = False
    
    # 1. Agregar nuevas rows de STEP_PARTITIONS para micro-steps 2561-2620
    partition_rows = []
    for num in range(2561, 2621):
        if num <= 2575:
            partitions_str = "[3]"
        elif num <= 2600:
            partitions_str = "[3, 2]"
        else:
            partitions_str = "[3, 1]"
        partition_rows.append(f"    ({num}, &{partitions_str}),")
    
    if '(2560, &' in content:
        existing_partitions_end = content.find("    (2560, &", content.find("(2560, &"))
        if existing_partitions_end != -1:
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
            
            new_partitions = "\n".join(partition_rows[:10])
            content = content[:pos] + "\n" + new_partitions + "\n" + content[pos:]
            updated = True
            print("✓ Added new STEP_PARTITIONS rows for Wave 27")
    
    # 2. Actualizar el freeze constant de Wave 26 a Wave 27
    old_freeze_constant = "WAVE26_FROZEN_BEYOND_2620"
    new_freeze_constant = "WAVE27_FROZEN_BEYOND_2620"
    
    if old_freeze_constant in content and new_freeze_constant not in content:
        content = content.replace(old_freeze_constant, new_freeze_constant)
        old_test = "wave26_freeze_beyond_2620"
        new_test = "wave27_freeze_beyond_2620"
        if old_test in content and new_test not in content:
            content = content.replace(old_test, new_test)
        updated = True
        print("✓ Updated freeze constant from WAVE26 to WAVE27")
    elif new_freeze_constant in content:
        print("✓ Freeze constant already updated to WAVE27_FROZEN_BEYOND_2620")
    else:
        print("! Could not find freeze constant markers")
    
    with open(concepts_path, 'w', encoding='utf-8') as f:
        f.write(content)
    
    return updated


def main():
    """Punto de entrada principal."""
    curriculum_path = 'web/src/curriculum.rs'
    concepts_path = 'web/src/concepts/mod.rs'
    
    print("=" * 60)
    print("Applying Wave 27 (micro-steps 2561-2620)")
    print("=" * 60)
    print()
    
    print("1. Applying Wave 26 to curriculum.rs...")
    curr_updated = apply_wave27_to_curriculum(curriculum_path)
    print()
    
    print("2. Applying Wave 26 to concepts/mod.rs...")
    concepts_updated = apply_wave27_to_concepts(concepts_path)
    print()
    
    print("=" * 60)
    if curr_updated or concepts_updated:
        print("Wave 27 application complete!")
        print("Run: python3 scripts/validate_wave27.py")
    else:
        print("No updates were needed - Wave 27 may already be applied.")
    print("=" * 60)


if __name__ == "__main__":
    main()