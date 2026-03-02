use core::sync::atomic::{AtomicU64, Ordering};

static TICKS: AtomicU64 = AtomicU64::new(0);

/// PIT input clock in Hz.
const PIT_INPUT_HZ: u64 = 1_193_182;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PitConfig {
    pub frequency_hz: u32,
    pub divisor: u16,
}

/// Computes the PIT divisor for a requested timer frequency.
pub fn configure_pit(frequency_hz: u32) -> PitConfig {
    let clamped = frequency_hz.max(1);
    let divisor = (PIT_INPUT_HZ / clamped as u64).clamp(1, u16::MAX as u64) as u16;
    PitConfig {
        frequency_hz: clamped,
        divisor,
    }
}

pub fn init(frequency_hz: u32) -> PitConfig {
    TICKS.store(0, Ordering::SeqCst);
    configure_pit(frequency_hz)
}

pub fn handle_timer_interrupt() -> u64 {
    TICKS.fetch_add(1, Ordering::Relaxed) + 1
}

pub fn uptime_ticks() -> u64 {
    TICKS.load(Ordering::Relaxed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pit_divisor_is_computed() {
        let cfg = configure_pit(100);
        assert_eq!(cfg.divisor, 11_931);
    }

    #[test]
    fn timer_ticks_increment() {
        init(100);
        assert_eq!(handle_timer_interrupt(), 1);
        assert_eq!(handle_timer_interrupt(), 2);
        assert_eq!(uptime_ticks(), 2);
    }
}
