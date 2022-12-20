use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion
};

use y_2022;

pub fn calories_benchmark(c: &mut Criterion) {
    c.bench_function("vector v binary heap", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, calories_benchmark);
criterion_main!(benches);
