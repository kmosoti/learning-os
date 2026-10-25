#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
cargo fmt --all --check
cargo clippy --locked --workspace --all-targets -- -D warnings
cargo clippy --locked -p learningos-web --target wasm32-unknown-unknown --no-default-features --features hydrate -- -D warnings
cargo test --locked --workspace
mkdir -p artifacts/v2
cargo run --locked -q -p learningos-domain --example replay_fixture > artifacts/v2/golden-replay.json
scripts/build-v2.sh coherent
npm ci --ignore-scripts --prefix tests/browser
npm test --prefix tests/browser
