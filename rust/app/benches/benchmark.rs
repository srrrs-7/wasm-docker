use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("hello", |b| b.iter(|| println!("hello")));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);