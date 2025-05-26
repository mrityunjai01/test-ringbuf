use criterion::{black_box, criterion_group, criterion_main, BatchSize::SmallInput, Criterion};
use ringbuf::{storage::Array, traits::*, LocalRb};

const BUFFER_CAP: usize = 1000;

fn benchmark_single_read(c: &mut Criterion) {
    c.bench_function("single_read_single_thread_empty_buffer", |b| {
        b.iter_batched_ref(
            || {
                let rb = LocalRb::<Array<usize, BUFFER_CAP>>::default();
                let (mut prod, cons) = rb.split();
                prod.try_push(1).unwrap();
                cons
            },
            |cons| {
                black_box(cons.try_pop().unwrap_or_default());
            },
            SmallInput,
        );
    });
    c.bench_function("single_read_single_thread_full_buffer", |b| {
        b.iter_batched_ref(
            || {
                let rb = LocalRb::<Array<usize, BUFFER_CAP>>::default();
                let (mut prod, cons) = rb.split();
                for i in 0..BUFFER_CAP {
                    prod.try_push(i).unwrap();
                }
                cons
            },
            |cons| {
                black_box(cons.try_pop().unwrap_or_default());
            },
            SmallInput,
        );
    });
    c.bench_function("single_read_single_thread_half_buffer", |b| {
        b.iter_batched_ref(
            || {
                let rb = LocalRb::<Array<usize, BUFFER_CAP>>::default();
                let (mut prod, cons) = rb.split();
                for i in 0..BUFFER_CAP / 2 {
                    prod.try_push(i).unwrap();
                }
                cons
            },
            |cons| {
                black_box(cons.try_pop().unwrap_or_default());
            },
            SmallInput,
        );
    });
}

fn benchmark_single_write(c: &mut Criterion) {
    c.bench_function("single_write_single_thread_empty_buffer", |b| {
        b.iter_batched_ref(
            || {
                let rb = LocalRb::<Array<usize, BUFFER_CAP>>::default();
                let (prod, _cons) = rb.split();
                prod
            },
            |prod| {
                prod.try_push(black_box(1)).unwrap();
            },
            SmallInput,
        );
    });
    c.bench_function("single_write_single_thread_full_buffer", |b| {
        b.iter_batched_ref(
            || {
                let rb = LocalRb::<Array<usize, BUFFER_CAP>>::default();
                let (mut prod, _cons) = rb.split();
                for i in 0..BUFFER_CAP {
                    prod.try_push(i).unwrap();
                }
                prod
            },
            |prod| {
                prod.try_push(black_box(1)).unwrap();
            },
            SmallInput,
        );
    });
    c.bench_function("single_write_single_thread_half_buffer", |b| {
        b.iter_batched_ref(
            || {
                let rb = LocalRb::<Array<usize, BUFFER_CAP>>::default();
                let (mut prod, _cons) = rb.split();
                for i in 0..BUFFER_CAP / 2 {
                    prod.try_push(i).unwrap();
                }
                prod
            },
            |prod| {
                prod.try_push(black_box(1)).unwrap();
            },
            SmallInput,
        );
    });
}

criterion_group!(benches, benchmark_single_read, benchmark_single_write);
criterion_main!(benches);
