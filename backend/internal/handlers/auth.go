package handlers

import (
	"encoding/json"
	"errors"
	"log"
	"net/http"
	"strings"

	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/domain"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/repositories"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/usecases"
)

type authCredentialsRequest struct {
	Email    string `json:"email"`
	Password string `json:"password"`
}

type authSuccessResponse struct {
	User  domain.PublicUser `json:"user"`
	Token string            `json:"token"`
}

// AuthHandler endpoints de registro, login, logout, /me y perfil.
type AuthHandler struct {
	service *usecases.AuthService
}

func NewAuthHandler(service *usecases.AuthService) *AuthHandler {
	return &AuthHandler{service: service}
}

func (h *AuthHandler) Register(w http.ResponseWriter, r *http.Request) {
	var req authCredentialsRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		writeJSONError(w, "JSON de entrada inválido", http.StatusBadRequest)
		return
	}

	out, err := h.service.Register(r.Context(), req.Email, req.Password)
	if err != nil {
		writeAuthError(w, err)
		return
	}
	writeJSON(w, http.StatusCreated, authSuccessResponse{User: out.User, Token: out.Token})
}

func (h *AuthHandler) Login(w http.ResponseWriter, r *http.Request) {
	var req authCredentialsRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		writeJSONError(w, "JSON de entrada inválido", http.StatusBadRequest)
		return
	}

	out, err := h.service.Login(r.Context(), req.Email, req.Password)
	if err != nil {
		writeAuthError(w, err)
		return
	}
	writeJSON(w, http.StatusOK, authSuccessResponse{User: out.User, Token: out.Token})
}

func (h *AuthHandler) Logout(w http.ResponseWriter, r *http.Request) {
	w.WriteHeader(http.StatusNoContent)
}

type forgotPasswordRequest struct {
	Email string `json:"email"`
}

type forgotPasswordResponse struct {
	Message    string `json:"message"`
	ResetToken string `json:"resetToken,omitempty"`
}

type resetPasswordRequest struct {
	Token    string `json:"token"`
	Password string `json:"password"`
}

func (h *AuthHandler) ForgotPassword(w http.ResponseWriter, r *http.Request) {
	var req forgotPasswordRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		writeJSONError(w, "JSON de entrada inválido", http.StatusBadRequest)
		return
	}
	out, err := h.service.ForgotPassword(r.Context(), req.Email)
	if err != nil {
		writeAuthError(w, err)
		return
	}
	writeJSON(w, http.StatusOK, forgotPasswordResponse{
		Message:    out.Message,
		ResetToken: out.ResetToken,
	})
}

func (h *AuthHandler) ResetPassword(w http.ResponseWriter, r *http.Request) {
	var req resetPasswordRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		writeJSONError(w, "JSON de entrada inválido", http.StatusBadRequest)
		return
	}
	out, err := h.service.ResetPassword(r.Context(), req.Token, req.Password)
	if err != nil {
		writeAuthError(w, err)
		return
	}
	writeJSON(w, http.StatusOK, authSuccessResponse{User: out.User, Token: out.Token})
}

func (h *AuthHandler) Me(w http.ResponseWriter, r *http.Request) {
	token, ok := bearerToken(r)
	if !ok {
		writeJSONError(w, "no autorizado", http.StatusUnauthorized)
		return
	}
	user, err := h.service.Me(r.Context(), token)
	if err != nil {
		writeAuthError(w, err)
		return
	}
	writeJSON(w, http.StatusOK, user)
}

func (h *AuthHandler) GetProfile(w http.ResponseWriter, r *http.Request) {
	token, ok := bearerToken(r)
	if !ok {
		writeJSONError(w, "no autorizado", http.StatusUnauthorized)
		return
	}
	profile, err := h.service.GetProfile(r.Context(), token)
	if err != nil {
		writeAuthError(w, err)
		return
	}
	writeJSON(w, http.StatusOK, profile)
}

func (h *AuthHandler) UpdateProfile(w http.ResponseWriter, r *http.Request) {
	token, ok := bearerToken(r)
	if !ok {
		writeJSONError(w, "no autorizado", http.StatusUnauthorized)
		return
	}

	var wire profileUpdateWire
	dec := json.NewDecoder(r.Body)
	if err := dec.Decode(&wire); err != nil {
		writeJSONError(w, "JSON de entrada inválido: se esperaba lifePurpose, urgency, vision5Years, techStack", http.StatusBadRequest)
		return
	}

	req := wire.toLearnerProfile()
	profile, err := h.service.UpdateProfile(r.Context(), token, req)
	if err != nil {
		writeAuthError(w, err)
		return
	}
	writeJSON(w, http.StatusOK, profile)
}

// profileUpdateWire acepta el contrato canónico y aliases de la síntesis UI.
type profileUpdateWire struct {
	LifePurpose  string `json:"lifePurpose"`
	Urgency      string `json:"urgency"`
	Vision5Years string `json:"vision5Years"`
	TechStack    string `json:"techStack"`
	Purpose      string `json:"purpose"`
	Vision       string `json:"vision"`
	Stack        string `json:"stack"`
}

func (w profileUpdateWire) toLearnerProfile() domain.LearnerProfile {
	life := strings.TrimSpace(w.LifePurpose)
	if life == "" {
		life = strings.TrimSpace(w.Purpose)
	}
	vision := strings.TrimSpace(w.Vision5Years)
	if vision == "" {
		vision = strings.TrimSpace(w.Vision)
	}
	stack := strings.TrimSpace(w.TechStack)
	if stack == "" {
		stack = strings.TrimSpace(w.Stack)
	}
	return domain.LearnerProfile{
		LifePurpose:  life,
		Urgency:      strings.TrimSpace(w.Urgency),
		Vision5Years: vision,
		TechStack:    stack,
	}
}

// Profile despacha GET (rehidratación) y PUT/POST (persistencia) en /api/user/profile.
func (h *AuthHandler) Profile(w http.ResponseWriter, r *http.Request) {
	switch r.Method {
	case http.MethodGet:
		h.GetProfile(w, r)
	case http.MethodPut, http.MethodPost:
		h.UpdateProfile(w, r)
	default:
		w.Header().Set("Allow", "GET, PUT, POST, OPTIONS")
		writeJSONError(w, "método no permitido", http.StatusMethodNotAllowed)
	}
}

func bearerToken(r *http.Request) (string, bool) {
	h := strings.TrimSpace(r.Header.Get("Authorization"))
	if h == "" {
		return "", false
	}
	const prefix = "Bearer "
	if !strings.HasPrefix(h, prefix) {
		return "", false
	}
	token := strings.TrimSpace(strings.TrimPrefix(h, prefix))
	return token, token != ""
}

func writeAuthError(w http.ResponseWriter, err error) {
	switch {
	case errors.Is(err, domain.ErrInvalidEmail), errors.Is(err, domain.ErrInvalidPassword), errors.Is(err, domain.ErrEmptyProfile), errors.Is(err, domain.ErrInvalidResetToken):
		writeJSONError(w, err.Error(), http.StatusBadRequest)
	case errors.Is(err, repositories.ErrEmailTaken):
		writeJSONError(w, "email ya registrado", http.StatusConflict)
	case errors.Is(err, domain.ErrInvalidCredentials):
		writeJSONError(w, "credenciales inválidas", http.StatusUnauthorized)
	case errors.Is(err, domain.ErrUnauthorized):
		writeJSONError(w, "no autorizado", http.StatusUnauthorized)
	default:
		log.Printf("auth error: %v", err)
		writeJSONError(w, "error interno de autenticación", http.StatusInternalServerError)
	}
}

func writeJSONError(w http.ResponseWriter, msg string, status int) {
	writeJSON(w, status, map[string]string{"error": msg})
}

func writeJSON(w http.ResponseWriter, status int, payload any) {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(status)
	if err := json.NewEncoder(w).Encode(payload); err != nil {
		log.Printf("error al escribir JSON: %v", err)
	}
}
