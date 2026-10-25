#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
variant="${1:-coherent}"
if [[ "$variant" != coherent && "$variant" != split ]]; then
  echo 'Usage: scripts/build-v2.sh coherent|split [isolated-bundle-directory]' >&2
  exit 2
fi
features=hydrate
if [[ "$variant" == split ]]; then features=hydrate,split-islands; fi
assets=apps/web/public
bundle="${2:-}"
if [[ -n "$bundle" ]]; then
  mkdir -p "$bundle/assets"
  cp -R apps/web/public/. "$bundle/assets/"
  assets="$bundle/assets"
fi
cargo build --locked -p learningos-web --target wasm32-unknown-unknown --profile wasm-release --no-default-features --features "$features"
wasm-bindgen --remove-name-section --remove-producers-section --target web --out-dir "$assets/pkg" target/wasm32-unknown-unknown/wasm-release/learningos_web.wasm
if [[ "$variant" == split ]]; then
  cargo build --locked -p learningos-server --release --features learningos-web/split-islands
else
  cargo build --locked -p learningos-server --release
fi
cache_version="$( { find "$assets" -type f ! -name '*.gz' ! -name sw.js ! -name '*.d.ts' -print0 | sort -z | xargs -0 sha256sum | cut -d' ' -f1; sha256sum target/release/learningos-server apps/web/service-worker.template.js | cut -d' ' -f1; } | sha256sum | cut -c1-20)"
sed "s/@CACHE_VERSION@/$cache_version/" apps/web/service-worker.template.js > "$assets/sw.js"
find "$assets" -type f \( -name '*.wasm' -o -name '*.js' -o -name '*.css' -o -name '*.svg' -o -name '*.webmanifest' -o -name '*.ttf' \) -exec gzip -n -9 -k -f {} +
if [[ -n "$bundle" ]]; then
  cp target/release/learningos-server "$bundle/learningos-server"
  {
    echo "variant=$variant"
    echo "base_revision=$(git rev-parse HEAD)"
    echo "cache_version=$cache_version"
    sha256sum Cargo.lock "$assets/pkg/learningos_web_bg.wasm" "$assets/pkg/learningos_web.js" "$assets/app.css" "$bundle/learningos-server"
  } > "$bundle/build-manifest.txt"
fi
