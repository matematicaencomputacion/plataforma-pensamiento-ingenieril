package usecases

import "testing"

func TestEvaluateCode(t *testing.T) {
	t.Parallel()

	service := NewEvaluationService()

	tests := []struct {
		name    string
		code    string
		levelID int
		want    bool
	}{
		{
			name:    "aprueba cuando contiene print",
			code:    "print('hola')",
			levelID: 1,
			want:    true,
		},
		{
			name:    "desaprueba cuando no contiene print",
			code:    "x = 1 + 1",
			levelID: 1,
			want:    false,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			t.Parallel()

			got, err := service.EvaluateCode(tt.code, tt.levelID)
			if err != nil {
				t.Fatalf("error inesperado: %v", err)
			}
			if got != tt.want {
				t.Fatalf("resultado inesperado: got %v, want %v", got, tt.want)
			}
		})
	}
}
