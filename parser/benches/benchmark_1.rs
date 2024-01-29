use crate::chart::chart;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../../charts/Adagio - Second Sight [Peddy]/notes.chart");
    c.bench_function("chart Adagio - Second Sight [Peddy]", |b| {
        b.iter(|| chart(black_box(input)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
