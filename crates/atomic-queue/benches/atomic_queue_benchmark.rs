use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("atomic_queue - push/pop", |b| {
        let queue = atomic_queue::Queue::<u64>::new(100);
        b.iter(|| {
            queue.push(black_box(0));
            queue.pop()
        })
    });

    c.bench_function("ringbuf - push/pop", |b| {
        let queue = ringbuf::RingBuffer::<u64>::new(100);
        let (mut prod, mut cons) = queue.split();
        b.iter(|| {
            prod.push(black_box(0)).unwrap();
            cons.pop().unwrap()
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
