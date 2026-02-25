#!/usr/bin/env bash
set -euo pipefail
KERNEL_ELF=${1:-target/x86_64-rustos/debug/kernel}
rust-gdb "$KERNEL_ELF"
