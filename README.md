# RustOsLinux

RustOsLinux is a Rust-first microkernel project targeting x86_64 with a strict kernel/userspace split, VM-first validation, and milestone-driven delivery.

## Project Definition

- **Kernel type:** Microkernel
- **Primary language:** Rust nightly (`#![no_std]` kernel path)
- **Boot path:** UEFI bootloader handing off structured `BootInfo`
- **Primary target:** `x86_64-rustos` custom target
- **Primary execution environment:** QEMU first, bare metal later
- **Development style:** phase-based implementation with CI-checkable milestones

## Architecture Decision: Kernel vs Userspace Boundaries

### Kernel Owns
- Low-level CPU initialization (GDT/IDT, interrupt gates, timer setup)
- Virtual memory and frame allocation primitives
- Scheduler and context switching
- Syscall dispatch
- Capability checks and IPC transport

### Userspace Owns
- Process manager policy and service orchestration
- Filesystem services (RAMFS → FAT → RustFS)
- Device/driver servers wherever feasible
- Networking stack services
- Shell/window manager/compositor/application runtime

This boundary is intentional: keep policy in userspace, mechanism in kernel.

## Repository Layout (Scaffold)

- `bootloader/` — UEFI handoff crate and boot contract (`BootInfo`)
- `kernel/` — no_std kernel entry and core initialization stages
- `libs/ipc/` — shared IPC message schema
- `libs/syscall/` — syscall numbers and ABI constants
- `userspace/init/` — initial userspace process manager placeholder
- `userspace/fs-server/` — filesystem server placeholder
- `userspace/net-server/` — networking server placeholder
- `targets/x86_64-rustos.json` — custom Rust target
- `.cargo/config.toml` — target + build-std configuration
- `scripts/` — build/run/debug helper scripts
- `config/kernel.ld` — initial linker script
- `docs/ROADMAP.md` — execution and delivery checklist anchor

## Environment & Toolchain Setup

1. Install prerequisites:
   - Rustup + nightly toolchain
   - `rust-src`, `llvm-tools-preview`, `rustfmt`, `clippy`
   - QEMU (`qemu-system-x86_64`)
   - GDB (`rust-gdb` preferred)
2. Use bundled toolchain config:
   - `rust-toolchain.toml` pins nightly + components.
3. Verify workspace resolves:
   - `cargo metadata --no-deps`
4. Build early scaffold:
   - `scripts/build.sh`
5. Run kernel stub in VM:
   - `scripts/run-qemu.sh target/x86_64-rustos/debug/kernel`

## Bootloader and Kernel Boot Sub-Steps

1. **UEFI entry**
   - Initialize UEFI services and console fallback.
2. **Framebuffer setup**
   - Locate GOP mode, map framebuffer, pass to kernel via `BootInfo`.
3. **Memory map handoff**
   - Capture UEFI memory descriptors, normalize regions, pass pointer + length.
4. **ELF load + jump**
   - Parse kernel ELF, map loadable segments, switch to kernel entry.
5. **Kernel early init**
   - Validate `BootInfo`, bring up IDT/GDT, initialize allocator primitives.

## Execution Plan (Phase 0–12)

### Phase 0 — Project Bootstrap
- Toolchain, target JSON, cargo config, scripts, workspace skeleton.
- **Exit criteria:** reproducible build environment and working scaffold scripts.

### Phase 1 — Bootloader Path
- UEFI app entry, framebuffer capture, memory map extraction, ELF kernel load.
- **Exit criteria:** kernel entry reached from bootloader under QEMU.

### Phase 2 — Kernel Bring-up
- `_start`, panic path, serial/framebuffer logging, GDT/IDT + exception handlers.
- **Exit criteria:** deterministic early logs and exception smoke checks.

### Phase 3 — Memory Management
- Physical frame allocator, paging policy, heap allocator, page-fault diagnostics.
- **Exit criteria:** dynamic memory and stable page fault reporting.

### Phase 4 — Timer + Scheduler
- Timer IRQ, task model, context switch, round-robin baseline.
- **Exit criteria:** multiple runnable tasks rotate on timer ticks.

### Phase 5 — Process Model
- PID/address space structures, userspace ELF loader, ring transition.
- **Exit criteria:** first userspace process executes with controlled syscalls.

### Phase 6 — IPC
- Message schema, channels, blocking receive, capability-gated send/recv.
- **Exit criteria:** reliable request/response between processes.

### Phase 7 — Core Servers
- Process manager, FS server baseline, driver-server communication model.
- **Exit criteria:** userspace services launched and discoverable.

### Phase 8 — Filesystem
- RAMFS first, FAT read support next, RustFS design after stability.
- **Exit criteria:** file open/read/write flow through FS server IPC.

### Phase 9 — Networking
- Ethernet framing, IPv4 basics, UDP/TCP minimum viability.
- **Exit criteria:** packet send/receive in VM networking mode.

### Phase 10 — UI Stack
- Framebuffer abstraction, compositor, window manager, shell UX path.
- **Exit criteria:** basic windowed output + input routing.

### Phase 11 — Security Model
- Capability enforcement, memory isolation hardening, syscall validation.
- **Exit criteria:** unauthorized access attempts are denied and logged.

### Phase 12 — Debugging + Stability
- Structured logging, serial diagnostics, leak/deadlock detection hooks.
- **Exit criteria:** regression suite + debugging workflow for core subsystems.

## Initial Success Criteria (Beyond “It Boots”)

RustOsLinux Phase-complete baseline requires:
- Boot to kernel in QEMU
- Framebuffer or serial output from kernel initialization
- Timer interrupts firing predictably
- Multi-task scheduling observed
- Spawn of at least one userspace server (`init` + one service)
- Working IPC exchange between two processes

## AI Usage Guidelines

When using AI-assisted generation:
- Generate one subsystem per change.
- Always include explicit interface contracts first.
- Prefer small, reviewable commits over broad speculative codegen.
- Require phase-linked acceptance criteria in PR descriptions.
- Reject generated code that mixes policy into kernel mechanism layers.

## Known Hard Problems

- Page-table correctness and higher-half mapping faults
- Interrupt race conditions and lock ordering bugs
- Context-switch ABI breakage
- Capability lifecycle correctness (revocation, duplication)
- Userspace driver isolation/performance tradeoffs
- Multi-core scheduler correctness and fairness under contention
- Kernel observability during early boot and triple-fault scenarios

## Long-Term Objective

Deliver a memory-safe, capability-secured, modular Rust microkernel OS with practical userspace services, VM-first developer workflow, and a path to real hardware support.
