import { component$, type QRL } from "@builder.io/qwik";
import {
  PYTHON_TYPE_CHIPS,
  type PythonTypeId,
} from "./python-type-catalog";

export type PythonTypeChipsProps = {
  activeType: PythonTypeId | null;
  onSelect$: QRL<(typeId: PythonTypeId | null) => void>;
};

/**
 * Chips exclusivos: un solo tipo "encendido".
 * La explicación permanente solo se muestra si hay uno activo.
 */
export const PythonTypeChips = component$((props: PythonTypeChipsProps) => {
  return (
    <div class="type-chips">
      <p class="type-chips__label">Tipos rápidos</p>
      <div class="type-chips__row" role="group" aria-label="Tipos de datos Python">
        {PYTHON_TYPE_CHIPS.map((chip) => {
          const on = props.activeType === chip.id;
          return (
            <button
              key={chip.id}
              type="button"
              class={`type-chips__chip${on ? " type-chips__chip--on" : ""}`}
              aria-pressed={on}
              title={
                on
                  ? `Apagar ejemplo ${chip.label}`
                  : `Insertar ejemplo ${chip.label}`
              }
              onClick$={() => {
                props.onSelect$(on ? null : chip.id);
              }}
            >
              <code>{chip.label}</code>
            </button>
          );
        })}
      </div>
      {props.activeType && (
        <p class="type-chips__explain" role="status">
          {
            PYTHON_TYPE_CHIPS.find((c) => c.id === props.activeType)
              ?.explanation
          }
        </p>
      )}
    </div>
  );
});
