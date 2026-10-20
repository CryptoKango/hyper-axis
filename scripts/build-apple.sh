#!/usr/bin/env bash
set -euo pipefail

repo_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
output_dir="${1:-$repo_dir/dist/apple}"

if [[ "$(uname -s)" != "Darwin" ]]; then
  echo "Apple artifacts must be built on macOS" >&2
  exit 1
fi

rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios
for target in aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios; do
  cargo build \
    --manifest-path "$repo_dir/Cargo.toml" \
    --package hyper-axis-ffi \
    --release \
    --target "$target"
done

mkdir -p "$output_dir/device" "$output_dir/simulator"
cp "$repo_dir/target/aarch64-apple-ios/release/libhyper_axis_ffi.a" \
  "$output_dir/device/libhyper_axis_ffi.a"
lipo -create \
  "$repo_dir/target/aarch64-apple-ios-sim/release/libhyper_axis_ffi.a" \
  "$repo_dir/target/x86_64-apple-ios/release/libhyper_axis_ffi.a" \
  -output "$output_dir/simulator/libhyper_axis_ffi.a"
cp "$repo_dir/README.md" "$output_dir/README.md"
