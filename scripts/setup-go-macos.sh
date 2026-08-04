#!/bin/zsh
# Bootstrap Go 1.25.0 para macOS Apple Silicon (darwin/arm64).
# Uso: ./scripts/setup-go-macos.sh
set -euo pipefail

GO_VERSION="1.25.0"
ARCH="$(uname -m)"
OS="$(uname -s)"

if [[ "$OS" != "Darwin" ]]; then
  echo "Este script está pensado para macOS. En Linux/WSL usa GOTOOLCHAIN=auto con go.mod."
  exit 0
fi

if [[ "$ARCH" != "arm64" ]]; then
  echo "Advertencia: arquitectura $ARCH (se esperaba arm64). Continuaré con el tarball darwin-${ARCH}."
fi

TARBALL="go${GO_VERSION}.darwin-${ARCH}.tar.gz"
URL="https://go.dev/dl/${TARBALL}"
INSTALL_DIR="${HOME}/.local/go-${GO_VERSION}"
TMP_DIR="$(mktemp -d)"

cleanup() { rm -rf "$TMP_DIR"; }
trap cleanup EXIT

if command -v go >/dev/null 2>&1; then
  CURRENT="$(go env GOVERSION 2>/dev/null || true)"
  if [[ "$CURRENT" == "go${GO_VERSION}" ]]; then
    echo ">> Go ${GO_VERSION} ya activo: $(go version)"
    exit 0
  fi
fi

echo ">> Descargando ${URL}"
curl -fsSL "$URL" -o "${TMP_DIR}/${TARBALL}"

echo ">> Instalando en ${INSTALL_DIR}"
rm -rf "$INSTALL_DIR"
mkdir -p "$INSTALL_DIR"
tar -C "$INSTALL_DIR" --strip-components=1 -xzf "${TMP_DIR}/${TARBALL}"

PROFILE_SNIPPET='export PATH="$HOME/.local/go-'"${GO_VERSION}"'/bin:$PATH"'
ZSHRC="${HOME}/.zshrc"
if ! grep -Fq "go-${GO_VERSION}/bin" "$ZSHRC" 2>/dev/null; then
  echo "" >> "$ZSHRC"
  echo "# PPI — Go ${GO_VERSION} (Apple Silicon)" >> "$ZSHRC"
  echo "$PROFILE_SNIPPET" >> "$ZSHRC"
  echo ">> Añadido PATH a ${ZSHRC}"
fi

export PATH="${INSTALL_DIR}/bin:$PATH"
echo ">> $(go version)"
echo ">> Reinicia la terminal o ejecuta: source ~/.zshrc"
