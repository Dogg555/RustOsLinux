# RustOS 32-bit Kernel

This directory contains a minimal Rust `no_std` kernel for x86 (32-bit protected mode) loaded by GRUB.

## Structure

- `kernel/`: Rust kernel crate (`#![no_std]`, `#![no_main]`)
- `i686-rustos.json`: custom target
- `.cargo/config.toml`: build-std + runner setup
- `src/main.rs`: host-side build runner (`cargo run` helper)
- `../linker/linker.ld`: linker script
- `../iso/boot/grub/grub.cfg`: GRUB menu entry
- `../scripts/run-rustos.sh`: ISO + QEMU launcher

## Build and run

From this `os/` folder:

1. `cargo build -p kernel --target i686-rustos.json`
2. `../scripts/run-rustos.sh target/i686-rustos/debug/kernel`

Or one-step orchestration:

- `cargo run`

`cargo run` performs:
1. Kernel compilation.
2. ISO generation via `grub-mkrescue`.
3. QEMU boot (`qemu-system-i386`).
