# RustOsLinux

> **Memory safety. Modular power. Built with Rust.**

RustOsLinux is an AI-assisted, Rust-native microkernel OS project focused on modular design, VM-first development, and long-term maintainability.

---

## 📌 Project Definition

- **Name:** RustOsLinux
- **Type:** Rust-based Microkernel Operating System
- **Architecture:** x86_64 (initially)
- **Boot Method:** UEFI
- **Primary Dev Environment:** QEMU
- **Language:** Rust (`#![no_std]`, nightly)
- **Design Model:** Microkernel + Userspace Servers
- **Development Strategy:** Modular AI-assisted generation

---

## 🧠 Architecture Decision

### Kernel Responsibilities

- Scheduler
- Memory management
- Interrupt handling
- IPC
- Syscall interface

### Userspace Responsibilities

- Filesystem
- Drivers
- Networking
- Window system
- Shell
- Applications

---

## 🚀 Full Development Execution Plan

### Phase 0 — Environment Setup

1. **Install toolchain**
   - Rust nightly
   - `rust-src`
   - LLVM tools
   - QEMU
   - GDB
2. **Create workspace**
   - Cargo workspace setup
   - Root `Cargo.toml`
   - Custom target JSON (`rustoslinux.json`)
   - `.cargo/config.toml`
   - Panic strategy `abort`
3. **Setup build system**
   - `Makefile`
   - Build script
   - QEMU launch script
   - Debug script

### Phase 1 — Bootloader (Goal: boot into Rust kernel)

1. **UEFI entry**
   - Create `bootloader` crate
   - Implement UEFI main entry
   - Disable `std`
   - Configure linker script
2. **Framebuffer setup**
   - Detect GOP
   - Initialize framebuffer
   - Basic pixel drawing
   - Simple text renderer
3. **Memory map extraction**
   - Retrieve UEFI memory map
   - Store physical memory regions
   - Pass map to kernel
4. **Kernel loading**
   - Load kernel ELF from disk
   - Parse ELF headers
   - Map segments
   - Jump to kernel entry

### Phase 2 — Kernel Initialization (Goal: kernel runs and prints)

1. **Kernel entry**
   - Implement `_start`
   - Setup stack
   - Clear BSS
   - Initialize logger
2. **GDT setup**
   - Create/load GDT
   - Setup kernel code/data segments
3. **IDT setup**
   - Create/load IDT
   - Install exception handlers
4. **Interrupt testing**
   - Trigger breakpoint interrupt
   - Verify handler execution

### Phase 3 — Memory Management (Goal: safe memory allocation)

1. **Physical frame allocator**
   - Parse boot memory map
   - Identify usable regions
   - Implement frame allocation
2. **Paging**
   - Setup PML4
   - Map higher-half kernel
   - Identity-map required memory
   - Enable paging
3. **Heap allocator**
   - Define heap region
   - Implement global allocator
   - Verify `alloc`
4. **Page fault handler**
   - Log fault address
   - Prevent silent crashes

### Phase 4 — Timer & Scheduler (Goal: run multiple tasks)

1. **Timer interrupt**
   - Configure PIT or APIC timer
   - Register handler
   - Increment system tick
2. **Task structure**
   - Define task struct
   - Track stack pointer
   - Track register state
3. **Context switching**
   - Save registers
   - Switch stack
   - Restore registers
4. **Round-robin scheduler**
   - Run queue
   - Tick-based task rotation
   - Validate multitasking

### Phase 5 — Process Model (Goal: userspace processes exist)

1. **Process structure**
   - PID
   - Address space
   - Capability table
   - State enum
2. **ELF loader**
   - Parse ELF binaries
   - Map sections
   - Setup userspace stack
   - Set entry point
3. **Ring transition**
   - Setup user segments
   - Configure syscall entry
   - Enter ring 3 safely

### Phase 6 — IPC System (Goal: message-based communication)

1. **Message format**
   - Header
   - Payload
   - Sender ID
   - Capability references
2. **Channels**
   - Channel abstraction
   - Send/receive
   - Blocking receive when empty
3. **Capability model**
   - Define capability tokens
   - Validate access rights
   - Enforce isolation

### Phase 7 — System Servers (Goal: move functionality out of kernel)

1. **Process server**
   - Lifecycle management
   - Program spawning
   - Exit handling
2. **Filesystem server**
   - RAMFS implementation
   - IPC file API
3. **Driver server**
   - Userspace drivers
   - IPC communication with kernel

### Phase 8 — Filesystem

- **Stage 1: RAMFS**
  - In-memory FS
  - Directory support
  - Read/write support
- **Stage 2: FAT support**
  - Parse FAT structures
  - Mount disk image
  - Read files
- **Stage 3: RustFS**
  - Rust-native filesystem design
  - Journaling
  - Capability-based access

### Phase 9 — Network Stack

1. **Ethernet layer**
   - Packet parsing
   - MAC handling
2. **IP layer**
   - IPv4 parsing
   - Routing table
3. **TCP/UDP layer**
   - Basic TCP handshake
   - UDP packet handling

### Phase 10 — UI System

1. **Framebuffer abstraction**
   - Drawing primitives
   - Double buffering
2. **Compositor**
   - Window surfaces
   - Z-order handling
3. **Window manager**
   - Input routing
   - Window lifecycle
4. **Shell**
   - Command interpreter
   - Process spawning

### Phase 11 — Security Model

- Capability-based permissions
- Process isolation
- Memory region enforcement
- Secure IPC validation

### Phase 12 — Debugging & Stability

- Kernel logging
- Serial debugging
- Panic handler improvements
- Memory leak detection
- Deadlock detection

---

## 🎯 Initial Success Criteria

RustOsLinux should be able to:

- Boot in QEMU
- Print to framebuffer
- Handle timer interrupts
- Run multiple scheduled tasks
- Spawn a userspace server
- Communicate via IPC

---

## 🧭 Long-Term Objective

RustOsLinux evolves into a:

- Fully modular
- VM-compatible
- Memory-safe
- Capability-secured
- Graphical
- Network-enabled
- Fully Rust-native OS

---

## 🤖 AI Usage Guidelines

For AI-assisted development (Codex and related tools):

- Generate one subsystem at a time
- Keep modules isolated
- Define clear interfaces up front
- Avoid cross-module coupling
- Write testable units
- Document every kernel interface

---

## ⚠️ Known Hard Problems

- Memory management bugs
- Page table misconfiguration
- Triple faults
- Context switching correctness
- Hardware driver complexity
- SMP race conditions
- Kernel debugging visibility
