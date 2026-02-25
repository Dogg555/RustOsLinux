#!/usr/bin/env bash
set -euo pipefail

cargo +nightly build \
  -p bootloader \
  -p kernel \
  -Z json-target-spec \
  -Z build-std=core,alloc,compiler_builtins \
  -Z build-std-features=compiler-builtins-mem
