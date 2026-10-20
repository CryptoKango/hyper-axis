#!/usr/bin/env bash
set -euo pipefail

repo_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
output_dir="${1:-$repo_dir/dist/linux-x86_64}"

cargo build \
  --manifest-path "$repo_dir/Cargo.toml" \
  --package hyper-axis-ffi \
  --release \
  --target x86_64-unknown-linux-gnu

mkdir -p "$output_dir"
cp "$repo_dir/target/x86_64-unknown-linux-gnu/release/libhyper_axis_ffi.so" "$output_dir/"
cp "$repo_dir/target/x86_64-unknown-linux-gnu/release/libhyper_axis_ffi.a" "$output_dir/"
cp "$repo_dir/README.md" "$output_dir/README.md"
