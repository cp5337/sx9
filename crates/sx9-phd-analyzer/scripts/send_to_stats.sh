#!/usr/bin/env bash
set -euo pipefail
ROOT="${1:-.}"
URL="${2:-http://localhost:18109/ingest}"
SOURCE="${3:-ctas7}"
TAG="${4:-$(date -Iseconds)}"
OUT="target/analyzer.json"
mkdir -p target
./target/release/ctas7-phd-analyzer "$ROOT" --json > "$OUT"
./target/release/post_stats --url "$URL" --source "$SOURCE" --tag "$TAG" "$OUT"
