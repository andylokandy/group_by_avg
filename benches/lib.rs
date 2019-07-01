use criterion::{black_box, criterion_group, criterion_main, Criterion};
use group_by_avg::group_by_avg;
use rand::{thread_rng, Rng};

fn bench_normal(c: &mut Criterion) {
    let mut rng = thread_rng();
    let input: Vec<_> = (0..10000)
        .map(|_| (rng.gen_range(-100, 100), rng.gen_range(-1000, 1000)))
        .collect();
    c.bench_function("normal 10000", move |b| {
        b.iter(|| group_by_avg(black_box(&input)))
    });
}

fn bench_dense_key(c: &mut Criterion) {
    let mut rng = thread_rng();
    let input: Vec<_> = (0..10000)
        .map(|_| (rng.gen_range(-1, 1), rng.gen_range(-1000, 1000)))
        .collect();
    c.bench_function("dense key 10000", move |b| {
        b.iter(|| group_by_avg(black_box(&input)))
    });
}

fn bench_sparse_key(c: &mut Criterion) {
    let mut rng = thread_rng();
    let input: Vec<_> = (0..10000)
        .map(|_| (rng.gen_range(-10000, 10000), rng.gen_range(-1000, 1000)))
        .collect();
    c.bench_function("sparse key 10000", move |b| {
        b.iter(|| group_by_avg(black_box(&input)))
    });
}

criterion_group!(benches, bench_normal, bench_dense_key, bench_sparse_key);
criterion_main!(benches);
