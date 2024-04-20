#!/bin/bash
cargo install wasm-bindgen-cli
rustup target install wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release
wasm-bindgen --no-typescript --target web --out-dir ./out/ ./target/wasm32-unknown-unknown/release/kurvix.wasm
cp index.html out
cp -r assets out
