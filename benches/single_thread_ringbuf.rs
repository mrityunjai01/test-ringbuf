use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ringbuf::{storage::Array, traits::*, LocalRb};

const BUFFER_CAP: usize = 1000;

fn benchmark_single_read_empty(c: &mut Criterion) {
    c.iter(|| {
        let rb = LocalRb::<Array<usize, BUFFER_CAP>>::default();
        let (mut prod, cons) = rb.split();
        prod.try_push(1).unwrap();
        black_box(cons.try_pop().unwrap_or_default());
    });
}

criterion_group!(benches, benchmark_single_read_empty);
criterion_main!(benches);
