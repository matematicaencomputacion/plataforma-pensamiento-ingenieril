# IngenierIA — Cloud Run image: Go API + Trunk/Leptos static SPA.
# Build context: monorepo root.
#
# Order is intentional:
#   1) web-builder runs `trunk build --release` → dist/
#   2) go builder COPIES dist into backend/static/ THEN `go build` (go:embed)
# Pass --build-arg PPI_BUILD_ID=<gitsha> to bust BuildKit cache on every deploy.

# ---- web (Wasm / Trunk) ----
FROM --platform=linux/amd64 rust:1-bookworm AS web-builder

ARG TRUNK_VERSION=0.21.14
# Cache bust: Cloud Build / Developer Connect should pass the git SHA.
ARG PPI_BUILD_ID=stamp-v3-20260810-table-grow
# Bookworm apt binaryen is too old for wasm-bindgen externref + wasm-opt.
# data-wasm-opt="0" in index.html — binaryen not required.

RUN apt-get update \
    && apt-get install -y --no-install-recommends curl ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && rustup target add wasm32-unknown-unknown \
    && curl -fsSL \
      "https://github.com/trunk-rs/trunk/releases/download/v${TRUNK_VERSION}/trunk-x86_64-unknown-linux-gnu.tar.gz" \
      | tar -xz \
    && install -m 755 trunk /usr/local/bin/trunk \
    && trunk --version

WORKDIR /src/web
COPY web/Cargo.toml web/Cargo.lock ./
COPY web/.cargo ./.cargo
COPY web/src ./src
COPY web/index.html web/styles.css web/Trunk.toml ./
COPY web/js ./js

# Clean slate — never ship the source index.html with data-trunk hooks.
# PPI_BUILD_ID in the RUN line invalidates Docker layer cache when SHA changes.
RUN rm -rf dist target \
    && echo "PPI_BUILD_ID=${PPI_BUILD_ID}" \
    && env -u NO_COLOR trunk build --release \
    && test -f dist/index.html \
    && ! grep -q 'data-trunk' dist/index.html \
    && grep -q 'import init' dist/index.html \
    && ls dist/*.wasm >/dev/null \
    && printf '%s\n' "${PPI_BUILD_ID}" > dist/ppi-build.txt \
    && js="$(ls dist/*-web-*.js | head -1)" \
    && wasm="$(ls dist/*_bg.wasm | head -1)" \
    && printf 'id=%s\njs=%s\nwasm=%s\nwasm_opt=0\nwasm_bindgen=0.2.127\n' \
         "${PPI_BUILD_ID}" "$(basename "$js")" "$(basename "$wasm")" \
         > dist/ppi-build.txt \
    && sed -i "s#</head>#<!-- ppi-build ${PPI_BUILD_ID} -->\\n</head>#" dist/index.html \
    && grep -q "ppi-build ${PPI_BUILD_ID}" dist/index.html \
    && cat dist/ppi-build.txt \
    && ls -la dist

# ---- go API (embeds Trunk dist) ----
FROM --platform=linux/amd64 golang:1.25-alpine AS builder

ARG PPI_BUILD_ID=stamp-v3-20260810-table-grow

RUN apk add --no-cache ca-certificates git

WORKDIR /src

COPY backend/go.mod backend/go.sum ./
RUN go mod download

COPY backend/ ./
# Replace placeholder static/ with Trunk release dist BEFORE compile (go:embed).
RUN rm -rf ./static && mkdir -p ./static
COPY --from=web-builder /src/web/dist/ ./static/
RUN test -f static/index.html \
    && test -f static/ppi-build.txt \
    && ! grep -q 'data-trunk' static/index.html \
    && grep -q 'import init' static/index.html \
    && grep -q "id=${PPI_BUILD_ID}" static/ppi-build.txt \
    && ls static/*.wasm >/dev/null \
    && echo "embedding SPA build:" \
    && cat static/ppi-build.txt \
    && ls -la static

# Bust Go compile cache when SPA stamp changes (embed content fingerprint).
RUN CGO_ENABLED=0 GOOS=linux GOARCH=amd64 \
    go build -trimpath -ldflags="-s -w -X main.ppiBuildID=${PPI_BUILD_ID}" \
    -o /out/server .

# ---- runtime ----
FROM --platform=linux/amd64 alpine:latest

ARG PPI_BUILD_ID=stamp-v3-20260810-table-grow

RUN apk add --no-cache ca-certificates tzdata \
    && adduser -D -H -u 10001 appuser

WORKDIR /app

COPY --from=builder /src/data ./data
COPY --from=builder /out/server ./server
# Disk copy kept as fallback / DEBUG; primary serve path is go:embed inside server.
COPY --from=web-builder /src/web/dist/ ./static/

RUN test -f /app/static/index.html \
    && test -f /app/static/ppi-build.txt \
    && ! grep -q 'data-trunk' /app/static/index.html \
    && grep -q 'import init' /app/static/index.html \
    && grep -q "id=${PPI_BUILD_ID}" /app/static/ppi-build.txt

ENV DATA_DIR=/app/data \
    STATIC_DIR=/app/static \
    DATABASE_URL=sqlite:///tmp/ppi.db \
    PORT=8080 \
    PPI_BUILD_ID=${PPI_BUILD_ID}

EXPOSE 8080

USER appuser

ENTRYPOINT ["/app/server"]
