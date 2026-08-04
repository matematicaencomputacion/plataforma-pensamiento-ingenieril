package assets

import _ "embed"

// CurriculumJSON contiene el curriculum embebido en el binario (go:embed).
//
//go:embed curriculum.json
var CurriculumJSON []byte
