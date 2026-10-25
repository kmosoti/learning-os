#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
command -v cargo
command -v wasm-bindgen
command -v node
cargo +1.99.0 build --manifest-path experiments/storage/Cargo.toml --locked --release -j 2
RUSTFLAGS='--cfg=web_sys_unstable_apis' cargo +1.99.0 build --manifest-path experiments/storage/browser-rust/Cargo.toml --locked --target wasm32-unknown-unknown --release -j 2
wasm-bindgen --target web --out-dir experiments/storage/browser/pkg experiments/storage/browser-rust/target/wasm32-unknown-unknown/release/learningos_browser_journal_experiment.wasm
npm --prefix experiments/storage/browser ci
npm --prefix experiments/storage/browser exec -- playwright install firefox
# Intentionally sequential: disk-intensive comparisons must not contend with each other.
python experiments/storage/run_native.py
python experiments/storage/concurrency.py
node experiments/storage/browser/run.mjs
node experiments/storage/browser/lifecycle.mjs
python experiments/storage/pack_results.py
