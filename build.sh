#!/bin/sh

export EMMAKEN_CFLAGS="-s USE_SDL=2"
cargo build --release --target wasm32-unknown-emscripten
mkdir -p wasm
mv target/wasm32-unknown-emscripten/release/deps/*.js wasm/wasm-test.js
mv target/wasm32-unknown-emscripten/release/deps/*.wasm wasm/wasm-test.wasm