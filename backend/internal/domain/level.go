package domain

// Level representa un nivel gamificado del recorrido de aprendizaje.
type Level struct {
	ID                  int
	Title               string
	Description         string
	RequiredTestsPassed bool
}
