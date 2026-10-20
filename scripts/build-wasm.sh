#!/usr/bin/env bash
set -euo pipefail

repo_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
output_dir="${1:-$repo_dir/dist/wasm}"

command -v wasm-bindgen >/dev/null 2>&1 || {
  echo "wasm-bindgen CLI is required (expected version 0.2.129)" >&2
  exit 1
}

rustup target add wasm32-unknown-unknown
cargo build \
  --manifest-path "$repo_dir/Cargo.toml" \
  --package hyper-axis-wasm \
  --release \
  --target wasm32-unknown-unknown

mkdir -p "$output_dir"
wasm-bindgen \
  "$repo_dir/target/wasm32-unknown-unknown/release/hyper_axis_wasm.wasm" \
  --target web \
  --typescript \
  --out-dir "$output_dir" \
  --out-name hyper_axis

cp "$repo_dir/README.md" "$output_dir/README.md"
