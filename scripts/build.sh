#!/usr/bin/env bash
set -euo pipefail

cargo +nightly build \
  -p kernel \
  --target x86_64-unknown-none \
  --offline
