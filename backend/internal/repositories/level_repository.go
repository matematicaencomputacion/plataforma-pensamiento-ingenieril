package repositories

import "github.com/tu-usuario/plataforma-edu-backend/internal/domain"

// LevelRepository define el puerto de acceso a niveles/retos.
type LevelRepository interface {
	GetByID(id int) (domain.Level, error)
	GetCurrent() (domain.Level, error)
	List() ([]domain.Level, error)
}
