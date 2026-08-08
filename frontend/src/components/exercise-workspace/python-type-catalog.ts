/** Catálogo de tipos para chips del paso Variables. */

export type PythonTypeId = "str" | "int" | "float" | "bool";

export type PythonTypeChip = {
  id: PythonTypeId;
  label: string;
  explanation: string;
  sampleCode: string;
};

export const PYTHON_TYPE_CHIPS: PythonTypeChip[] = [
  {
    id: "str",
    label: "str",
    explanation:
      "str (string) guarda texto. Siempre va entre comillas simples o dobles: nombres, mensajes, rutas.",
    sampleCode: 'nombre = "Ana"\nprint(nombre)\nprint(type(nombre))  # <class \'str\'>',
  },
  {
    id: "int",
    label: "int",
    explanation:
      "int guarda un entero sin decimales: edad, cantidad, índices. Se escribe sin comillas.",
    sampleCode: "edad = 25\nprint(edad)\nprint(type(edad))  # <class 'int'>",
  },
  {
    id: "float",
    label: "float",
    explanation:
      "float guarda un número decimal (con punto): altura, precio, promedios.",
    sampleCode: "altura = 1.75\nprint(altura)\nprint(type(altura))  # <class 'float'>",
  },
  {
    id: "bool",
    label: "bool",
    explanation:
      "bool solo admite True o False (con mayúscula). Sirve para condiciones y flags.",
    sampleCode: "activo = True\nprint(activo)\nprint(type(activo))  # <class 'bool'>",
  },
];

export function getPythonTypeChip(
  id: PythonTypeId | null,
): PythonTypeChip | undefined {
  if (!id) {
    return undefined;
  }
  return PYTHON_TYPE_CHIPS.find((chip) => chip.id === id);
}

/** Hints hover sobre identificadores del micro-reto Variables. */
export type VariableHoverHint = {
  typeLabel: string;
  blurb: string;
};

export const PY02_VARIABLE_HINTS: Record<string, VariableHoverHint> = {
  nombre: {
    typeLabel: "str",
    blurb: "Variable de tipo str (texto / cadena de caracteres).",
  },
  edad: {
    typeLabel: "int",
    blurb: "Variable de tipo int (número entero).",
  },
};

/** Steps donde se muestra la barra de tipos en el enunciado. */
export function stepShowsPythonTypeChips(stepId: string, title: string): boolean {
  if (stepId === "py-02-variables" || stepId === "py-07-variables") {
    return true;
  }
  return /variable/i.test(title);
}
