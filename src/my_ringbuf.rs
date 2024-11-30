use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering::{Release, Acquire}};
//use crossbeam_utils::CachePadded;

pub struct Producer<T> {
    rb: Arc<UnsafeCell<RingBuffer<T>>>,
    local_produce_index: usize,
    local_consume_index: usize,
}
pub struct Consumer<T> {
    rb: Arc<UnsafeCell<RingBuffer<T>>>,
    local_produce_index: usize,
    local_consume_index: usize,
}

unsafe impl<T> Send for Producer<T> {}
unsafe impl<T> Send for Consumer<T> {}

struct RingBuffer<T> {
    //produce_index: CachePadded<AtomicUsize>,
    //consume_index: CachePadded<AtomicUsize>,
    produce_index: AtomicUsize,
    consume_index: AtomicUsize,
    buffer: Box<[MaybeUninit<T>]>,
}

pub fn my_ringbuf<T>(capacity: usize) -> (Producer<T>, Consumer<T>) {
    let rb = Arc::new(UnsafeCell::new(RingBuffer {
        produce_index: AtomicUsize::new(0),
        consume_index: AtomicUsize::new(0),
        //produce_index: CachePadded::new(AtomicUsize::new(0)),
        //consume_index: CachePadded::new(AtomicUsize::new(0)),
        buffer: Box::new_uninit_slice(capacity),
    }));
    (Producer {
        rb: rb.clone(),
        local_produce_index: 0,
        local_consume_index: 0,
    },
    Consumer {
        rb,
        local_produce_index: 0,
        local_consume_index: 0,
    })
}

impl<T> Producer<T> {
    pub fn try_push(&mut self, item: T) -> Option<T> {
        let rb = unsafe { &mut *self.rb.get() };
        let cap = rb.buffer.len();

        let p = self.local_produce_index;
        let c = self.local_consume_index;
        if p + 1 == c || (p == cap-1 && c == 0) { // full, fetch real consume_index
            self.local_consume_index = rb.consume_index.load(Acquire);
            let c = self.local_consume_index;
            if p + 1 == c || (p == cap-1 && c == 0) { // check again
                return Some(item);
            }
        }

        let next = next_index(p, cap);
        rb.buffer[p].write(item);
        rb.produce_index.store(next, Release);
        self.local_produce_index = next; // update local_produce_index too
        None
    }
}

impl<T> Consumer<T> {
    pub fn try_pop(&mut self) -> Option<T> {
        let rb = unsafe { &mut *self.rb.get() };

        let p = self.local_produce_index;
        let c = self.local_consume_index;
        if p == c { // empty, fetch real produce_index
            self.local_produce_index = rb.produce_index.load(Acquire);
            let c = self.local_produce_index;
            if p == c {
                return None; // check again
            }
        }

        let next = next_index(c, rb.buffer.len());
        let item = unsafe { rb.buffer.get_unchecked(c).assume_init_read() };
        rb.consume_index.store(next, Release);
        self.local_consume_index = next; // update local_consume_index too

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
