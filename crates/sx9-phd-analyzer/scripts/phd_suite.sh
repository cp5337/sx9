#!/usr/bin/env bash
set -euo pipefail
root="${1:-.}"
out="${2:-target/phd_suite_report.txt}"
echo "[PHD] Clippy" > "$out"; cargo clippy -- -D warnings >> "$out" || true
echo >> "$out"
echo "[PHD] Geiger (unsafe scan)" >> "$out"; cargo geiger -q >> "$out" || true
echo >> "$out"
echo "[PHD] Audit (RustSec)" >> "$out"; cargo audit >> "$out" || true
echo >> "$out"
if command -v cargo-llvm-cov >/dev/null; then
  echo "[PHD] Coverage (llvm-cov)" >> "$out"
  cargo llvm-cov --no-report --tests >> "$out" || true
elif command -v cargo-tarpaulin >/dev/null; then
  echo "[PHD] Coverage (tarpaulin)" >> "$out"
  cargo tarpaulin -o Stdout >> "$out" || true
fi
if command -v tokei >/dev/null; then
  echo >> "$out"; echo "[PHD] LOC (tokei)" >> "$out"; tokei "$root" >> "$out" || true
fi
echo "Saved: $out"
