#!/usr/bin/env bash
set -euo pipefail
( cd orbstack-static && ./run.sh )
( cd orbstack-semantic && ./run.sh )
( cd orbstack-discovery && ./run.sh )
