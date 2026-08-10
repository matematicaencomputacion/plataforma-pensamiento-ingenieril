# IngenierIA — Cloud Run image: Go API + Trunk/Leptos static SPA.
# Build context: monorepo root.
#
# NOTE: go.mod requires Go 1.25 — do not use golang:1.22 (build fails).

# ---- web (Wasm / Trunk) ----
FROM rust:1-bookworm AS web-builder

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

RUN env -u NO_COLOR trunk build --release

# ---- go API ----
FROM golang:1.25-alpine AS builder

RUN apk add --no-cache ca-certificates git

WORKDIR /src

COPY backend/go.mod backend/go.sum ./
RUN go mod download

COPY backend/ ./
RUN CGO_ENABLED=0 GOOS=linux GOARCH=amd64 \
    go build -trimpath -ldflags="-s -w" -o /out/server .

# ---- runtime ----
FROM alpine:latest

RUN apk add --no-cache ca-certificates tzdata \
    && adduser -D -H -u 10001 appuser

WORKDIR /app

COPY --from=builder /src/data ./data
COPY --from=builder /out/server ./server
COPY --from=web-builder /src/web/dist ./static

ENV DATA_DIR=/app/data \
    STATIC_DIR=/app/static \
    DATABASE_URL=sqlite:///tmp/ppi.db \
    PORT=8080

EXPOSE 8080

USER appuser

ENTRYPOINT ["/app/server"]
