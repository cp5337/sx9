#!/usr/bin/env bash
set -euo pipefail
mkdir -p ../work/discovery ../work/glaf ../work/qdrant
docker compose up --abort-on-container-exit
