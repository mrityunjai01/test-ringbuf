use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering::{Release, Relaxed}};

pub struct Producer<T>(Arc<UnsafeCell<RingBuffer<T>>>);
pub struct Consumer<T>(Arc<UnsafeCell<RingBuffer<T>>>);

unsafe impl<T> Send for Producer<T> {}
unsafe impl<T> Send for Consumer<T> {}

struct RingBuffer<T> {
    produce_index: AtomicUsize,
    consume_index: AtomicUsize,
    buffer: Box<[MaybeUninit<T>]>,
}

pub fn my_ringbuf<T>(capacity: usize) -> (Producer<T>, Consumer<T>) {
    let rb = Arc::new(UnsafeCell::new(RingBuffer {
        produce_index: AtomicUsize::new(0),
        consume_index: AtomicUsize::new(0),
        buffer: Box::new_uninit_slice(capacity),
    }));
    (Producer(rb.clone()), Consumer(rb))
}

impl<T> Producer<T> {
    pub fn try_push(&mut self, item: T) -> Option<T> {
        let current = unsafe { &mut *self.0.get() };

        let c = current.consume_index.load(Relaxed); // Relaxed should be OK
        let p = current.produce_index.load(Relaxed);
        let cap = current.buffer.len();
        if p + 1 == c || (p == cap-1 && c == 0) { // full
            return Some(item);
        }

        let next = next_index(p, cap);
        current.buffer[p].write(item);
        current.produce_index.store(next, Release);
        None
    }
}

impl<T> Consumer<T> {
    pub fn try_pop(&mut self) -> Option<T> {
        let current = unsafe { &mut *self.0.get() };

        let c = current.consume_index.load(Relaxed);
        let p = current.produce_index.load(Relaxed);
        if p == c { // empty
            return None;
        }

        let next = next_index(c, current.buffer.len());
        let item = unsafe { current.buffer.get_unchecked(c).assume_init_read() };
        current.consume_index.store(next, Release);

        Some(item)
    }
}

fn next_index(i: usize, capacity: usize) -> usize {
    if i == capacity - 1 {
        0
    } else {
        i + 1
    }
}
