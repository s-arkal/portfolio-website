#!/bin/bash

set -o nounset
set -o pipefail
set -o errexit

echo "Building styling..."
mkdir -p site/pkg
npm run build:css
cp style/output.css site/pkg/rust-leptos-portfolio.css

echo "Compiling client and server binaries to WebAssembly..."
cargo build --release --bin server --no-default-features --target wasm32-unknown-unknown --features ssr
WEB3FORMS_ACCESS_KEY="${WEB3FORMS_ACCESS_KEY:-MISSING_API_KEY}" cargo build --release --bin client --no-default-features --target wasm32-unknown-unknown --features hydrate

echo "Generating browser and worker bindings..."
wasm-bindgen target/wasm32-unknown-unknown/release/server.wasm --out-name index --no-typescript --target bundler --out-dir site
wasm-bindgen target/wasm32-unknown-unknown/release/client.wasm --out-name index --no-typescript --target web --out-dir site/pkg

echo "Copying Cloudflare Pages configuration and public assets..."
cp _worker.js _routes.json site/
cp -r public/* site/

echo "Build complete."
