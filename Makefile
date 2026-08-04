# PPI — Makefile (optimizado para macOS Apple Silicon / ARM64)
# Uso típico desde la raíz del monorepo:
#   make toolchain   # asegura Go 1.25
#   make test
#   make fmt
#   make build

SHELL := /bin/zsh
.SHELLFLAGS := -euo pipefail -c

BACKEND_DIR := backend
BIN_DIR     := $(BACKEND_DIR)/bin
BIN_NAME    := ppi-api
MODULE      := github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend

# Detecta host; en Apple Silicon fuerza arm64 nativo.
UNAME_S := $(shell uname -s)
UNAME_M := $(shell uname -m)

ifeq ($(UNAME_S),Darwin)
  export GOOS ?= darwin
  ifeq ($(UNAME_M),arm64)
    export GOARCH ?= arm64
  else
    export GOARCH ?= amd64
  endif
else
  export GOOS ?= $(shell go env GOOS 2>/dev/null || echo linux)
  export GOARCH ?= $(shell go env GOARCH 2>/dev/null || echo amd64)
endif

export GOTOOLCHAIN ?= auto
export CGO_ENABLED ?= 0

GO ?= go
GOFLAGS ?=

.PHONY: help toolchain fmt vet test build run clean openspec-validate ci

help: ## Muestra targets disponibles
	@awk 'BEGIN {FS = ":.*##"; printf "\nTargets PPI:\n"} /^[a-zA-Z0-9_-]+:.*?##/ { printf "  %-18s %s\n", $$1, $$2 }' $(MAKEFILE_LIST)

toolchain: ## Instala/activa Go 1.25 (macOS ARM64 o via GOTOOLCHAIN)
	@if [ "$(UNAME_S)" = "Darwin" ]; then \
	  ./scripts/setup-go-macos.sh; \
	else \
	  echo ">> Entorno no-Darwin: usando GOTOOLCHAIN=auto con go.mod (1.25.0)"; \
	  cd $(BACKEND_DIR) && $(GO) version; \
	fi

fmt: ## gofmt + go mod tidy
	@cd $(BACKEND_DIR) && $(GO) fmt ./...
	@cd $(BACKEND_DIR) && $(GO) mod tidy

vet: ## go vet
	@cd $(BACKEND_DIR) && $(GO) vet ./...

test: ## Ejecuta tests unitarios del backend
	@cd $(BACKEND_DIR) && $(GO) test $(GOFLAGS) ./...

build: ## Compila el binario del API (backend/bin/ppi-api)
	@mkdir -p $(BIN_DIR)
	@cd $(BACKEND_DIR) && \
	  $(GO) build $(GOFLAGS) -trimpath -ldflags="-s -w" \
	    -o bin/$(BIN_NAME) .
	@echo ">> built $(BIN_DIR)/$(BIN_NAME) ($(GOOS)/$(GOARCH))"

run: build ## Compila y levanta el API en :8080
	@cd $(BACKEND_DIR) && ./bin/$(BIN_NAME)

clean: ## Limpia binarios
	@rm -rf $(BIN_DIR)

openspec-validate: ## Valida el change PPI 1.1
	@openspec validate ppi-1-1-foundations --no-interactive

ci: fmt vet test build openspec-validate ## Pipeline local rápido
	@echo ">> CI local OK — module $(MODULE)"
