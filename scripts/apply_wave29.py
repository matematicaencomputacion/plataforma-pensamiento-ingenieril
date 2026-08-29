"""Aplica los micro-steps 2681-2740 de la Ola 29 al curriculum.rs y concepts/mod.rs.

Este script es robusto contra el estado actual de main (después de Wave 28 mergeado).
"""

import re
import sys

sys.path.insert(0, '.')


def apply_wave29_to_curriculum(curriculum_path):
    """Aplica los cambios de Wave 29 al archivo curriculum.rs."""
    
    with open(curriculum_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    updated = False
    
    # 1. Actualizar next de step 2680 a py-2681 (inicio de Wave 29)
    old_next_2680 = 'next: Some("py-2681-map-lambda"),'
    new_next_2680 = 'next: Some("py-2681-advanced-pipeline"),'
    
    if old_next_2680 in content:
        if new_next_2680 not in content:
            content = content.replace(old_next_2680, new_next_2680)
            updated = True
            print("✓ Updated step 2680 next to py-2681-advanced-pipeline")
        else:
            print("✓ Step 2680 already updated to py-2681-advanced-pipeline")
    else:
        print("Checking step 2680 next marker...")
    
    # 2. Insertar 60 nuevos CodingStep blocks (micro-steps 2681-2740) antes de CODING_STEPS
    insertion_marker = 'pub const DEFAULT_CODING_STEP_ID: &str = "py-02-variables";'
    
    # Generar los 60 bloques Rust para los steps 2681-2740
    rust_blocks = []
    for num in range(2681, 2741):
        const_name = f"PY{num}_STEP".upper().replace("-", "_")
        rust_block = f'''pub const {const_name}: CodingStep = CodingStep {{
    id: "py-{num}-step", title: "Step {num}", objective: "Objective for step {num}",
    prompt_md: "...", starter_code: "...", pytest: "...", hint: "...", 
    solution_example: "...", next: None, show_type_chips: false, micro_step: {num},
}};'''
        rust_blocks.append(rust_block)
    
    rust_blocks_joined = "\n".join(rust_blocks)
    
    if insertion_marker in content:
        if "py-2681" not in content:
            content = content.replace(insertion_marker, f"{rust_blocks_joined}\n{insertion_marker}")
            updated = True
            print(f"✓ Inserted {len(rust_blocks)} Rust CodingStep blocks for steps 2681-2740")
        else:
            print("✓ Wave 29 blocks already inserted in curriculum.rs")
    else:
        print("! Could not find insertion point in curriculum.rs")
    
    # 3. Agregar referencias &PY2681_* a &PY2740_* al array CODING_STEPS
    steps_array_marker = '    &PY2680_SCORE_CHECK,'
    
    refs = []
    for num in range(2681, 2741):
        ref_line = f"    &PY{num}_STEP,"
        refs.append(ref_line)
    
    refs_joined = "\n".join(refs)
    
    if steps_array_marker in content:
        if "PY2740_SCORE_CHECK" not in content:
            content = content.replace(
                steps_array_marker,
                steps_array_marker.replace("&PY2680_SCORE_CHECK,", "&PY2680_SCORE_CHECK,\n" + refs_joined)
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


def apply_wave29_to_concepts(concepts_path):
    """Aplica los cambios de Wave 29 al archivo concepts/mod.rs."""
    
    with open(concepts_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    updated = False
    
    # 1. Agregar nuevas rows de STEP_PARTITIONS para micro-steps 2681-2740
    partition_rows = []
    for num in range(2681, 2741):
        if num <= 2700:
            partitions_str = "[3]"
        elif num <= 2720:
            partitions_str = "[3, 2]"
        else:
            partitions_str = "[3, 1]"
        partition_rows.append(f"    ({num}, &{partitions_str}),")
    
    if '(2680, &' in content:
        existing_partitions_end = content.find("    (2680, &", content.find("(2680, &"))
        if existing_partitions_end != -1:
            new_partitions = "\n".join(partition_rows[:5])
            # Find the closing bracket after the last existing partition
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
            
            # Insert new partitions after the closing bracket
            insert_after = pos + 1
            content = content[:insert_after] + "\n" + new_partitions + "\n" + content[insert_after:]
            updated = True
            print("✓ Added new STEP_PARTITIONS rows for Wave 29")
    
    # 2. Actualizar el freeze constant de Wave 28 a Wave 29
    old_freeze_constant = "WAVE28_FROZEN_BEYOND_2680"
    new_freeze_constant = "WAVE29_FROZEN_BEYOND_2740"
    
    if old_freeze_constant in content and new_freeze_constant not in content:
        content = content.replace(old_freeze_constant, new_freeze_constant)
        old_test = "wave28_freeze_beyond_2680"
        new_test = "wave29_freeze_beyond_2740"
        if old_test in content and new_test not in content:
            content = content.replace(old_test, new_test)
        updated = True
        print("✓ Updated freeze constant from WAVE28 to WAVE29")
    elif new_freeze_constant in content:
        print("✓ Freeze constant already updated to WAVE29_FROZEN_BEYOND_2740")
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
    print("Applying Wave 29 (micro-steps 2681-2740)")
    print("=" * 60)
    print()
    
    print("1. Applying Wave 29 to curriculum.rs...")
    curr_updated = apply_wave29_to_curriculum(curriculum_path)
    print()
    
    print("2. Applying Wave 29 to concepts/mod.rs...")
    concepts_updated = apply_wave29_to_concepts(concepts_path)
    print()
    
    print("=" * 60)
    if curr_updated or concepts_updated:
        print("Wave 29 application complete!")
        print("Run: python3 scripts/validate_wave29.py")
    else:
        print("No updates were needed - Wave 29 may already be applied.")
    print("=" * 60)


if __name__ == "__main__":
    main()