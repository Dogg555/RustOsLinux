# Phase 3 – Memory Management

This milestone adds a first cut memory-management subsystem in the kernel crate.

## Delivered pieces

1. **Physical frame allocator**
   - Parses `BootInfo` memory regions.
   - Skips non-usable regions.
   - Allocates 4KiB frames sequentially.

2. **Paging scaffolding (x86_64 style)**
   - Adds PML4/PDPT/PD tables.
   - Supports higher-half kernel mapping helper.
   - Supports identity-map helper used for early bootstrap.
   - `enable_paging` is intentionally isolated until CPU bring-up phase wires CR3/CR0.

3. **Heap allocator**
   - Adds a `#[global_allocator]` bump allocator.
   - Statically reserved heap region (1 MiB).
   - Allocation error telemetry recorded for diagnostics.

4. **Page fault handling**
   - Adds fault recording API (fault address + error code).
   - Adds hard-stop handler to prevent silent crashes.

## Validation done

- Host-target check for kernel crate (`x86_64-unknown-linux-gnu`).
- Bootloader crate unit tests.
