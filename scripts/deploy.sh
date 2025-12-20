#!/bin/sh

set -eu

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"

if [ "${SKIP_BUILD:-}" != "true" ]; then
    if [ ! -d "$ROOT_DIR/backend/.sqlx" ]; then
        echo >&2 ".sqlx is missing."
        echo >&2 "Run: make start-db"
        exit 1
    fi

    if ! command -v cargo >/dev/null 2>&1; then
        echo >&2 "cargo is not installed."
        exit 1
    fi

    BUILD_TARGET="${BUILD_TARGET:-}"

    if [ -n "$BUILD_TARGET" ]; then
        (cd "$ROOT_DIR/backend" && SQLX_OFFLINE=true SQLX_OFFLINE_DIR=".sqlx" cargo build --release --target "$BUILD_TARGET")
        DEFAULT_BINARY="$ROOT_DIR/backend/target/$BUILD_TARGET/release/checklist"
    else
        (cd "$ROOT_DIR/backend" && SQLX_OFFLINE=true SQLX_OFFLINE_DIR=".sqlx" cargo build --release)
        DEFAULT_BINARY="$ROOT_DIR/backend/target/release/checklist"
    fi
else
    DEFAULT_BINARY=""
fi

BINARY_PATH="${BINARY_PATH:-$DEFAULT_BINARY}"

if [ -z "${BINARY_PATH:-}" ]; then
    echo >&2 "BINARY_PATH is required when SKIP_BUILD=true."
    exit 1
fi

case "$BINARY_PATH" in
    /*) ;;
    *) BINARY_PATH="$ROOT_DIR/$BINARY_PATH" ;;
esac

if [ ! -f "$BINARY_PATH" ]; then
    echo >&2 "Binary not found: $BINARY_PATH"
    exit 1
fi

if [ "${SKIP_FRONTEND_BUILD:-}" != "true" ]; then
    if ! command -v npm >/dev/null 2>&1; then
        echo >&2 "npm is not installed."
        exit 1
    fi

    FRONTEND_API_BASE_URL="${FRONTEND_API_BASE_URL:-https://checklist.deschenes.me}"
    (cd "$ROOT_DIR/frontend" && VITE_API_BASE_URL="$FRONTEND_API_BASE_URL" npm run build)
    FRONTEND_DIST_PATH="$ROOT_DIR/frontend/dist"
else
    FRONTEND_DIST_PATH="${FRONTEND_DIST_PATH:-}"
fi

if [ -z "${FRONTEND_DIST_PATH:-}" ]; then
    echo >&2 "FRONTEND_DIST_PATH is required when SKIP_FRONTEND_BUILD=true."
    exit 1
fi

case "$FRONTEND_DIST_PATH" in
    /*) ;;
    *) FRONTEND_DIST_PATH="$ROOT_DIR/$FRONTEND_DIST_PATH" ;;
esac

if [ ! -d "$FRONTEND_DIST_PATH" ]; then
    echo >&2 "Frontend dist not found: $FRONTEND_DIST_PATH"
    exit 1
fi

if ! command -v ansible-playbook >/dev/null 2>&1; then
    echo >&2 "ansible-playbook is not installed."
    exit 1
fi

cd "$ROOT_DIR/infra"
VAULT_ARGS=""
if [ "${ASK_VAULT_PASS:-}" = "true" ]; then
    VAULT_ARGS="--ask-vault-pass"
elif [ -n "${VAULT_PASSWORD_FILE:-}" ]; then
    VAULT_ARGS="--vault-password-file $VAULT_PASSWORD_FILE"
elif [ -z "${ANSIBLE_VAULT_PASSWORD_FILE:-}" ] && [ -f "$ROOT_DIR/infra/group_vars/all/vault.yml" ]; then
    VAULT_ARGS="--ask-vault-pass"
fi

ansible-playbook deploy.yml --extra-vars "binary_path=$BINARY_PATH frontend_dist_path=$FRONTEND_DIST_PATH" $VAULT_ARGS "$@"
