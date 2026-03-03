# Phase 4 – Timer & Scheduler

This milestone introduces a minimal timer path and a round-robin scheduler model to run multiple kernel tasks.

## Step 1 – Timer Interrupt

- Added `kernel/src/timer.rs`.
- Added PIT divisor computation helper (`configure_pit`).
- Added interrupt-side tick increment (`handle_timer_interrupt`).
- Added global uptime tick query (`uptime_ticks`).

## Step 2 – Task Structure

- Added `Task` with:
  - task id
  - stack pointer
  - full register snapshot (`RegisterState`) containing general registers, `rip`, and `rflags`.

## Step 3 – Context Switching

- Added context transition model in scheduler:
  - `save_current_registers` to persist interrupted task CPU state.
  - `load_next_registers` to restore selected task state.
  - `ContextSwitch` record returned by scheduler on tick rotation.

## Step 4 – Round-Robin Scheduler

- Added `RoundRobinScheduler` with a fixed-size run queue.
- `on_timer_tick` rotates tasks in strict round-robin order.
- Unit tests validate:
  - task rotation order
  - register save/restore behavior.

## Validation done

- `cargo fmt`
- `cargo +nightly check -p kernel --target x86_64-unknown-linux-gnu`
- `cargo +nightly test -p kernel --target x86_64-unknown-linux-gnu`
