use core::sync::atomic::{AtomicU64, Ordering};

static LAST_FAULT_ADDR: AtomicU64 = AtomicU64::new(0);
static LAST_ERROR_CODE: AtomicU64 = AtomicU64::new(0);

pub fn init() {
    LAST_FAULT_ADDR.store(0, Ordering::SeqCst);
    LAST_ERROR_CODE.store(0, Ordering::SeqCst);
}

pub fn record_fault(addr: u64, error_code: u64) {
    LAST_FAULT_ADDR.store(addr, Ordering::Relaxed);
    LAST_ERROR_CODE.store(error_code, Ordering::Relaxed);
}

pub fn last_fault() -> (u64, u64) {
    (
        LAST_FAULT_ADDR.load(Ordering::Relaxed),
        LAST_ERROR_CODE.load(Ordering::Relaxed),
    )
}

pub fn handle_page_fault(fault_addr: u64, error_code: u64) -> ! {
    record_fault(fault_addr, error_code);

    // Prevent silent crash by entering a known halted loop.
    loop {
        core::hint::spin_loop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn records_fault_context() {
        init();
        record_fault(0xdead_beef, 0b101);
        assert_eq!(last_fault(), (0xdead_beef, 0b101));
    }
}
