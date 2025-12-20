#!/bin/sh

set -eu

if [ -z "${DATABASE_URL:-}" ]; then
    echo >&2 "DATABASE_URL is not set."
    echo >&2 "Example: postgres://app:secret@localhost:5432/newdbname?sslmode=disable"
    exit 1
fi

if ! command -v cargo >/dev/null 2>&1; then
    echo >&2 "cargo is not installed."
    exit 1
fi

if ! command -v cargo-sqlx >/dev/null 2>&1; then
    echo >&2 "sqlx-cli is not installed."
    echo >&2 "Install with:"
    echo >&2 "    cargo install sqlx-cli --no-default-features --features rustls,postgres"
    exit 1
fi

cd backend
SQLX_OFFLINE=false SQLX_OFFLINE_DIR=".sqlx" cargo sqlx prepare -- --bin checklist
