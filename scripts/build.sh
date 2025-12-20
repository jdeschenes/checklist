#!/bin/sh

set -eu

if [ ! -d "backend/.sqlx" ]; then
    echo >&2 ".sqlx is missing."
    echo >&2 "make start-db"
    exit 1
fi

cd backend && SQLX_OFFLINE=true SQLX_OFFLINE_DIR=".sqlx" cargo build --release
cd ..
cd frontend && npm run build
