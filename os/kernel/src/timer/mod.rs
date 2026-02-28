use core::sync::atomic::{AtomicU64, Ordering};
use x86::io::outb;

static TICKS: AtomicU64 = AtomicU64::new(0);

pub fn init(hz: u32) {
    let divisor: u16 = (1193180 / hz) as u16;
    unsafe {
        outb(0x43, 0x36);
        outb(0x40, (divisor & 0xFF) as u8);
        outb(0x40, ((divisor >> 8) & 0xFF) as u8);
    }
}

pub fn tick() {
    TICKS.fetch_add(1, Ordering::Relaxed);
}

pub fn uptime_ticks() -> u64 {
    TICKS.load(Ordering::Relaxed)
}
