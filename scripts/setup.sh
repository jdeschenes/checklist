#!/bin/sh
cargo install sqlx-cli --no-default-features --features rustls,postgres
cargo install cargo-watch

cd frontend/ && npm install
