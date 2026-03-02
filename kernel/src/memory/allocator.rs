use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use core::sync::atomic::{AtomicUsize, Ordering};

const HEAP_SIZE: usize = 1024 * 1024;

#[repr(align(4096))]
struct AlignedHeap([u8; HEAP_SIZE]);

static mut HEAP: AlignedHeap = AlignedHeap([0; HEAP_SIZE]);
static NEXT: AtomicUsize = AtomicUsize::new(0);
static LAST_ALLOC_ERROR_SIZE: AtomicUsize = AtomicUsize::new(0);
static LAST_ALLOC_ERROR_ALIGN: AtomicUsize = AtomicUsize::new(0);

pub struct BumpAllocator;

#[cfg(not(test))]
#[global_allocator]
static GLOBAL_ALLOCATOR: BumpAllocator = BumpAllocator;

#[cfg(test)]
static GLOBAL_ALLOCATOR: BumpAllocator = BumpAllocator;

pub fn init() {
    NEXT.store(0, Ordering::SeqCst);
}

pub fn record_alloc_error(size: usize, align: usize) {
    LAST_ALLOC_ERROR_SIZE.store(size, Ordering::Relaxed);
    LAST_ALLOC_ERROR_ALIGN.store(align, Ordering::Relaxed);
}

pub fn last_alloc_error() -> (usize, usize) {
    (
        LAST_ALLOC_ERROR_SIZE.load(Ordering::Relaxed),
        LAST_ALLOC_ERROR_ALIGN.load(Ordering::Relaxed),
    )
}

// Safety: this allocator is single-address-space early-boot only and monotonic;
// deallocation is intentionally a no-op.
unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let align_mask = layout.align().saturating_sub(1);

        let mut current = NEXT.load(Ordering::Relaxed);
        loop {
            let aligned = (current + align_mask) & !align_mask;
            let next = match aligned.checked_add(layout.size()) {
                Some(v) => v,
                None => return null_mut(),
            };

            if next > HEAP_SIZE {
                return null_mut();
            }

            match NEXT.compare_exchange_weak(current, next, Ordering::SeqCst, Ordering::Relaxed) {
                Ok(_) => {
                    let base = core::ptr::addr_of_mut!(HEAP.0) as *mut u8;
                    return base.add(aligned);
                }
                Err(observed) => current = observed,
            }
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bump_allocator_produces_aligned_non_null_allocations() {
        init();
        let layout = Layout::from_size_align(64, 32).unwrap();
        let ptr = unsafe { GLOBAL_ALLOCATOR.alloc(layout) };
        assert!(!ptr.is_null());
        assert_eq!((ptr as usize) % 32, 0);
    }
}
