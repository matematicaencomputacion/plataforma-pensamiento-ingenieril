# IngenierIA — Cloud Run image: Go API + Trunk/Leptos static SPA.
# Build context: monorepo root.
#
# NOTE: go.mod requires Go 1.25 — do not use golang:1.22 (build fails).
# Trunk release dist is COPIED into backend/static and go:embed'd so Cloud Run
# cannot accidentally serve web/index.html (fuente con data-trunk).

# ---- web (Wasm / Trunk) ----
FROM --platform=linux/amd64 rust:1-bookworm AS web-builder

ARG TRUNK_VERSION=0.21.14

RUN apt-get update \
    && apt-get install -y --no-install-recommends curl ca-certificates binaryen \
    && rm -rf /var/lib/apt/lists/* \
    && rustup target add wasm32-unknown-unknown \
    && curl -fsSL \
      "https://github.com/trunk-rs/trunk/releases/download/v${TRUNK_VERSION}/trunk-x86_64-unknown-linux-gnu.tar.gz" \
      | tar -xz \
    && install -m 755 trunk /usr/local/bin/trunk \
    && trunk --version \
    && wasm-opt --version

WORKDIR /src/web
COPY web/Cargo.toml web/Cargo.lock ./
COPY web/src ./src
COPY web/index.html web/styles.css web/Trunk.toml ./
COPY web/js ./js

# Clean slate — never ship the source index.html with data-trunk hooks.
RUN rm -rf dist \
    && env -u NO_COLOR trunk build --release \
    && test -f dist/index.html \
    && ! grep -q 'data-trunk' dist/index.html \
    && grep -q 'import init' dist/index.html \
    && ls dist/*.wasm >/dev/null \
    && ls -la dist

# ---- go API (embeds Trunk dist) ----
FROM --platform=linux/amd64 golang:1.25-alpine AS builder

RUN apk add --no-cache ca-certificates git

WORKDIR /src

COPY backend/go.mod backend/go.sum ./
RUN go mod download

COPY backend/ ./
# Replace placeholder static/ with Trunk release dist BEFORE compile (go:embed).
RUN rm -rf ./static && mkdir -p ./static
COPY --from=web-builder /src/web/dist/ ./static/
RUN test -f static/index.html \
    && ! grep -q 'data-trunk' static/index.html \
    && grep -q 'import init' static/index.html \
    && ls static/*.wasm >/dev/null \
    && ls -la static

RUN CGO_ENABLED=0 GOOS=linux GOARCH=amd64 \
    go build -trimpath -ldflags="-s -w" -o /out/server .

# ---- runtime ----
FROM --platform=linux/amd64 alpine:latest

RUN apk add --no-cache ca-certificates tzdata \
    && adduser -D -H -u 10001 appuser

WORKDIR /app

COPY --from=builder /src/data ./data
COPY --from=builder /out/server ./server
# Disk copy kept as fallback / DEBUG; primary serve path is go:embed inside server.
COPY --from=web-builder /src/web/dist/ ./static/

RUN test -f /app/static/index.html \
    && ! grep -q 'data-trunk' /app/static/index.html \
    && grep -q 'import init' /app/static/index.html

ENV DATA_DIR=/app/data \
    STATIC_DIR=/app/static \
    DATABASE_URL=sqlite:///tmp/ppi.db \
    PORT=8080

EXPOSE 8080

USER appuser

ENTRYPOINT ["/app/server"]
