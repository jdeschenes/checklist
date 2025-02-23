#!/bin/sh

cd frontend
npm run dev &
cd ../backend && cargo run
