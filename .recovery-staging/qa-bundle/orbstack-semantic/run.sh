#!/usr/bin/env bash
set -euo pipefail
mkdir -p ../work/semantic
docker compose up --abort-on-container-exit
