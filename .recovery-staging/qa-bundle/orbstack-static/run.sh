#!/usr/bin/env bash
set -euo pipefail
mkdir -p ../work/static
docker compose up --abort-on-container-exit
