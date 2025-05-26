use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput::Elements};
use ringbuf::{storage::Array, traits::*, LocalRb};

const BUFFER_CAP: usize = 100;

fn benchmark_read_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("read_throughput_single_threaded");
    group.throughput(Elements(BUFFER_CAP as u64));
    group.bench_function("read_throughput_single_threaded", |b| {
        b.iter_batched(
            || {
                let rb = LocalRb::<Array<usize, BUFFER_CAP>>::default();
                let (mut prod, cons) = rb.split();
                for i in 0..BUFFER_CAP {
                    prod.try_push(i).unwrap();
                }
                cons
            },
            |mut cons| {
                for _ in 0..BUFFER_CAP {
                    black_box(cons.try_pop().unwrap_or_default());
                }
            },
            criterion::BatchSize::PerIteration,
        );
    });
}

fn benchmark_write_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("write_throughput_single_threaded");
    group.throughput(Elements(BUFFER_CAP as u64));
    group.bench_function("write_throughput_single_threaded", |b| {
        b.iter_batched(
            || {
                let rb = LocalRb::<Array<usize, BUFFER_CAP>>::default();
                let (prod, _cons) = rb.split();
                prod
            },
            |mut prod| {
                for _ in 0..BUFFER_CAP {
                    prod.try_push(black_box(1)).unwrap();
                }
            },
            criterion::BatchSize::PerIteration,
        );
    });
}

fn benchmark_write_read_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("write_read_throughput_single_threaded");
    group.throughput(Elements(BUFFER_CAP as u64));
    group.bench_function("write_read_throughput_single_threaded", |b| {
        b.iter(|| {
            let rb = LocalRb::<Array<usize, BUFFER_CAP>>::default();
            let (mut prod, mut cons) = rb.split();
            for i in 0..BUFFER_CAP {
                prod.try_push(i).unwrap();
            }
            for _ in 0..BUFFER_CAP {
                black_box(cons.try_pop().unwrap_or_default());
            }
        });
    });
}
criterion_group!(
    benches,
    benchmark_read_throughput,
    benchmark_write_throughput,
    benchmark_write_read_throughput
);
criterion_main!(benches);
