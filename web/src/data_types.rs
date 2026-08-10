//! Data-type acronyms for the Paso 2 enunciado chip rail.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DataTypeChip {
    /// Button label (e.g. `str`).
    pub id: &'static str,
    /// Full English name.
    pub name_en: &'static str,
    /// Spanish gloss.
    pub gloss_es: &'static str,
    /// Optional CSS tone token (`str`, `num`, `col`, …).
    pub tone: &'static str,
}

/// Tipos de datos y estructuras — orden pedagógico (str primero).
pub const DATA_TYPE_CHIPS: &[DataTypeChip] = &[
    DataTypeChip {
        id: "str",
        name_en: "String",
        gloss_es: "Cadena de texto",
        tone: "str",
    },
    DataTypeChip {
        id: "int",
        name_en: "Integer",
        gloss_es: "Número entero",
        tone: "num",
    },
    DataTypeChip {
        id: "bool",
        name_en: "Boolean",
        gloss_es: "Booleano: true o false (también: bln)",
        tone: "bool",
    },
    DataTypeChip {
        id: "char",
        name_en: "Character",
        gloss_es: "Carácter individual (también: chr)",
        tone: "str",
    },
    DataTypeChip {
        id: "float",
        name_en: "Floating-point",
        gloss_es: "Número decimal / punto flotante (también: flt)",
        tone: "num",
    },
    DataTypeChip {
        id: "dbl",
        name_en: "Double",
        gloss_es: "Decimal de doble precisión",
        tone: "num",
    },
    DataTypeChip {
        id: "num",
        name_en: "Number",
        gloss_es: "Número genérico",
        tone: "num",
    },
    DataTypeChip {
        id: "arr",
        name_en: "Array",
        gloss_es: "Arreglo, vector o lista fija",
        tone: "col",
    },
    DataTypeChip {
        id: "vec",
        name_en: "Vector",
        gloss_es: "Colección dinámica o lista",
        tone: "col",
    },
    DataTypeChip {
        id: "list",
        name_en: "List",
        gloss_es: "Lista (también: lst)",
        tone: "col",
    },
    DataTypeChip {
        id: "dict",
        name_en: "Dictionary",
        gloss_es: "Diccionario / mapa de clave-valor",
        tone: "map",
    },
    DataTypeChip {
        id: "map",
        name_en: "Map",
        gloss_es: "Mapa o estructura clave-valor",
        tone: "map",
    },
    DataTypeChip {
        id: "obj",
        name_en: "Object",
        gloss_es: "Objeto",
        tone: "obj",
    },
    DataTypeChip {
        id: "struct",
        name_en: "Structure",
        gloss_es: "Estructura de datos personalizada",
        tone: "obj",
    },
    DataTypeChip {
        id: "enum",
        name_en: "Enumeration",
        gloss_es: "Conjunto de constantes con nombre",
        tone: "obj",
    },
    DataTypeChip {
        id: "fn",
        name_en: "Function",
        gloss_es: "Función o puntero a función (también: func)",
        tone: "fn",
    },
    DataTypeChip {
        id: "ptr",
        name_en: "Pointer",
        gloss_es: "Puntero a memoria",
        tone: "fn",
    },
    DataTypeChip {
        id: "buf",
        name_en: "Buffer",
        gloss_es: "Búfer de memoria o datos binarios",
        tone: "fn",
    },
];

pub fn chip_by_id(id: &str) -> Option<&'static DataTypeChip> {
    DATA_TYPE_CHIPS.iter().find(|c| c.id == id)
}

pub fn chip_explanation(chip: &DataTypeChip) -> String {
    format!("{}: {} ({})", chip.id, chip.name_en, chip.gloss_es)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_is_first_and_catalog_complete() {
        assert_eq!(DATA_TYPE_CHIPS[0].id, "str");
        assert_eq!(DATA_TYPE_CHIPS.len(), 18);
        assert!(chip_by_id("obj").is_some());
        assert!(chip_by_id("nope").is_none());
    }

    #[test]
    fn explanation_format() {
        let str_chip = &DATA_TYPE_CHIPS[0];
        let text = chip_explanation(str_chip);
        assert!(text.contains("String"));
        assert!(text.contains("Cadena de texto"));
    }
}
