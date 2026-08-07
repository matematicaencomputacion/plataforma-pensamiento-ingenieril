/** Catálogo de tipos para chips del paso Variables. */

export type PythonTypeId = "str" | "int" | "bool" | "float";

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
      "str — texto (cadena). Se escribe entre comillas: nombres, mensajes, rutas.",
    sampleCode: 'nombre = "Ana"\nprint(nombre)\nprint(type(nombre))\n',
  },
  {
    id: "int",
    label: "int",
    explanation:
      "int — entero. Números enteros sin parte decimal: edad, cantidad, índices.",
    sampleCode: "edad = 25\nprint(edad)\nprint(type(edad))\n",
  },
  {
    id: "bool",
    label: "bool",
    explanation:
      "bool — booleano. Solo dos valores: True o False (condiciones y flags).",
    sampleCode: "activo = True\nprint(activo)\nprint(type(activo))\n",
  },
  {
    id: "float",
    label: "float",
    explanation:
      "float — decimal. Números con punto: altura, precio, promedios.",
    sampleCode: "altura = 1.75\nprint(altura)\nprint(type(altura))\n",
  },
];

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
