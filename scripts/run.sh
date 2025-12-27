#!/bin/sh
set -e

frontend_pid=""

cleanup() {
  if [ -n "$frontend_pid" ] && kill -0 "$frontend_pid" 2>/dev/null; then
    kill "$frontend_pid" 2>/dev/null
    wait "$frontend_pid" 2>/dev/null || true
  fi
}

trap 'cleanup; exit 130' INT TERM
trap 'cleanup' EXIT

( cd frontend && npm run dev ) &
frontend_pid=$!
cd backend
cargo watch -x 'run'
