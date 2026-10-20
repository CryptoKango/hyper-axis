#!/usr/bin/env bash
set -euo pipefail

repo_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
output_dir="${1:-$repo_dir/dist/android}"

command -v cargo-ndk >/dev/null 2>&1 || {
  echo "cargo-ndk is required" >&2
  exit 1
}

mkdir -p "$output_dir"
cd "$repo_dir"
cargo ndk \
  --target arm64-v8a \
  --target armeabi-v7a \
  --target x86_64 \
  --platform 24 \
  --output-dir "$output_dir" \
  build \
  --package hyper-axis-ffi \
  --release

cp "$repo_dir/README.md" "$output_dir/README.md"
