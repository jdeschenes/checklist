#!/bin/sh
cargo install sqlx-cli --no-default-features --features rustls,postgres

cd frontend/ && npm install
