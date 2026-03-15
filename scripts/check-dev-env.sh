#!/usr/bin/env bash
set -euo pipefail

missing=0

check_cmd() {
  local c="$1"
  if command -v "$c" >/dev/null 2>&1; then
    echo "[ok] $c -> $(command -v "$c")"
  else
    echo "[missing] $c"
    missing=1
  fi
}

echo "RustOsLinux developer environment check"
check_cmd python3
check_cmd pip3
check_cmd rustup
check_cmd cargo
check_cmd qemu-system-x86_64
check_cmd gdb

if command -v rustup >/dev/null 2>&1; then
  rustup toolchain list | rg -q '^nightly' && echo "[ok] nightly toolchain installed" || {
    echo "[missing] nightly toolchain"
    missing=1
  }
fi

if [[ $missing -eq 1 ]]; then
  echo "Environment is missing required tools. Run: scripts/bootstrap-dev-env.sh -y"
  exit 1
fi

echo "Environment looks good."
