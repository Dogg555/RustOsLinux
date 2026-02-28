#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
KERNEL="${1:-$ROOT/os/target/i686-rustos/debug/kernel}"
ISO_DIR="$ROOT/iso"
ISO_OUT="$ROOT/os/target/rustos.iso"

mkdir -p "$ISO_DIR/boot"
cp "$KERNEL" "$ISO_DIR/boot/kernel.elf"
grub-mkrescue -o "$ISO_OUT" "$ISO_DIR"
qemu-system-i386 -cdrom "$ISO_OUT" -serial stdio
