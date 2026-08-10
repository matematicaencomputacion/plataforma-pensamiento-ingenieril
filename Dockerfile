# IngenierIA API — multi-stage image for Google Cloud Run.
# Build context: monorepo root. Module path: backend/
#
# NOTE: go.mod requires Go 1.25 — do not use golang:1.22 (build fails).

# ---- build ----
FROM golang:1.25-alpine AS builder

RUN apk add --no-cache ca-certificates git

WORKDIR /src

# Dependency layer (cache-friendly)
COPY backend/go.mod backend/go.sum ./
RUN go mod download

# Source + compile static binary
COPY backend/ ./
RUN CGO_ENABLED=0 GOOS=linux GOARCH=amd64 \
    go build -trimpath -ldflags="-s -w" -o /out/server .

# ---- runtime ----
FROM alpine:latest

RUN apk add --no-cache ca-certificates tzdata \
    && adduser -D -H -u 10001 appuser

WORKDIR /app

# Seed JSON used by levels / cognitive profiles
COPY --from=builder /src/data ./data
COPY --from=builder /out/server ./server

ENV DATA_DIR=/app/data \
    DATABASE_URL=sqlite:///tmp/ppi.db \
    PORT=8080

EXPOSE 8080

USER appuser

ENTRYPOINT ["/app/server"]
