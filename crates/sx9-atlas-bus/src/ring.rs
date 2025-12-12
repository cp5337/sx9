//! Core ring buffer implementation
//!
//! Single-producer, single-consumer (SPSC) lock-free ring buffer.
//! Cache-line aligned to prevent false sharing between producer and consumer.

use core::cell::UnsafeCell;
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicUsize, Ordering};

/// Cache line size (64 bytes on x86_64, ARM64)
const CACHE_LINE: usize = 64;

/// Lock-free SPSC ring buffer
///
/// # Memory Layout
/// ```text
/// ┌────────────────────────────────────────────────────────────────┐
/// │ head (producer index) │ padding to 64 bytes                    │
/// ├────────────────────────────────────────────────────────────────┤
/// │ tail (consumer index) │ padding to 64 bytes                    │
/// ├────────────────────────────────────────────────────────────────┤
/// │ buffer[0] │ buffer[1] │ ... │ buffer[N-1]                      │
/// └────────────────────────────────────────────────────────────────┘
/// ```
#[repr(C)]
pub struct Ring<T, const N: usize> {
    // Producer cache line
    head: CacheAligned<AtomicUsize>,

    // Consumer cache line
    tail: CacheAligned<AtomicUsize>,

    // Data buffer
    buffer: UnsafeCell<[MaybeUninit<T>; N]>,
}

/// Cache-line aligned wrapper
#[repr(C, align(64))]
struct CacheAligned<T> {
    value: T,
    _pad: [u8; CACHE_LINE - core::mem::size_of::<AtomicUsize>()],
}

impl<T> CacheAligned<T> {
    const fn new(value: T) -> Self {
        Self {
            value,
            _pad: [0; CACHE_LINE - core::mem::size_of::<AtomicUsize>()],
        }
    }
}

// Safety: Ring is safe to share between threads when T is Send
unsafe impl<T: Send, const N: usize> Send for Ring<T, N> {}
unsafe impl<T: Send, const N: usize> Sync for Ring<T, N> {}

impl<T, const N: usize> Ring<T, N> {
    /// Create a new ring buffer
    ///
    /// # Panics
    /// Panics if N is not a power of 2 (required for fast modulo)
    pub const fn new() -> Self {
        assert!(N.is_power_of_two(), "Ring size must be power of 2");
        assert!(N > 1, "Ring size must be > 1");

        Self {
            head: CacheAligned::new(AtomicUsize::new(0)),
            tail: CacheAligned::new(AtomicUsize::new(0)),
            buffer: UnsafeCell::new(unsafe { MaybeUninit::uninit().assume_init() }),
        }
    }

    /// Push an item into the ring buffer
    ///
    /// Returns `true` if successful, `false` if buffer is full.
    /// This is the producer-side operation.
    #[inline]
    pub fn push(&self, item: T) -> bool {
        let head = self.head.value.load(Ordering::Relaxed);
        let next = (head + 1) & (N - 1); // Fast modulo for power of 2

        // Check if full (next would equal tail)
        if next == self.tail.value.load(Ordering::Acquire) {
            return false;
        }

        // Write item
        unsafe {
            let buffer = &mut *self.buffer.get();
            buffer[head].write(item);
        }

        // Publish new head
        self.head.value.store(next, Ordering::Release);
        true
    }

    /// Pop an item from the ring buffer
    ///
    /// Returns `Some(item)` if available, `None` if buffer is empty.
    /// This is the consumer-side operation.
    #[inline]
    pub fn pop(&self) -> Option<T> {
        let tail = self.tail.value.load(Ordering::Relaxed);

        // Check if empty (tail equals head)
        if tail == self.head.value.load(Ordering::Acquire) {
            return None;
        }

        // Read item
        let item = unsafe {
            let buffer = &*self.buffer.get();
            buffer[tail].assume_init_read()
        };

        // Publish new tail
        let next = (tail + 1) & (N - 1);
        self.tail.value.store(next, Ordering::Release);

        Some(item)
    }

    /// Check if the ring buffer is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.tail.value.load(Ordering::Acquire) == self.head.value.load(Ordering::Acquire)
    }

    /// Check if the ring buffer is full
    #[inline]
    pub fn is_full(&self) -> bool {
        let head = self.head.value.load(Ordering::Acquire);
        let next = (head + 1) & (N - 1);
        next == self.tail.value.load(Ordering::Acquire)
    }

    /// Get the number of items in the buffer
    #[inline]
    pub fn len(&self) -> usize {
        let head = self.head.value.load(Ordering::Acquire);
        let tail = self.tail.value.load(Ordering::Acquire);

        if head >= tail {
            head - tail
        } else {
            N - tail + head
        }
    }

    /// Get the capacity of the buffer
    #[inline]
    pub const fn capacity(&self) -> usize {
        N - 1 // One slot always empty to distinguish full from empty
    }

    /// Get the current fill ratio (0.0 to 1.0)
    #[inline]
    pub fn pressure(&self) -> f32 {
        self.len() as f32 / self.capacity() as f32
    }

    /// Clear all items from the buffer
    ///
    /// # Safety
    /// Only call from the consumer side
    pub fn clear(&self) {
        while self.pop().is_some() {}
    }
}

impl<T, const N: usize> Default for Ring<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop() {
        let ring: Ring<u64, 16> = Ring::new();

        assert!(ring.is_empty());
        assert!(ring.push(42));
        assert!(!ring.is_empty());
        assert_eq!(ring.pop(), Some(42));
        assert!(ring.is_empty());
    }

    #[test]
    fn test_full() {
        let ring: Ring<u64, 4> = Ring::new();

        assert!(ring.push(1));
        assert!(ring.push(2));
        assert!(ring.push(3));
        assert!(ring.is_full());
        assert!(!ring.push(4)); // Should fail

        assert_eq!(ring.pop(), Some(1));
        assert!(ring.push(4)); // Now succeeds
    }

    #[test]
    fn test_fifo_order() {
        let ring: Ring<u64, 8> = Ring::new();

        for i in 0..5 {
            ring.push(i);
        }

        for i in 0..5 {
            assert_eq!(ring.pop(), Some(i));
        }
    }

    #[test]
    fn test_pressure() {
        let ring: Ring<u64, 8> = Ring::new();

        assert_eq!(ring.pressure(), 0.0);
        ring.push(1);
        ring.push(2);
        ring.push(3);
        // 3 items in capacity of 7 = ~0.43
        assert!(ring.pressure() > 0.4 && ring.pressure() < 0.5);
    }
}
