#!/usr/bin/env bash
set -euo pipefail
KERNEL_BIN=${1:-target/x86_64-rustos/debug/kernel}
qemu-system-x86_64 \
  -machine q35 \
  -cpu qemu64 \
  -m 256M \
  -serial stdio \
  -display none \
  -kernel "$KERNEL_BIN"
