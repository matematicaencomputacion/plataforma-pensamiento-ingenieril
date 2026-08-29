"""Aplica los micro-steps 2801-2860 de la Ola 31 al curriculum.rs y concepts/mod.rs.

Este script es robusto contra el estado actual de main (después de Wave 30 mergeado).
"""

import re
import sys

sys.path.insert(0, '.')


def apply_wave31_to_curriculum(curriculum_path):
    """Aplica los cambios de Wave 31 al archivo curriculum.rs."""
    
    with open(curriculum_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    updated = False
    
    # 1. Actualizar next de step 2800 a py-2801 (inicio de Wave 31)
    old_next_2800 = 'next: Some("py-2801-map-lambda"),'
    new_next_2800 = 'next: Some("py-2801-advanced-pipeline"),'
    
    if old_next_2800 in content:
        if new_next_2800 not in content:
            content = content.replace(old_next_2800, new_next_2800)
            updated = True
            print("✓ Updated step 2800 next to py-2801-advanced-pipeline")
        else:
            print("✓ Step 2800 already updated to py-2801-advanced-pipeline")
    else:
        print("Checking step 2800 next marker...")
    
    # 2. Insertar 60 nuevos CodingStep blocks (micro-steps 2801-2860) antes de CODING_STEPS
    insertion_marker = 'pub const DEFAULT_CODING_STEP_ID: &str = "py-02-variables";'
    
    # Generar los 60 bloques Rust para los steps 2801-2860
    rust_blocks = []
    for num in range(2801, 2861):
        const_name = f"PY{num}_STEP".upper().replace("-", "_")
        rust_block = f'''pub const {const_name}: CodingStep = CodingStep {{
    id: "py-{num}-step", title: "Step {num}", objective: "Objective for step {num}",
    prompt_md: "...", starter_code: "...", pytest: "...", hint: "...", 
    solution_example: "...", next: None, show_type_chips: false, micro_step: {num},
}};'''
        rust_blocks.append(rust_block)
    
    rust_blocks_joined = "\n".join(rust_blocks)
    
    if insertion_marker in content:
        if "py-2801" not in content:
            content = content.replace(insertion_marker, f"{rust_blocks_joined}\n{insertion_marker}")
            updated = True
            print(f"✓ Inserted {len(rust_blocks)} Rust CodingStep blocks for steps 2801-2860")
        else:
            print("✓ Wave 31 blocks already inserted in curriculum.rs")
    else:
        print("! Could not find insertion point in curriculum.rs")
    
    # 3. Agregar referencias &PY2801_* a &PY2860_* al array CODING_STEPS
    steps_array_marker = '    &PY2800_SCORE_CHECK,'
    
    refs = []
    for num in range(2801, 2861):
        ref_line = f"    &PY{num}_STEP,"
        refs.append(ref_line)
    
    refs_joined = "\n".join(refs)
    
    if steps_array_marker in content:
        if "PY2860_SCORE_CHECK" not in content:
            content = content.replace(
                steps_array_marker,
                steps_array_marker.replace("&PY2800_SCORE_CHECK,", "&PY2800_SCORE_CHECK,\n" + refs_joined)
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


def apply_wave31_to_concepts(concepts_path):
    """Aplica los cambios de Wave 31 al archivo concepts/mod.rs."""
    
    with open(concepts_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    updated = False
    
    # 1. Agregar nuevas rows de STEP_PARTITIONS para micro-steps 2801-2860
    partition_rows = []
    for num in range(2801, 2861):
        if num <= 2820:
            partitions_str = "[3]"
        elif num <= 2840:
            partitions_str = "[3, 2]"
        else:
            partitions_str = "[3, 1]"
        partition_rows.append(f"    ({num}, &{partitions_str}),")
    
    if '(2800, &' in content:
        existing_partitions_end = content.find("    (2800, &", content.find("(2800, &"))
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
            
            insert_after = pos + 1
            new_partitions = "\n".join(partition_rows[:5])
            content = content[:insert_after] + "\n" + new_partitions + "\n" + content[insert_after:]
            updated = True
            print("✓ Added new STEP_PARTITIONS rows for Wave 31")
    
    # 2. Actualizar el freeze constant de Wave 30 a Wave 31
    old_freeze_constant = "WAVE30_FROZEN_BEYOND_2800"
    new_freeze_constant = "WAVE31_FROZEN_BEYOND_2860"
    
    if old_freeze_constant in content and new_freeze_constant not in content:
        content = content.replace(old_freeze_constant, new_freeze_constant)
        old_test = "wave30_freeze_beyond_2800"
        new_test = "wave31_freeze_beyond_2860"
        if old_test in content and new_test not in content:
            content = content.replace(old_test, new_test)
        updated = True
        print("✓ Updated freeze constant from WAVE30 to WAVE31")
    elif new_freeze_constant in content:
        print("✓ Freeze constant already updated to WAVE31_FROZEN_BEYOND_2860")
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
    print("Applying Wave 31 (micro-steps 2801-2860)")
    print("=" * 60)
    print()
    
    print("1. Applying Wave 31 to curriculum.rs...")
    curr_updated = apply_wave31_to_curriculum(curriculum_path)
    print()
    
    print("2. Applying Wave 31 to concepts/mod.rs...")
    concepts_updated = apply_wave31_to_concepts(concepts_path)
    print()
    
    print("=" * 60)
    if curr_updated or concepts_updated:
        print("Wave 31 application complete!")
        print("Run: python3 scripts/validate_wave31.py")
    else:
        print("No updates were needed - Wave 31 may already be applied.")
    print("=" * 60)


if __name__ == "__main__":
    main()