#!/bin/sh

cd backend && cargo build --release
cd ..
cd frontend && npm run build
